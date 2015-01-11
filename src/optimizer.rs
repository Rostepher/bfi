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
    // TODO
    println!("comment_loop_opt() called!");
    ast.clone()
}

/// Removes unused loops from an `Ast`. Two types of unused loops are removed,
/// comment loops and loops that start immediately after another loop closed,
/// which could never execute as, the current cell would be 0.
fn unused_loop_opt(ast: &Ast) -> Ast {
    // TODO
    println!("unused_loop_opt() called!");
    ast.clone()
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
    // TODO
    println!("contract_opt() called!");
    ast.clone()
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
    // TODO
    println!("clear_loop_opt() called!");

    // optimized abstract syntax tree
    let mut opt_ast = Vec::new();

    if ast.len() > 2 {
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
    } else {
        opt_ast = ast.clone();
    }

    opt_ast
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
    // TODO
    println!("scan_loop_opt() called!");

    // optimized abstract syntax tree
    let mut opt_ast = Vec::new();

    if ast.len() > 2 {
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
    } else {
        opt_ast = ast.clone();
    }

    opt_ast
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
    println!("mul_copy_loop_opt() called!");
    ast.clone()
}

/// Optimizes an `Ast`.
pub fn optimize(opt_config: &OptConfig, ast: &Ast) -> Ast {
    let mut opt_ast = ast.clone();

    // always optimize out comment loops and unused loops
    opt_ast = comment_loop_opt(&opt_ast);
    opt_ast = unused_loop_opt(&opt_ast);

    // optimize according to the opt_config
    if opt_config.clear_loop_opt {
        opt_ast = clear_loop_opt(&opt_ast);
    }
    if opt_config.mul_copy_loop_opt {
        opt_ast = mul_copy_loop_opt(&opt_ast);
    }
    if opt_config.scan_loop_opt {
        opt_ast = scan_loop_opt(&opt_ast);
    }

    // contract_opt is always executed last, as the other optimizations assume
    // the Ast is not contracted.
    if opt_config.contract_opt {
        opt_ast = contract_opt(&opt_ast);
    }

    opt_ast
}
