use crate::instruction::Opcode;

pub struct VirtualMachine {
    pub registers: [i32; 32], // register set
    heap: Vec<u8>,            // heap memory
    pc: usize,                // program counter
    pub program: Vec<u8>,     // vector to store the bytecode
    remainder: u32,           // to store the remainder of a division
    equal_flag: bool,         // to store the result of the last comparison operation
}

impl Default for VirtualMachine {
    fn default() -> Self {
        VirtualMachine::new()
    }
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 32],
            heap: vec![],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
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

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
    }

    fn execute_instruction(&mut self) -> bool {
        // The program counter must be within the program
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                // Cast to usize so to use it as index into the array
                let register_idx = self.next_8_bits() as usize;
                // Read the number
                let number = self.next_16_bits();
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
            Opcode::GTE => {
                // Get values from registers
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                // Store the result in the dedicated register
                self.equal_flag = val1 >= val2;
                self.next_8_bits();
            }
            Opcode::LTE => {
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
            Opcode::JNEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::ALOC => {
                let idx = self.next_8_bits() as usize;
                let nbytes = self.registers[idx];
                let new_len = self.heap.len() as i32 + nbytes;
                self.heap.resize(new_len as usize, 0);
            }
            Opcode::INC => {
                let idx = self.next_8_bits() as usize;
                self.registers[idx] += 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DEC => {
                let idx = self.next_8_bits() as usize;
                self.registers[idx] -= 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::HLT => {
                println!("Executing HLT");
                return true;
            }
            Opcode::IGL => {
                println!("Illegal code received");
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = VirtualMachine::new();
        // check if all registers are set to 0
        for register in vm.registers.into_iter() {
            assert_eq!(register, 0);
        }
        // check if heap if empty
        assert_eq!(vm.heap.is_empty(), true);
        // check if pc is 0
        assert_eq!(vm.pc, 0);
        // check if program is empty
        assert_eq!(vm.program.is_empty(), true);
        // check if remainder is 0
        assert_eq!(vm.remainder, 0);
        // check if equal flag is false
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_add_byte() {
        let mut vm = VirtualMachine::new();
        let bytes: Vec<u8> = vec![0, 23, 58, 231, 145];

        for &byte in bytes.iter() {
            vm.add_byte(byte);
        }

        for (idx, &byte) in vm.program.iter().enumerate() {
            assert_eq!(byte, bytes[idx]);
        }
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
    fn test_opcode_gte() {
        let mut test_vm = VirtualMachine::new();
        test_vm.registers[3] = 5;
        test_vm.registers[7] = 10;
        test_vm.registers[9] = 10;
        let test_program = vec![
            Opcode::GTE as u8,
            3,
            7,
            0,
            Opcode::GTE as u8,
            7,
            3,
            0,
            Opcode::GTE as u8,
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
    fn test_opcode_lte() {
        let mut test_vm = VirtualMachine::new();
        test_vm.registers[3] = 5;
        test_vm.registers[7] = 10;
        test_vm.registers[9] = 10;
        let test_program = vec![
            Opcode::LTE as u8,
            3,
            7,
            0,
            Opcode::LTE as u8,
            7,
            3,
            0,
            Opcode::LTE as u8,
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
        let test_program = vec![Opcode::EQ as u8, 1, 2, 0, Opcode::JEQ as u8, 8, 0, 0];
        test_vm.program = test_program;
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 13);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut test_vm = VirtualMachine::new();
        test_vm.registers[8] = 9;
        test_vm.registers[1] = 8;
        test_vm.registers[2] = 5;
        let test_program = vec![Opcode::EQ as u8, 1, 2, 0, Opcode::JNEQ as u8, 8, 0, 0];
        test_vm.program = test_program;
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 9);
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = VirtualMachine::new();
        let test_program = vec![Opcode::ALOC as u8, 1, Opcode::ALOC as u8, 2];
        test_vm.registers[1] = 100;
        test_vm.registers[2] = 700;
        test_vm.program = test_program;
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 100);
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 800);
    }

    #[test]
    fn test_opcode_inc() {
        let mut test_vm = VirtualMachine::new();
        let test_program = vec![Opcode::INC as u8, 1, 0, 0];
        test_vm.registers[1] = 50;
        test_vm.program = test_program;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1], 51);
    }

    #[test]
    fn test_opcode_dec() {
        let mut test_vm = VirtualMachine::new();
        let test_program = vec![Opcode::DEC as u8, 1, 0, 0];
        test_vm.registers[1] = 50;
        test_vm.program = test_program;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1], 49);
    }
}
