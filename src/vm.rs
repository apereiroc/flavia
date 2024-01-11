use crate::instruction::Opcode;

pub struct VirtualMachine {
    registers: [i32; 32], // register set
    pc: usize,            // program counter
    program: Vec<u8>,     // vector to store the bytecode
    remainder: u32,       // to store the remainder of a division
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
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
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        // The program counter must be within the program
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                // Cast to usize so to use it as index into the array
                let register_idx = self.next_8_bits() as usize;
                // Read the number
                let number = self.next_16_bits() as u16;
                // Cast the number as our registers are i32
                self.registers[register_idx] = number as i32;
            }
            Opcode::ADD => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 + val2;
            }
            Opcode::SUB => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 - val2;
            }
            Opcode::MUL => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 * val2;
            }
            Opcode::DIV => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 / val2;
                self.remainder = (val1 % val2) as u32;
            }
            Opcode::HLT => {
                println!("Executing HLT");
                return false;
            }
            Opcode::IGL => {
                println!("Illegal code received");
                return false;
            }
        }
        return true;
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
    test_vm.run_once();
    assert_eq!(test_vm.pc, 1);
}

#[test]
fn test_opcode_igl() {
    let mut test_vm = VirtualMachine::new();
    let test_program = vec![Opcode::IGL as u8, 0, 0, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 1);
}

#[test]
fn test_opcode_load() {
    let mut test_vm = VirtualMachine::new();
    let test_program = vec![Opcode::LOAD as u8, 0, 1, 244]; // 256 * 1 + 244 = 500 :-)
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.registers[0], 500);
}

#[test]
fn test_opcode_add() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[4] = 5;
    test_vm.registers[9] = 3;
    let test_program = vec![Opcode::ADD as u8, 4, 9, 17];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.registers[17], 8);
}
