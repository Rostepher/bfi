use std::io::File;

use syntax::{Ast, Ir, Left, Right};

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
            Ir::Move(Left, steps)  => format!("p -= {};", steps),
            Ir::Move(Right, steps) => format!("p += {};", steps),
            Ir::Read               => "mem[p] = getchar();".to_string(),
            Ir::Write              => "putchar(mem[p]);".to_string(),
            Ir::Open               => "while (mem[p] != 0) {".to_string(),
            Ir::Close              => "}".to_string(),

            // optimizations
            Ir::Clear              => "mem[p] = 0;".to_string(),
            Ir::Scan(Left)         => "while (mem[p] != 0) { p -= 1; }".to_string(),
            Ir::Scan(Right)        => "while (mem[p] != 0) { p += 1; }".to_string(),
            Ir::Copy(Left, steps)  => format!("mem[p - {}] = mem[p];", steps),
            Ir::Copy(Right, steps) => format!("mem[p + {}] = mem[p];", steps),
            Ir::Mul(Left, steps, factor) => {
                format!("mem[p - {}] = mem[p] * {}", steps, factor)
            },
            Ir::Mul(Right, steps, factor) => {
                format!("mem[p + {}] = mem[p] * {}", steps, factor)
            },
        } + "\n";

        file.write(ir_str.as_bytes());
    }

    // close the main function
    file.write_str("}");
}
