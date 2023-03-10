use std::env;
use std::fs;
use std::io;

fn main() {
    const MAX_SIZE: usize = 65535;
    let mut arr: [u8; MAX_SIZE] = [0; 65535];
    let mut pointer: usize = 0;
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    println!("In {}", filepath);
    let contents = fs::read_to_string(filepath);
    dbg!(args);
    let mut result: String;
    match contents {
        Ok(val) => {
            result = val;
            result.retain(|c| {
                c == '<'
                    || c == '>'
                    || c == '+'
                    || c == '-'
                    || c == '.'
                    || c == ','
                    || c == '['
                    || c == ']'
            });
        }
        Err(err) => {
            panic!("Encounter error in reading file: {}", err);
        }
    }
    let inst: Vec<char> = result.chars().collect();
    let mut i: usize = 0;
    while i < inst.len() {
        match inst[i] {
            '>' => {
                if pointer < MAX_SIZE {
                    pointer += 1;
                } else {
                    panic!("Trying to increment pointer past max size of {}", MAX_SIZE);
                }
            }
            '<' => {
                pointer -= 1;
            }
            '+' => {
                arr[pointer] += 1;
            }
            '-' => {
                arr[pointer] -= 1;
            }
            '.' => {
                print!("{}", arr[pointer] as char)
            }
            ',' => {
                let mut input = String::new();
                println!("Input a character");
                while input == "" {
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                }
                arr[pointer] = input.chars().next().unwrap() as u8;
            }
            '[' => {
                if arr[pointer] == 0 {
                    let mut open_brace: u16 = 1;
                    while open_brace > 0 {
                        i += 1;
                        if inst[i] == '[' {
                            open_brace += 1;
                        } else if inst[i] == ']' {
                            open_brace -= 1;
                        }
                    }
                }
            }
            ']' => {
                let mut open_brace: u16 = 1;
                while open_brace > 0 {
                    i -= 1;
                    if inst[i] == '[' {
                        open_brace -= 1;
                    } else if inst[i] == ']' {
                        open_brace += 1;
                    }
                }
                i -= 1;
            }
            _ => {
                panic!("Invalid input");
            }
        }
        i += 1;
    }
}
