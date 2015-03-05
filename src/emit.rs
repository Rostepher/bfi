use std::old_io::{File, IoResult};

use syntax::{Ast, Ir, Left, Right};

/// Helper function to check that an IoResult is not Err.
fn check_io_result(io_result: &IoResult<()>) {
    match *io_result {
        Ok(_)      => {},
        Err(ref e) => panic!("{}", e),
    }
}

/// Emits a file which contains the optmized `Ast`.
pub fn emit_ir(file_name: &str, ast: &Ast) {
    let ir_file_name = &(file_name.to_string() + ".ir")[..];
    let mut file = match File::create(&Path::new(ir_file_name)) {
        Ok(file) => file,
        Err(e)       => panic!("{}", e),
    };

    for ir in ast.iter() {
        let io_result = file.write_line(&format!("{:?}", *ir)[..]);
        check_io_result(&io_result);
    }
}

/// Emits a C file with `file_name` created from `ast`.
pub fn emit_c(file_name: &str, ast: &Ast) {
    let c_file_name = &(file_name.to_string() + ".c")[..];
    let mut file = match File::create(&Path::new(c_file_name)) {
        Ok(file) => file,
        Err(e)       => panic!("{}", e),
    };

    // save the result values from each write
    let mut io_result;

    // standard includes, main function and mem/p declarations
    io_result = file.write_str("\
    #include <stdio.h>\n\
    #include <stdint.h>\n\
    #include <stdlib.h>\n\
    \n\
    int main(int argc, char **argv) {\n\
    uint8_t mem[65536] = {0};\n\
    uint32_t p = 0;\n\
    ");
    check_io_result(&io_result);

    // write each ir as a line
    for ir in ast.iter() {
        let ir_str = match *ir {
            Ir::Add(value)          => format!("mem[p] += {};", value),
            Ir::Sub(value)          => format!("mem[p] -= {};", value),
            Ir::Shift(Left, steps)  => format!("p -= {};", steps),
            Ir::Shift(Right, steps) => format!("p += {};", steps),
            Ir::Read                => "mem[p] = getchar();".to_string(),
            Ir::Write               => "putchar(mem[p]);".to_string(),
            Ir::Open                => "while (mem[p] != 0) {".to_string(),
            Ir::Close               => "}".to_string(),

            // optimizations
            Ir::Clear               => "mem[p] = 0;".to_string(),
            Ir::Scan(Left)          => "while (mem[p] != 0) { p -= 1; }".to_string(),
            Ir::Scan(Right)         => "while (mem[p] != 0) { p += 1; }".to_string(),
            Ir::Copy(Left, steps)   => format!("mem[p - {}] += mem[p];", steps),
            Ir::Copy(Right, steps)  => format!("mem[p + {}] += mem[p];", steps),
            Ir::Mul(Left, steps, factor) => {
                format!("mem[p - {}] += mem[p] * {};", steps, factor)
            },
            Ir::Mul(Right, steps, factor) => {
                format!("mem[p + {}] += mem[p] * {};", steps, factor)
            },
        } + "\n";

        io_result = file.write_all(ir_str.as_bytes());
        check_io_result(&io_result);
    }

    // close the main function
    io_result = file.write_str("}");
    check_io_result(&io_result);
}

/// Emits a Rust file with `file_name` created from `ast`.
pub fn emit_rust(file_name: &str, ast: &Ast) {
    let rs_file_name = &(file_name.to_string() + ".rs")[..];
    let mut file = match File::create(&Path::new(rs_file_name)) {
        Ok(file) => file,
        Err(e)       => panic!("{}", e),
    };

    // save the result values from each write
    let mut io_result;

    // standard includes, main function and mem/p declarations
    io_result = file.write_str("\
    #![allow(unstable)]\n\
    \n\
    /// Reads a `char` from `stdin`.\n\
    fn read_char() -> char {\n\
        match std::io::stdin().read_char() {\n\
            Ok(c)  => c,\n\
            Err(e) => panic!(\"{}\", e),\n\
        }\n\
    }\n\
    \n\
    /// Writes a `char` to `stdout`.\n\
    fn write_char(c: char) {\n\
        match std::io::stdout().write_char(c) {\n\
            Ok(_)  => {},\n\
            Err(e) => panic!(\"{}\", e),\n\
        }\n\
    }\n\
    \n\
    fn main() {\n\
    let mut mem = [0u8; 65_536us];\n\
    let mut p = 0us;\n\
    \n\
    ");
    check_io_result(&io_result);

    // write each ir as a line
    for ir in ast.iter() {
        let ir_str = match *ir {
            Ir::Add(value)          => format!("mem[p] += {};", value),
            Ir::Sub(value)          => format!("mem[p] -= {};", value),
            Ir::Shift(Left, steps)  => format!("p -= {};", steps),
            Ir::Shift(Right, steps) => format!("p += {};", steps),
            Ir::Read                => "mem[p] = read_char() as u8;".to_string(),
            Ir::Write               => "write_char(mem[p] as char);".to_string(),
            Ir::Open                => "while mem[p] != 0 {".to_string(),
            Ir::Close               => "}".to_string(),

            // optimizations
            Ir::Clear               => "mem[p] = 0;".to_string(),
            Ir::Scan(Left)          => "while mem[p] != 0 { p -= 1; }".to_string(),
            Ir::Scan(Right)         => "while mem[p] != 0 { p += 1; }".to_string(),
            Ir::Copy(Left, steps)   => format!("mem[p - {}] += mem[p];", steps),
            Ir::Copy(Right, steps)  => format!("mem[p + {}] += mem[p];", steps),
            Ir::Mul(Left, steps, factor) => {
                format!("mem[p - {}] += mem[p] * {};", steps, factor)
            },
            Ir::Mul(Right, steps, factor) => {
                format!("mem[p + {}] += mem[p] * {};", steps, factor)
            },
        } + "\n";

        io_result = file.write_all(ir_str.as_bytes());
        check_io_result(&io_result);
    }

    // close the main function
    io_result = file.write_str("}");
    check_io_result(&io_result);
}
