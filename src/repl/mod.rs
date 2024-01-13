use crate::instruction;
use crate::vm::VirtualMachine;
use std;
use std::io;
use std::io::Write;

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
                    println!("Invalid input");
                }
            }
        }
    }
}
