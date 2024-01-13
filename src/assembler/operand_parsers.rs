use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

// Parser for integer numbers
// We preface with `#` in our assembly language: #100
named!(integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            // Look for `#` and pass the result
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

#[test]
fn test_parse_integer_operand() {
    let result = integer_operand(CompleteStr("#10"));
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::IntegerOperand { value: 10 });

    let result = integer_operand(CompleteStr("10"));
    assert_eq!(result.is_ok(), false);
}
