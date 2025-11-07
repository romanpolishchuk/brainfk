use std::time::Instant;

#[derive(PartialEq)]
enum Opcode {
    Right(u64),
    Left(u64),
    Plus(u64),
    Minus(u64),
    Out,
    Input,
    OpenLoop(usize),
    CloseLoop(usize),
}

fn optimize(program: Vec<char>) -> Vec<Opcode> {
    let mut opcodes: Vec<Opcode> = vec![];
    let mut stack: Vec<usize> = vec![];
    for opcode in program {
        match opcode {
            '>' => {
                if let Some(op) = opcodes.last_mut() {
                    if let Opcode::Right(n) = op {
                        *n += 1;
                    } else {
                        opcodes.push(Opcode::Right(1));
                    }
                } else {
                    opcodes.push(Opcode::Right(1));
                }
            }
            '<' => {
                if let Some(op) = opcodes.last_mut() {
                    if let Opcode::Left(n) = op {
                        *n += 1;
                    } else {
                        opcodes.push(Opcode::Left(1));
                    }
                } else {
                    opcodes.push(Opcode::Left(1));
                }
            }
            '+' => {
                if let Some(op) = opcodes.last_mut() {
                    if let Opcode::Plus(n) = op {
                        *n += 1;
                    } else {
                        opcodes.push(Opcode::Plus(1));
                    }
                } else {
                    opcodes.push(Opcode::Plus(1));
                }
            }
            '-' => {
                if let Some(op) = opcodes.last_mut() {
                    if let Opcode::Minus(n) = op {
                        *n += 1;
                    } else {
                        opcodes.push(Opcode::Minus(1));
                    }
                } else {
                    opcodes.push(Opcode::Minus(1));
                }
            }
            '.' => {
                opcodes.push(Opcode::Out);
            }
            ',' => {
                opcodes.push(Opcode::Input);
            }
            '[' => {
                opcodes.push(Opcode::OpenLoop(0));
                stack.push(opcodes.len() - 1);
            }
            ']' => {
                let open_index = stack.pop().unwrap();

                opcodes.push(Opcode::CloseLoop(open_index));
                opcodes[open_index] = Opcode::OpenLoop(opcodes.len() - 1);
            }
            _ => {}
        }
    }

    opcodes
}

fn compile(program: Vec<char>) {
    let opcodes = optimize(program);
    let mut file = String::new();

    file += "fn main(){\n";
    file += "
        let mut ram: [u8; 30000] = [0; 30000];
        let mut data_pointer = 0 as usize;
    ";

    for op in opcodes {
        match op {
            Opcode::Right(count) => {
                file += &format!("data_pointer += {};\n", count);
            }
            Opcode::Left(count) => {
                file += &format!("data_pointer -= {};\n", count);
            }
            Opcode::Plus(count) => {
                file += &format!("ram[data_pointer] += {};\n", count);
            }
            Opcode::Minus(count) => {
                file += &format!("ram[data_pointer] -= {};\n", count);
            }
            Opcode::Out => {
                file += "print!(\"{}\", ram[data_pointer] as char);\n";
            }
            Opcode::Input => {
                unimplemented!(",");
            }
            Opcode::OpenLoop(_) => {
                file += "while ram[data_pointer] != 0 {\n";
            }
            Opcode::CloseLoop(_) => {
                file += "}\n";
            }
        }
    }

    file += "}";
    std::fs::write("optimized.rs", file);
}

fn run(opcodes: Vec<Opcode>) {
    let mut ram: [u8; 30000] = [0; 30000];
    let mut data_pointer = 0 as usize;
    let mut program_pointer = 0 as usize;

    while program_pointer < opcodes.len() {
        let opcode = &opcodes[program_pointer];
        program_pointer += 1;

        match opcode {
            Opcode::Right(count) => {
                data_pointer += *count as usize;
            }
            Opcode::Left(count) => {
                data_pointer -= *count as usize;
            }
            Opcode::Plus(count) => {
                ram[data_pointer] += *count as u8;
            }
            Opcode::Minus(count) => {
                ram[data_pointer] -= *count as u8;
            }
            Opcode::Out => {
                print!("{}", ram[data_pointer] as char);
            }
            Opcode::Input => {
                unimplemented!(",");
            }
            Opcode::OpenLoop(close_index) => {
                if ram[data_pointer] == 0 {
                    program_pointer = close_index + 1;
                }
            }
            Opcode::CloseLoop(open_index) => {
                if ram[data_pointer] != 0 {
                    program_pointer = open_index + 1;
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path;

    if args.len() == 1 {
        path = "./assets/program.bf";
    } else {
        path = &args[1];
    }

    let program: Vec<char> = std::fs::read_to_string(path).unwrap().chars().collect();

    // compile(program);

    let timer = Instant::now();
    let opcodes = optimize(program);
    println!("Optimization: {}/s", timer.elapsed().as_secs_f64());

    let timer = Instant::now();
    run(opcodes);
    println!("Run: {}/s", timer.elapsed().as_secs_f64());
}
