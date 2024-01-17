use crate::assembler::opcode_parsers::opcode;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results: Vec<u8> = vec![];
        match &self.opcode {
            Token::Op { code } => match code {
                _ => {
                    let b: u8 = *code as u8;
                    results.push(b);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as i16;
                let byte1: u8 = converted as u8;
                let byte2: u8 = (converted >> 8).try_into().unwrap();
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        i: integer_operand >>
        (

            AssemblerInstruction{
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);

#[test]
fn test_parse_instruction_form_one() {
    let result = instruction_one(CompleteStr("load $0 #100\n"));
    assert_eq!(
        result,
        Ok((
            CompleteStr(""),
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::LOAD },
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 100 }),
                operand3: None
            }
        ))
    );
}
