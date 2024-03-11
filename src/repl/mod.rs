use nom::types::CompleteStr;

use crate::assembler::program_parsers::program;
use crate::vm::VirtualMachine;
use std;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use std::path::Path;

// Core structure for the REPL for the assembler
pub struct Repl {
    // To store all commands entered by the user
    command_buffer: Vec<String>,
    // The virtual machine the REPL will use to execute the code
    vm: VirtualMachine,
}

impl Default for Repl {
    fn default() -> Self {
        Repl::new()
    }
}

impl Repl {
    // Creates and returns a new assembly REPL
    pub fn new() -> Repl {
        // Print a welcome message with available commands
        println!("Welcome to flavia VM!");
        println!(
            "Type {:?}, {:?}, {:?}, {:?}, {:?} for more information",
            ".prog", ".reg", ".history", ".load_file", ".clear_program"
        );
        println!("Type {:?} to exit", ".q");

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
            print!(">>> ");
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
                ".prog" => {
                    println!("List of instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                }
                ".reg" => {
                    println!("List of the registers' contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of list.");
                }
                ".q" => {
                    println!("Exiting. Bye bye!");
                    std::process::exit(0);
                }
                ".clear_program" => {
                    self.vm.program.clear();
                }
                ".load_file" => {
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin
                        .read_line(&mut tmp)
                        .expect("Unable to read line from user");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("There was an error reading from the file");
                    let program = match program(CompleteStr(&contents)) {
                        // Rusts pattern matching is pretty powerful an can even be nested
                        Ok((_, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                ".history" => {
                    for cmd in &self.command_buffer {
                        println!("{}", cmd);
                    }
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if parsed_program.is_err() {
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
}
