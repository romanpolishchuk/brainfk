use std::time::Instant;

#[derive(PartialEq)]
enum Opcode {
    Right(u64),
    Left(u64),
    Plus(u64),
    Minus(u64),
    Out,
    Input,
    OpenLoop(usize, usize),
    CloseLoop(usize, usize),
    None,
}

fn main() {
    let timer = Instant::now();

    let mut ram: Vec<u8> = vec![0; 1000000];
    let mut data_pointer = 0 as usize;
    let mut program_pointer = 0 as usize;

    let program: Vec<char> = std::fs::read_to_string("./assets/program.bf")
        .unwrap()
        .chars()
        .filter(|c| {
            *c == '>'
                || *c == '<'
                || *c == '+'
                || *c == '-'
                || *c == '.'
                || *c == ','
                || *c == '['
                || *c == ']'
        })
        .collect();

    let mut stack: Vec<usize> = vec![];
    let mut opcodes: Vec<Opcode> = vec![];
    while program_pointer < program.len() {
        let opcode = program[program_pointer];
        program_pointer += 1;

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
                opcodes.push(Opcode::OpenLoop(opcodes.len(), 0));
                stack.push(opcodes.len() - 1);
            }
            ']' => {
                let open_index = stack.pop().unwrap();
                if let Opcode::OpenLoop(open, _) = opcodes[open_index] {
                    opcodes.push(Opcode::CloseLoop(opcodes.len(), open));
                    opcodes[open] = Opcode::OpenLoop(open, opcodes.len() - 1);
                }
            }
            _ => {}
        }
    }

    program_pointer = 0;

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
                ram[data_pointer] = ram[data_pointer].wrapping_add(*count as u8);
            }
            Opcode::Minus(count) => {
                ram[data_pointer] = ram[data_pointer].wrapping_sub(*count as u8);
            }
            Opcode::Out => {
                print!("{}", ram[data_pointer] as char);
            }
            Opcode::Input => {
                unimplemented!(",");
            }
            Opcode::OpenLoop(_, close_index) => {
                if ram[data_pointer] == 0 {
                    program_pointer = close_index + 1;
                }
            }
            Opcode::CloseLoop(_, open_index) => {
                if ram[data_pointer] != 0 {
                    program_pointer = open_index + 1;
                }
            }
            _ => {}
        }
    }

    println!("Took: {}s", timer.elapsed().as_secs_f32());
}
