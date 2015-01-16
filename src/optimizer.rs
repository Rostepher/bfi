///! The optimizer module is inspired by the article (brainfuck optimization
///! strategies)[http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html]
///! written by Mats Linander. It implements many of the optimization
///! strategies discussed in the article.

use std::collections::HashMap;
use std::num::SignedInt;

use syntax::{Ast, Ir, Right, Left};

/// Removes comment loop(s), which exist at the very beginning of the `Ast` and
/// would never execute as the current cell would be 0.
fn comment_loop_opt(ast: &Ast) -> Ast {
    // optimized abstract syntax tree
    let mut opt_ast = ast.clone();

    while !opt_ast.is_empty() {
        match opt_ast[0] {
            Ir::Open => {
                // remove Ir::Open
                opt_ast.remove(0);

                // remove loop with the assumption that there is a matching
                // Ir::Close, hence no check that opt_ast is not empty
                let mut unmatched = 1u32;
                while unmatched > 0 {
                    match opt_ast[0] {
                        Ir::Open  => unmatched += 1,
                        Ir::Close => unmatched -= 1,
                        _ => {}, // skip all other ir
                    }
                    opt_ast.remove(0);
                }
            }
            _ => break, // end of comment loops
        }
    }

    opt_ast
}

/// Removes unused loops from an `Ast`. Two types of unused loops are removed,
/// comment loops and loops that start immediately after another loop closed,
/// which could never execute as, the current cell would be 0.
fn unused_loop_opt(ast: &Ast) -> Ast {
    if ast.len() > 1 {
        // optimized abstract syntax tree
        let mut opt_ast = Vec::new();

        let mut prev = ast[0];
        opt_ast.push(ast[0]);

        let mut i = 1us;
        while i < ast.len() {
            if prev == Ir::Close && ast[i] == Ir::Open {
                // skip loop with the assumption that there is a matching
                // Ir::Close, hence no bounds check
                let mut unmatched = 1u32;
                while unmatched > 0 {
                    i += 1;
                    match ast[i] {
                        Ir::Open  => unmatched += 1,
                        Ir::Close => unmatched -= 1,
                        _         => {}, // skip all other ir
                    }
                }
            } else {
                prev = ast[i];
                opt_ast.push(ast[i]);
            }
            i += 1;
        }

        opt_ast
    } else {
        ast.clone()
    }
}

/// Optimizes (contracts) sequential uses of `Ir::Add`, `Ir::Sub`, `Ir::Left`
/// and `Ir::Right` into single instructions. If the contracted instructions
/// would balance out, the operations are removed altogether.
///
/// # Example
///
/// ```brainfuck
/// >>>+++<<<---
/// ```
///
/// would be optimized to
///
/// ```
/// MoveRight(3), Add(3), Sub(3), MoveLeft(3)
/// ```
fn contract_opt(ast: &Ast) -> Ast {
    if ast.len() > 1 {
        // optimized abstract syntax tree
        let mut opt_ast = Vec::new();
        opt_ast.push(ast[0]);

        // combine ir of the same type
        for i in 1..ast.len() {
            let prev = opt_ast.pop().unwrap();
            match (prev, ast[i]) {
                (Ir::Add(prev_value), Ir::Add(value)) => {
                    opt_ast.push(Ir::Add(prev_value + value));
                },
                (Ir::Sub(prev_value), Ir::Sub(value)) => {
                    opt_ast.push(Ir::Sub(prev_value + value));
                },
                (Ir::Move(Left, prev_steps), Ir::Move(Left, steps)) => {
                    opt_ast.push(Ir::Move(Left, prev_steps + steps));
                },
                (Ir::Move(Right, prev_steps), Ir::Move(Right, steps)) => {
                    opt_ast.push(Ir::Move(Right, prev_steps + steps));
                },
                _ => {
                    opt_ast.push(prev);
                    opt_ast.push(ast[i]);
                }, // not a match
            }
        }

        let prev_opt_ast = opt_ast;
        opt_ast = Vec::new();
        opt_ast.push(prev_opt_ast[0]);

        // combine ir of opposite types, i.e. Ir::Add and Ir::Sub or
        // Ir::MoveLeft and Ir::MoveRight, if they appear directly after each
        // other.
        for i in 1..prev_opt_ast.len() {
            let prev = opt_ast.pop().unwrap();
            match (prev, prev_opt_ast[i]) {
                (Ir::Add(prev_value), Ir::Sub(value)) => {
                    if prev_value > value {
                        opt_ast.push(Ir::Add(prev_value - value));
                    } else if prev_value < value {
                        opt_ast.push(Ir::Sub(value - prev_value));
                    } else {} // they cancel out
                },
                (Ir::Sub(prev_value), Ir::Add(value)) => {
                    if prev_value > value {
                        opt_ast.push(Ir::Sub(prev_value - value));
                    } else if prev_value < value {
                        opt_ast.push(Ir::Add(value - prev_value));
                    } else {} // they cancel out
                },
                (Ir::Move(Left, prev_steps), Ir::Move(Right, steps)) => {
                    if prev_steps > steps {
                        opt_ast.push(Ir::Move(Left, prev_steps - steps));
                    } else if prev_steps < steps {
                        opt_ast.push(Ir::Move(Right, steps - prev_steps));
                    } else {} // they cancel out
                },
                (Ir::Move(Right, prev_steps), Ir::Move(Left, steps)) => {
                    if prev_steps > steps {
                        opt_ast.push(Ir::Move(Right, prev_steps - steps));
                    } else if prev_steps < steps {
                        opt_ast.push(Ir::Move(Left, steps - prev_steps));
                    } else {} // they cancel out
                },
                _ => {
                    opt_ast.push(prev);
                    opt_ast.push(prev_opt_ast[i]);
                }, // not opposites
            }
        }

        opt_ast
    } else {
        ast.clone()
    }
}

/// Optimizes 'clear loops', which have the form `[-]` or `[+]` into a single
/// `Ir::Clear` instruction.
///
/// # Example
///
/// ```brainfuck
/// [-]
/// ```
///
/// would be optimized to
///
/// ```
/// Clear(0)
/// ```
fn clear_loop_opt(ast: &Ast) -> Ast {
    if ast.len() > 2 {
        // optimized abstract syntax tree
        let mut opt_ast = Vec::new();

        opt_ast.push(ast[0]);
        opt_ast.push(ast[1]);

        for i in 2..ast.len() {
            let prev = (ast[i - 2], ast[i - 1], ast[i]);
            match prev {
                (Ir::Open, Ir::Add(1), Ir::Close) |
                (Ir::Open, Ir::Sub(1), Ir::Close) => {
                    opt_ast.pop();
                    opt_ast.pop();
                    opt_ast.push(Ir::Clear);
                },
                _ => { opt_ast.push(ast[i]); },
            }
        }

        opt_ast
    } else {
        ast.clone()
    }
}

/// Optimizes 'scan loops', which have the form `[<]` or `[>]` into a single
/// `Ir::ScanLeft` or `Ir::ScanRight` instruction.
///
/// # Example
///
/// ```brainfuck
/// [<]
/// ```
///
/// would be optimized to
///
/// ```
/// ScanLeft
/// ```
fn scan_loop_opt(ast: &Ast) -> Ast {
    if ast.len() > 2 {
        // optimized abstract syntax tree
        let mut opt_ast = Vec::new();

        opt_ast.push(ast[0]);
        opt_ast.push(ast[1]);

        for i in 2..ast.len() {
            let prev = (ast[i - 2], ast[i - 1], ast[i]);
            match prev {
                (Ir::Open, Ir::Move(dir, 1), Ir::Close)  => {
                    opt_ast.pop();
                    opt_ast.pop();
                    opt_ast.push(Ir::Scan(dir));
                },
                _ => { opt_ast.push(ast[i]); },
            }
        }

        opt_ast
    } else {
        ast.clone()
    }
}

/// Optimizes 'multiplication loops', 'division loops' and subsequently
/// 'copy loops', which are a special form of multiplication loops into a set
/// of simplified instructions.
///
/// # Example
///
/// ```brainfuck
/// [->>+++++>++<<<]
/// ```
///
/// would be optimized to
///
/// ```
/// Mul(2, 5), Mul(3, 2), Clear
/// ```
///
/// while
///
/// ```brainfuck
/// [->+>+]
/// ```
///
/// would be optimized to
///
/// ```
/// Copy(1), Copy(2), Clear
/// ```
fn copy_mul_div_loop_opt(ast: &Ast) -> Ast {
    let mut opt_ast = Vec::new();

    // index of the current loop's Ir::Open
    let mut i = 0;

    'outer: loop {
        // find the index for the Ir::Open and Ir::Close of the next loop that
        // does not contain any other loops
        while i < ast.len() {
            match ast[i] {
                Ir::Open => break,

                // push current ir onto the opt_ast then keep looking for an
                // Ir::Open
                _ => {
                    opt_ast.push(ast[i]);
                    i += 1;
                },
            }
        }
        // break if the end of the ast has been reached
        if i >= ast.len() {
            break;
        }

        // index of Ir::Close
        let mut k = i + 1;

        // find the close of the loop
        while k < ast.len() {
            match ast[k] {
                // if another loop is found, push all previous ir onto opt_ast
                // and then move on
                Ir::Open  => {
                    for ir in ast[i..k + 1].iter() {
                        opt_ast.push(*ir);
                    }
                    i = k + 1;
                    continue 'outer;
                },
                Ir::Close => break,  // found the Ir::Close
                _         => k += 1, // keep searching for the Ir::Close
            }
        }

        // break if the end of the ast has been reached
        if k >= ast.len() {
            break;
        }

        for ir in ast[i..k + 1].iter() { print!("{:?}, ", *ir); }
        println!("");

        // verify that the loop only contains Ir::Add, Ir::Sub, Ir::MoveLeft
        // or Ir::MoveRight
        for ir in ast[i + 1..k].iter() {
            match *ir {
                // approved ir
                Ir::Add(_)     |
                Ir::Sub(_)     |
                Ir::Move(_, _) => {}, // ignore all correct ir

                // if any other ir appears, push all previous ir onto opt_ast
                // and then move on
                _ => {
                    println!("loop contained {:?}", *ir);
                    for ir in ast[i..k + 1].iter() {
                        opt_ast.push(*ir);
                    }
                    i = k + 1;
                    continue 'outer;
                }
            }
        }

        // track the pointer position in the loop and the value of the
        // affected cells
        let mut mem: HashMap<isize, i8> = HashMap::new();
        let mut p = 0is;
        mem.insert(p, 0i8);

        let mut old_ir = Vec::new();

        for ir in ast[i + 1..k].iter() {
            old_ir.push(*ir);
            match *ir {
                Ir::Add(value) => {
                    let new_value = match mem.get(&p) {
                        Some(curr) => *curr + (value as i8),
                        None       => value as i8,
                    };
                    mem.insert(p, new_value);
                },
                Ir::Sub(value) => {
                    let new_value = match mem.get(&p) {
                        Some(curr) => *curr - (value as i8),
                        None       => -(value as i8),
                    };
                    mem.insert(p, new_value);
                },
                Ir::Move(Left, steps)  => p -= steps as isize,
                Ir::Move(Right, steps) => p += steps as isize,
                _                      => panic!("error: unexpected {:?}!", *ir),
            }
        }

        // if the pointer ends in cell 0 and the loop subtracted exactly 1 from
        // cell 0, then the loop can be optimized into an Ir::Copy, Ir::Mul or
        // Ir::Div, otherwise, copy all ir from the loop into opt_ast and
        // continue looking for an appropriate loop
        if p != 0 || mem[0] != -1 {
            for ir in ast[i..k + 1].iter() {
                opt_ast.push(*ir);
            }
            i = k + 1;
            continue 'outer;
        }

        // remove cell 0 from mem
        mem.remove(&(0));

        let mut new_ir = Vec::new();

        // replace the loop with Ir::Copy, Ir::Mul or Ir::Div where appropriate
        for (steps, factor) in mem.iter() {
            println!("factor = {}", *factor);

            // cast values safely to u8
            let usize_steps = SignedInt::abs(*steps) as usize;
            let u8_factor = SignedInt::abs(*factor) as u8;

            // calculate the direction from ptr to move
            let dir = if *steps < 0 {
                Left
            } else if *steps > 0 {
                Right
            } else {
                continue; // copys neither left or right
            };

            // if the factor at mem[p] is 1, then it is only copying
            if *factor == 1 {
                new_ir.push(Ir::Copy(dir, usize_steps));
                opt_ast.push(Ir::Copy(dir, usize_steps));
            } else if *factor > 1 {
                new_ir.push(Ir::Mul(dir, usize_steps, u8_factor));
                opt_ast.push(Ir::Mul(dir, usize_steps, u8_factor));
            } else if *factor < 0 {
                new_ir.push(Ir::Div(dir, usize_steps, u8_factor));
                opt_ast.push(Ir::Div(dir, usize_steps, u8_factor));
            } else {
                panic!("error: factor of 0 found!");
            }
        }

        // insert the clear ir
        new_ir.push(Ir::Clear);
        opt_ast.push(Ir::Clear);

        println!("old_ir = {:?}", old_ir);
        println!("new_ir = {:?}", new_ir);
        println!("");
        // println!("ast = {:?}", ast);
        // println!("opt_ast = {:?}", opt_ast);
        // println!("");

        // move i to the ir after k
        i = k + 1;
    }

    opt_ast
}

/// Optimization level selected by the user in the command line.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Show)]
pub enum OptLevel {
    No,         // -O0
    Less,       // -O1
    Default,    // -O2
    Aggressive, // -O3
}

/// Optimizes an `Ast` using the `OptLevel` to customize which optimizations
/// to execute.
pub fn optimize(opt_level: OptLevel, ast: &Ast) -> Ast {
    let mut opt_ast = ast.clone();

    if opt_level >= OptLevel::Less {
        opt_ast = comment_loop_opt(&opt_ast);
        opt_ast = unused_loop_opt(&opt_ast);
    }

    if opt_level >= OptLevel::Default {
        opt_ast = contract_opt(&opt_ast);
        opt_ast = clear_loop_opt(&opt_ast);
        opt_ast = scan_loop_opt(&opt_ast);
    }

    if opt_level == OptLevel::Aggressive {
        opt_ast = copy_mul_div_loop_opt(&opt_ast);
    }

    opt_ast
}
