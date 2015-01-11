///! The optimizer module is inspired by the article (brainfuck optimization
///! strategies)[http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html]
///! written by Mats Linander. It implements many of the optimization
///! strategies discussed in the article.

use std::default::Default;

use syntax::{Ast, Ir};

/// The Optimization Configuration or `OptConfig` struct holds settings for which
/// optimization techniques to use.
#[derive(Clone, Copy, Show)]
pub struct OptConfig {
    contract_opt: bool,
    clear_loop_opt: bool,
    scan_loop_opt: bool,
    mul_copy_loop_opt: bool,
}

impl OptConfig {
    #[inline]
    pub fn new(use_contract_opt: bool,
               use_clear_loop_opt: bool,
               use_scan_loop_opt: bool,
               use_mul_copy_loop_opt: bool) -> OptConfig {
        OptConfig {
            contract_opt: use_contract_opt,
            clear_loop_opt: use_clear_loop_opt,
            scan_loop_opt: use_scan_loop_opt,
            mul_copy_loop_opt: use_mul_copy_loop_opt,
        }
    }
}

impl Default for OptConfig {
    fn default() -> OptConfig {
        OptConfig::new(true,    // contract opt
                       true,    // clear loop opt
                       true,    // scan loop opt
                       true)    // mul/copy loop opt
    }
}

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
        for i in range(1, ast.len()) {
            let prev = opt_ast.pop().unwrap();
            match (prev, ast[i]) {
                (Ir::Add(prev_value), Ir::Add(value)) => {
                    opt_ast.push(Ir::Add(prev_value + value));
                },
                (Ir::Sub(prev_value), Ir::Sub(value)) => {
                    opt_ast.push(Ir::Sub(prev_value + value));
                },
                (Ir::MoveLeft(prev_steps), Ir::MoveLeft(steps)) => {
                    opt_ast.push(Ir::MoveLeft(prev_steps + steps));
                },
                (Ir::MoveRight(prev_steps), Ir::MoveRight(steps)) => {
                    opt_ast.push(Ir::MoveRight(prev_steps + steps));
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
        for i in range(1, prev_opt_ast.len()) {
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
                (Ir::MoveLeft(prev_steps), Ir::MoveRight(steps)) => {
                    if prev_steps > steps {
                        opt_ast.push(Ir::MoveLeft(prev_steps - steps));
                    } else if prev_steps < steps {
                        opt_ast.push(Ir::MoveRight(steps - prev_steps));
                    } else {} // they cancel out
                },
                (Ir::MoveRight(prev_steps), Ir::MoveLeft(steps)) => {
                    if prev_steps > steps {
                        opt_ast.push(Ir::MoveRight(prev_steps - steps));
                    } else if prev_steps < steps {
                        opt_ast.push(Ir::MoveLeft(steps - prev_steps));
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

        for i in range(2, ast.len()) {
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

        for i in range(2, ast.len()) {
            let prev = (ast[i - 2], ast[i - 1], ast[i]);
            match prev {
                (Ir::Open, Ir::MoveLeft(1), Ir::Close)  => {
                    opt_ast.pop();
                    opt_ast.pop();
                    opt_ast.push(Ir::ScanLeft);
                },
                (Ir::Open, Ir::MoveRight(1), Ir::Close) => {
                    opt_ast.pop();
                    opt_ast.pop();
                    opt_ast.push(Ir::ScanRight);
                },
                _ => { opt_ast.push(ast[i]); },
            }
        }

        opt_ast
    } else {
        ast.clone()
    }
}

/// Optimizes 'multiplication loops' and subsequently 'copy loops', which are
/// a special form of multiplication loops into a set of simplified instructions.
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
fn mul_copy_loop_opt(ast: &Ast) -> Ast {
    // TODO
    ast.clone()
}

/// Optimizes an `Ast` using the `OptConfig` to customize which optimizations
/// to execute.
pub fn optimize(opt_config: &OptConfig, ast: &Ast) -> Ast {
    let mut opt_ast = ast.clone();

    // always optimize out comment loops and unused loops
    opt_ast = comment_loop_opt(&opt_ast);
    opt_ast = unused_loop_opt(&opt_ast);

    // optimize according to the opt_config
    if opt_config.contract_opt {
        opt_ast = contract_opt(&opt_ast);
    }
    if opt_config.clear_loop_opt {
        opt_ast = clear_loop_opt(&opt_ast);
    }
    if opt_config.scan_loop_opt {
        opt_ast = scan_loop_opt(&opt_ast);
    }
    if opt_config.mul_copy_loop_opt {
        opt_ast = mul_copy_loop_opt(&opt_ast);
    }

    opt_ast
}
