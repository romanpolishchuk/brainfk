fn main() {
    let mut ram: Vec<u8> = vec![0; 1000000];
    let mut stack: Vec<usize> = vec![];
    let mut data_pointer = 0 as usize;
    let mut program_pointer = 0 as usize;

    let program: Vec<char> = std::fs::read_to_string("./assets/program.bf")
        .unwrap()
        .chars()
        .collect();

    while program_pointer < program.len() {
        let opcode = program[program_pointer];
        //println!("{}, {}", program_pointer, opcode);
        program_pointer += 1;

        match opcode {
            '>' => {
                data_pointer += 1;
            }
            '<' => {
                data_pointer -= 1;
            }
            '+' => {
                ram[data_pointer] = ram[data_pointer].wrapping_add(1);
            }
            '-' => {
                ram[data_pointer] = ram[data_pointer].wrapping_sub(1);
            }
            '.' => {
                print!("{}", ram[data_pointer] as char);
            }
            ',' => {
                unimplemented!(",");
            }
            '[' => {
                if ram[data_pointer] == 0 {
                    let mut count = 1;
                    while count > 0 {
                        let op = program[program_pointer];
                        if op == '[' {
                            count += 1;
                        } else if op == ']' {
                            count -= 1;
                        }
                        program_pointer += 1;
                    }
                } else {
                    //println!("[ {}", program_pointer - 1);
                    stack.push(program_pointer - 1);
                }
            }
            ']' => {
                //println!("] {}", program_pointer - 1);
                if ram[data_pointer] != 0 {
                    program_pointer = stack.pop().unwrap();
                } else {
                    stack.pop().unwrap();
                }
            }
            _ => {}
        }
    }
}
