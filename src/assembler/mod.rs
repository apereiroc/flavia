use crate::assembler::program_parsers::{program, Program};
use crate::assembler::symbols::{Symbol, SymbolTable, SymbolType};
use crate::instruction::Opcode;
use nom::types::CompleteStr;
pub mod directive_parsers;
pub mod instruction_parsers;
pub mod label_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parsers;
pub mod register_parsers;
pub mod symbols;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerPhase {
    First,
    Second,
}

impl Default for AssemblerPhase {
    fn default() -> Self {
        AssemblerPhase::First
    }
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

impl Assembler {
    // Assembler constructed from the first assembler phase
    // and a new symbol table, which initialises its internal vector of symbols
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    // Takes a raw string reference which contains the program.
    // If can be parsed, goes to first assembler phase which extracts the labels and builds the
    // symbol table.
    // In the second phase, all instructions are transformed into bytes.
    // The final vector of bytes (program) is returned
    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                self.process_first_phase(&program);
                Some(self.process_second_phase(&program))
            }
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbols);
            program.append(&mut bytes);
        }
        program
    }

    // Go through every instruction and look for label declarations
    // These are of the form: some_name: <opcode> ...
    // If some one is found, it is added to the symbol vector inside the symbol table
    fn extract_labels(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                if let Some(name) = i.label_name() {
                    let symbol = Symbol::new(name, SymbolType::Label, c);
                    self.symbols.add_symbol(symbol);
                };
            }
            c += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::VirtualMachine;

    use super::*;

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string =
            "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njeq @test\nhlt";
        let program = asm.assemble(test_string).unwrap();
        let mut vm = VirtualMachine::new();
        assert_eq!(program.len(), 28);
        vm.add_bytes(program);
        assert_eq!(vm.program.len(), 28);
    }
}
