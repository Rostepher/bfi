use std::io::File;

use syntax::{Ast, Ir};

/// Emits a C file with `file_name` created from `ast`.
pub fn emit_c(file_name: &str, ast: &Ast) {
    let mut file = match File::create(&Path::new(file_name)) {
        Ok(mut file) => file,
        Err(e)       => panic!("{}", e),
    };

    // standard includes, main function and mem/p declarations
    file.write_str("\
    #include <stdio.h>\n\
    #include <stdint.h>\n\
    #include <stdlib.h>\n\
    \n\
    int main(int argc, char **argv) {\n\
    uint8_t mem[65536] = {0};\n\
    uint32_t p = 0;\n\
    ");

    // write each ir as a line
    for ir in ast.iter() {
        let ir_str = match *ir {
            Ir::Add(value)         => format!("mem[p] += {};", value),
            Ir::Sub(value)         => format!("mem[p] -= {};", value),
            Ir::MoveLeft(steps)    => format!("p -= {};", steps),
            Ir::MoveRight(steps)   => format!("p += {};", steps),
            Ir::Read               => "mem[p] = getchar();".to_string(),
            Ir::Write              => "putchar(mem[p]);".to_string(),
            Ir::Open               => "while (mem[p] != 0) {".to_string(),
            Ir::Close              => "}".to_string(),
            Ir::Clear              => "mem[p] = 0;".to_string(),
            Ir::Copy(steps)        => format!("mem[p + {}] = mem[p];", steps),
            Ir::Mul(steps, factor) => format!("mem[p + {}] = mem[p] * {}", steps, factor),
            Ir::ScanLeft           => "while (mem[p] != 0) { p -= 1; }".to_string(),
            Ir::ScanRight          => "while (mem[p] != 0) { p += 1; }".to_string(),
        } + "\n";

        file.write(ir_str.as_bytes());
    }

    // close the main function
    file.write_str("}");
}
