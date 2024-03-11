use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

// Parser for register index
// We preface with `$` in our assembly language:
// $3
named!(pub register <CompleteStr, Token>,
    // Consume whitespaces
    ws!(
        do_parse!(
            // Look for $ and pass the result
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                  reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}
