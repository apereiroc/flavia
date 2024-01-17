use nom::types::CompleteStr;

use crate::assembler::program_parsers::program;
use crate::vm::VirtualMachine;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

// Core structure for the REPL for the assembler
pub struct Repl {
    // To store all commands entered by the user
    command_buffer: Vec<String>,
    // The virtual machine the REPL will use to execute the code
    vm: VirtualMachine,
}

impl Repl {
    // Creates and returns a new assembly REPL
    pub fn new() -> Repl {
        Repl {
            command_buffer: vec![],
            vm: VirtualMachine::new(),
        }
    }

    // The infinite loop
    pub fn run(&mut self) {
        loop {
            // Allocate a new string to store the commands typed by the user
            // TODO: figure out how to create this outside of the loop and re-use it every
            // iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // `print!` does not automatically flush stdout like `println!` does
            // so we have to do that there for the user to see the prompt
            print!("> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Look at the string the user gave us
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            // Remove white spaces
            let buffer = buffer.trim();

            // Add command to history buffer
            self.command_buffer.push(buffer.to_string());

            // Process the input
            match buffer {
                ".program" => {
                    println!("List of instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of list.");
                }
                ".registers" => {
                    println!("List of the registers' contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of list.");
                }
                ".q" => {
                    println!("Exiting. Bye bye!");
                    std::process::exit(0);
                }
                ".history" => {
                    for cmd in &self.command_buffer {
                        println!("{}", cmd);
                    }
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    // Accepts a hexadecimal string without `0x`
    // Returns a Vec of u8
    // Example, to LOAD the number 1000 into register 1: 01 01 03 E8
    fn parse_hex(&mut self, num_str: &str) -> Result<Vec<u8>, ParseIntError> {
        let vec_of_str = num_str.split(" ").collect::<Vec<&str>>();
        let mut result: Vec<u8> = vec![];

        for hex_string in vec_of_str {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(this_result) => {
                    result.push(this_result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(result)
    }
}
