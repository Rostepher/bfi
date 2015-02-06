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
/// ShiftRight(3), Add(3), Sub(3), ShiftLeft(3)
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
                (Ir::Shift(Left, prev_steps), Ir::Shift(Left, steps)) => {
                    opt_ast.push(Ir::Shift(Left, prev_steps + steps));
                },
                (Ir::Shift(Right, prev_steps), Ir::Shift(Right, steps)) => {
                    opt_ast.push(Ir::Shift(Right, prev_steps + steps));
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
        // Ir::ShiftLeft and Ir::ShiftRight, if they appear directly after each
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
                (Ir::Shift(Left, prev_steps), Ir::Shift(Right, steps)) => {
                    if prev_steps > steps {
                        opt_ast.push(Ir::Shift(Left, prev_steps - steps));
                    } else if prev_steps < steps {
                        opt_ast.push(Ir::Shift(Right, steps - prev_steps));
                    } else {} // they cancel out
                },
                (Ir::Shift(Right, prev_steps), Ir::Shift(Left, steps)) => {
                    if prev_steps > steps {
                        opt_ast.push(Ir::Shift(Right, prev_steps - steps));
                    } else if prev_steps < steps {
                        opt_ast.push(Ir::Shift(Left, steps - prev_steps));
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
                (Ir::Open, Ir::Shift(dir, 1), Ir::Close)  => {
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

/// Helper function for `copy_mul_div_loop_opt` that finds the next unnested
/// loop, starting from `start_index` and returns a tuple of
/// `Some(open_index, close_index)` or `None` if no such loop exists.
fn next_unnested_loop(ast: &Ast, start_index: usize) -> Option<(usize, usize)> {
    let mut open_index = start_index;
    'outer: loop {
        while open_index < ast.len() {
            match ast[open_index] {
                Ir::Open => break,
                _        => open_index += 1,
            }
        }

        // bounds check open_index
        if open_index >= ast.len() {
            return None;
        }

        let mut close_index = open_index + 1;
        while close_index < ast.len() {
            match ast[close_index] {
                Ir::Open => {
                    open_index = close_index;
                    continue 'outer;
                },
                Ir::Close => break, // found the close
                _         => close_index += 1,
            }
        }

        // bounds check close_index
        if close_index >= ast.len() {
            return None;
        }

        return Some((open_index, close_index));
    }
}

/// Helper function for `mul_copy_loop_opt` that analyzes `loop_ast` and
/// returns the optimized `Ast`. This function assumes that `loop_ast` only
/// contains Ir::Add, Ir::Sub, Ir::Shift, Ir::Open or Ir::Close.
fn replace_mul_copy_loop(loop_ast: &Ast) -> Option<Ast> {
    let mut opt_loop_ast = Vec::new();

    // track the pointer position in the loop and the value of the
    // affected cells
    let mut mem: HashMap<isize, i8> = HashMap::new();
    let mut p = 0is;
    mem.insert(p, 0i8);

    for ir in loop_ast.iter() {
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
            Ir::Shift(Left, steps)  => p -= steps as isize,
            Ir::Shift(Right, steps) => p += steps as isize,
            Ir::Open | Ir::Close   => {}, // skip loop open and close
            _                      => panic!("error: unexpected {:?}!", *ir),
        }
    }

    // if the pointer ends in cell 0 and the loop subtracted exactly 1 from
    // cell 0, then the loop can be optimized into an Ir::Copy, Ir::Mul or
    // Ir::Div, otherwise, return None
    if p != 0 || mem[0] != -1 {
        return None;
    }

    // remove cell 0 from mem
    mem.remove(&(0));

    // replace the loop with Ir::Copy, Ir::Mul or Ir::Div where appropriate
    for (steps, factor) in mem.iter() {
        // cast steps safely to u8
        let usize_steps = SignedInt::abs(*steps) as usize;

        // calculate the direction from ptr to move
        let dir = if *steps < 0 {
            Left
        } else if *steps > 0 {
            Right
        } else {
            continue; // copys neither left or right
        };

        // when factor is 1, it is an Ir::Copy
        if *factor == 1 {
            opt_loop_ast.push(Ir::Copy(dir, usize_steps));
        }

        // when factor is not 0 and not 1, it is an Ir::Mul
        else if *factor != 1 && *factor != 0 {
            opt_loop_ast.push(Ir::Mul(dir, usize_steps, *factor));
        }

        // factor is 0, so it does nothing
        else {
            continue;
        }
    }

    // insert the clear ir
    opt_loop_ast.push(Ir::Clear);

    Some(opt_loop_ast)
}

/// Optimizes 'multiplication loops', 'division loops' and subsequently
/// 'copy loops', which are a special form of multiplication loops into a set
/// of simplified instructions.
///
/// # Example
///
/// ```brainfuck
/// [->+>++>--<<<]
/// ```
///
/// would be optimized to
///
/// ```
/// Copy(1), Mul(2, 2), Div(3, 2), Clear
/// ```
fn copy_mul_div_loop_opt(ast: &Ast) -> Ast {
    let mut opt_ast = Vec::new();

    let mut start = 0us;
    'outer: loop {
        // next unnested loop
        let (open, close) = match next_unnested_loop(ast, start) {
            // found an unnested loop
            Some(tuple) => tuple,

            // no such loops remain, therefore add all ir left in ast to
            // opt_ast and break
            None => {
                for ir in ast[start..].iter() {
                    opt_ast.push(*ir);
                }
                break;
            },
        };

        // add all ir previous to the loop to opt_ast
        for ir in ast[start..open].iter() {
            opt_ast.push(*ir);
        }

        // verify that the loop only contains Ir::Add, Ir::Sub, Ir::ShiftLeft
        // or Ir::ShiftRight
        for ir in ast[open + 1..close].iter() {
            match *ir {
                // ignore all correct ir
                Ir::Add(_) | Ir::Sub(_) | Ir::Shift(_, _) => {},

                // if any other ir appears, push all previous ir onto opt_ast
                // and then move on
                _ => {
                    for ir in ast[open..close + 1].iter() {
                        opt_ast.push(*ir);
                    }
                    start = close + 1;
                    continue 'outer;
                }
            }
        }

        // collect all ir in the loop
        let loop_ast = ast[open..close + 1].iter().map(|&ir| ir).collect();

        match replace_mul_copy_loop(&loop_ast) {
            // the loop was replacable so append the new ir to opt_ast
            Some(opt_loop_ast) => {
                opt_ast.push_all(&opt_loop_ast[]);
                println!("loop_ast     = {:?}", loop_ast);
                println!("opt_loop_ast = {:?}", opt_loop_ast);
                println!("");
            },

            // the loop was not a copy, mul or div loop, therefore push all
            // ir from loop_ast
            None => {
                opt_ast.push_all(&loop_ast[]);
            },
        }

        // move open to the ir after close
        start = close + 1;
    }

    opt_ast
}

/// Optimization level selected by the user in the command line.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug)]
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
