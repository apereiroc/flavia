use crate::instruction::Opcode;

pub struct VirtualMachine {
    registers: [i32; 32], // register set
    pc: usize,            // program counter
    program: Vec<u8>,     // vector to store the bytecode
    remainder: u32,       // to store the remainder of a division
    equal_flag: bool,     // to store the result of the last comparison operation
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
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
            Opcode::JMP => {
                // Get the register where the memory address where to move to is stored
                let target = self.registers[self.next_8_bits() as usize];
                // Assign it
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                // Get the register where the number of bytes is stored
                let value = self.registers[self.next_8_bits() as usize];
                // Increase it
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                // Get the register where the number of bytes is stored
                let value = self.registers[self.next_8_bits() as usize];
                // Decrease it
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 == val2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 != val2;
                self.next_8_bits();
            }
            Opcode::GT => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 > val2;
                self.next_8_bits();
            }
            Opcode::LT => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 < val2;
                self.next_8_bits();
            }
            Opcode::GTQ => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 >= val2;
                self.next_8_bits();
            }
            Opcode::LTQ => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 <= val2;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
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

#[test]
fn test_opcode_sub() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[8] = 5;
    test_vm.registers[3] = 3;
    let test_program = vec![Opcode::SUB as u8, 8, 3, 7];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.registers[7], 2);
}

#[test]
fn test_opcode_mul() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[8] = 9;
    test_vm.registers[3] = 5;
    let test_program = vec![Opcode::MUL as u8, 8, 3, 12];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.registers[12], 45);
}

#[test]
fn test_opcode_div() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 15;
    test_vm.registers[6] = 4;
    let test_program = vec![Opcode::DIV as u8, 3, 6, 19];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.registers[19], 3);
    assert_eq!(test_vm.remainder, 3);
}

#[test]
fn test_opcode_jmp() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 7;
    let test_program = vec![Opcode::JMP as u8, 3, 0, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 7);
}

#[test]
fn test_opcode_jmpf() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[8] = 20;
    let test_program = vec![Opcode::JMPF as u8, 8, 0, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 22);
}

#[test]
fn test_opcode_jmpb() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[7] = 2;
    let test_program = vec![Opcode::LOAD as u8, 0, 0, 0, Opcode::JMPB as u8, 7, 0, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    test_vm.run_once();
    assert_eq!(test_vm.pc, 4);
}

#[test]
fn test_opcode_eq() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 10;
    test_vm.registers[7] = 10;
    let test_program = vec![Opcode::EQ as u8, 3, 7, 0, Opcode::EQ as u8, 3, 5, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
}

#[test]
fn test_opcode_neq() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 10;
    test_vm.registers[7] = 10;
    let test_program = vec![Opcode::NEQ as u8, 3, 7, 0, Opcode::NEQ as u8, 3, 5, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
}

#[test]
fn test_opcode_gt() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 5;
    test_vm.registers[7] = 10;
    let test_program = vec![Opcode::GT as u8, 3, 7, 0, Opcode::GT as u8, 7, 3, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
}

#[test]
fn test_opcode_lt() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 5;
    test_vm.registers[7] = 10;
    let test_program = vec![Opcode::LT as u8, 3, 7, 0, Opcode::LT as u8, 7, 3, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
}

#[test]
fn test_opcode_gtq() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 5;
    test_vm.registers[7] = 10;
    test_vm.registers[9] = 10;
    let test_program = vec![
        Opcode::GTQ as u8,
        3,
        7,
        0,
        Opcode::GTQ as u8,
        7,
        3,
        0,
        Opcode::GTQ as u8,
        7,
        9,
        0,
    ];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
}

#[test]
fn test_opcode_ltq() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[3] = 5;
    test_vm.registers[7] = 10;
    test_vm.registers[9] = 10;
    let test_program = vec![
        Opcode::LTQ as u8,
        3,
        7,
        0,
        Opcode::LTQ as u8,
        7,
        3,
        0,
        Opcode::LTQ as u8,
        7,
        9,
        0,
    ];
    test_vm.program = test_program;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
}

#[test]
fn test_opcode_jeq() {
    let mut test_vm = VirtualMachine::new();
    test_vm.registers[8] = 13;
    test_vm.registers[1] = 5;
    test_vm.registers[2] = 5;
    let test_program = vec![Opcode::EQ as u8, 1, 2, 0, Opcode::JMP as u8, 8, 0, 0];
    test_vm.program = test_program;
    test_vm.run_once();
    test_vm.run_once();
    assert_eq!(test_vm.pc, 13);
}
