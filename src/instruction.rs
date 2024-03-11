use nom::types::CompleteStr;

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
    GTE,  // Short for greater than or equal. Compare if a number is greater than or equal to other
    LTE,  // Short for greater than or equal. Compare if a number is less than or equal to other
    JEQ,  // Short for jump if equal. Jump if the last comparison was evaluated to true
    JNEQ, // Short for jump if not equal. Jump if the last comparison was evaluated to false
    ALOC, // Short for allocate. Extends the size of the heap by the number of bytes in the corresponding register
    INC,  // Short for increment. Increments the value in the register provided by 1
    DEC,  // Short for decrement. Decrements the value in the register provided by 1
    IGL,  // Short for illegal. Terminates with an error
}

// Create opcode from byte
impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,
            17 => Opcode::ALOC,
            18 => Opcode::INC,
            19 => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

// Create opcode from case insensitive string
impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gte") => Opcode::GTE,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lte") => Opcode::LTE,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("jeq") => Opcode::JEQ,
            CompleteStr("jneq") => Opcode::JNEQ,
            CompleteStr("aloc") => Opcode::ALOC,
            CompleteStr("inc") => Opcode::INC,
            CompleteStr("dec") => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
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

#[test]
fn test_str_to_opcode() {
    let opcode = Opcode::from(CompleteStr("load"));
    assert_eq!(opcode, Opcode::LOAD);
    let opcode = Opcode::from(CompleteStr("jmp"));
    assert_eq!(opcode, Opcode::JMP);
    let opcode = Opcode::from(CompleteStr("gte"));
    assert_eq!(opcode, Opcode::GTE);
    let opcode = Opcode::from(CompleteStr("caca"));
    assert_eq!(opcode, Opcode::IGL);
}
