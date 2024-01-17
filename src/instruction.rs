#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    HLT,  // Short for halt. Stops the execution.
    LOAD, // Load a number into a register
    ADD,  // Add two numbers and save the result in a register
    SUB,  // Subtract two numbers and save the result in a register
    MUL,  // Multiply two numbers and save the result in a register
    DIV,  // Divide two numbers and save the result in a register
    JMP,  // Short for jump. Absolute jump; move the program counter to a byte in the program
    JMPF, // Short for jump forwards. Relative jump; move the program counter forwards by x bytes
    JMPB, // Short for jump backwards. Relative jump; move the program counter backwards by x bytes
    EQ,   // Short for equal. Compare if two numbers are equal
    NEQ,  // Short for not equal. Compare if two numbers are not equal
    GT,   // Short for greater than. Compare if a number is greater than other
    LT,   // Short for less than. Compare if a number is less than other
    GTQ,  // Short for greater than or equal. Compare if a number is greater than or equal to other
    LTQ,  // Short for greater than or equal. Compare if a number is less than or equal to other
    JEQ,  // Short for jump if equal. Jump if the last comparison was evaluated to true
    JNEQ, // Short for jump if not equal. Jump if the last comparison was evaluated to false
    IGL,  // Short for illegal. Terminates with an error
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GT,
            12 => return Opcode::LT,
            13 => return Opcode::GTQ,
            14 => return Opcode::LTQ,
            15 => return Opcode::JEQ,
            16 => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

#[test]
fn test_create_hlt() {
    let opcode = Opcode::HLT;
    assert_eq!(opcode, Opcode::HLT);
}

#[test]
fn test_create_instruction() {
    let instruction = Instruction::new(Opcode::HLT);
    assert_eq!(instruction.opcode, Opcode::HLT);
}
