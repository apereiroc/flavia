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

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
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
                Opcode::LOAD => {
                    // Cast to usize so to use it as index into the array
                    let register_idx = self.next_8_bits() as usize;
                    // Read the number
                    let number = self.next_16_bits() as u16;
                    // Cast the number as our registers are i32
                    self.registers[register_idx] = number as i32;
                    // Continue the loop
                    continue;
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
    let test_program = vec![Opcode::HLT as u8, 0, 0, 0];
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

#[test]
fn test_load_opcode() {
    let mut test_vm = VirtualMachine::new();
    let test_program = vec![Opcode::LOAD as u8, 0, 1, 244]; // 256 * 1 + 244 = 500 :-)
    test_vm.program = test_program;
    test_vm.run();
    assert_eq!(test_vm.registers[0], 500);
}
