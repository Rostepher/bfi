use std::default::Default;

use syntax::{Ast, Ir};

/// The Optimization Configuration or `OptConfig` struct holds settings for which
/// optimization techniques to use.
#[derive(Clone, Copy, Show)]
pub struct OptConfig {
    contract_opt: bool,
    clear_loop_opt: bool,
    copy_loop_opt: bool,
    mul_loop_opt: bool,
    scan_loop_opt: bool,
}

impl OptConfig {
    #[inline]
    pub fn new(use_contract_opt: bool,
               use_clear_loop_opt: bool,
               use_copy_loop_opt: bool,
               use_mul_loop_opt: bool,
               use_scan_loop_opt: bool) -> OptConfig {
        OptConfig {
            contract_opt: use_contract_opt,
            clear_loop_opt: use_clear_loop_opt,
            copy_loop_opt: use_copy_loop_opt,
            mul_loop_opt: use_mul_loop_opt,
            scan_loop_opt: use_scan_loop_opt,
        }
    }
}

impl Default for OptConfig {
    fn default() -> OptConfig {
        OptConfig::new(true, true, true, true, true)
    }
}

/// Removes comment loop(s), which exist at the very beginning of the `Ast` and
/// would never execute as the current cell would be 0.
fn comment_loop_opt(ast: &mut Ast) {
    while !ast.is_empty() {
        match ast[0] {
            Ir::Loop(_) => { ast.remove(0); },
            _           => break, // all loops removed
        }
    }
}

/// Removes unused loops from an `Ast`. Two types of unused loops are removed,
/// comment loops and loops that start immediately after another loop closed,
/// which could never execute as, the current cell would be 0.
fn unused_loop_opt(ast: &mut Ast) {
    comment_loop_opt(ast);

    // TODO
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
/// Right(3), Add(3), Sub(3), Left(3)
/// ```
fn contract_opt(ast: &mut Ast) {
    // TODO
    println!("contract_opt() called!");
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
fn clear_loop_opt(ast: &mut Ast) {
    // TODO
    println!("clear_loop_opt() called!");
}

/// Optimizes 'copy loops' into a set of simplified instructions.
///
/// # Example
///
/// ```brainfuck
/// [->>+>+<<<]
/// ```
///
/// would be optimized to
///
/// ```
/// Copy(2), Copy(3), Clear(0)
/// ```
fn copy_loop_opt(ast: &mut Ast) {
    // TODO
    println!("copy_loop_opt() called!");
}

/// Optimizes 'multiplication loops' into a set of simplified instructions.
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
/// Mul(2, 5), Mul(3, 2), Clear(0)
/// ```
fn mul_loop_opt(ast: &mut Ast) {
    // TODO
    println!("mul_loop_opt() called!");
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
fn scan_loop_opt(ast: &mut Ast) {
    // TODO
    println!("scan_loop_opt() called!");
}

/// Optimizes an `Ast`.
pub fn optimize(opt_config: &OptConfig, ast: &mut Ast) {
    // always optimize out unused loops
    unused_loop_opt(ast);

    // optimize according to the opt_config
    if opt_config.clear_loop_opt { clear_loop_opt(ast); }
    if opt_config.copy_loop_opt { copy_loop_opt(ast); }
    if opt_config.mul_loop_opt { mul_loop_opt(ast); }
    if opt_config.scan_loop_opt { scan_loop_opt(ast); }

    // contract_opt is always executed last, as the other optimizations assume
    // the Ast is not contracted.
    if opt_config.contract_opt { contract_opt(ast); }
}
