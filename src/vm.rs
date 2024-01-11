use crate::instruction::Opcode;

pub struct VirtualMachine {
    registers: [i32; 32], // register set
    pc: usize,            // program counter
    program: Vec<u8>,     // vector to store the bytecode
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        }
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    pub fn run(&mut self) {
        loop {
            // The program counter must be within the program
            if self.pc >= self.program.len() {
                break;
            }

            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("Executing HLT");
                    return;
                }
                _ => {
                    println!("Opcode not valid. Terminating.");
                    return;
                }
            }
        }
    }
}

#[test]
fn test_create_vm() {
    let vm = VirtualMachine::new();
    assert_eq!(vm.registers[0], 0);
}

#[test]
fn test_opcode_hlt() {
    let mut test_vm = VirtualMachine::new();
    let test_program = vec![0, 0, 0, 0];
    test_vm.program = test_program;
    test_vm.run();
    assert_eq!(test_vm.pc, 1);
}

#[test]
fn test_opcode_igl() {
    let mut test_vm = VirtualMachine::new();
    let test_program = vec![200, 0, 0, 0];
    test_vm.program = test_program;
    test_vm.run();
    assert_eq!(test_vm.pc, 1);
}
