use ll_lexer::get_rt;
use ll_lexer::lexer;
use ll_lexer::symbol::LexSym;

#[test]
fn single_value() {
    assert_eq!(
        lexer(String::from("1"), get_rt()),
        Ok(vec![LexSym::TsNbr(1)])
    );
}

#[test]
fn multi_digit_nbr() {
    assert_eq!(
        lexer(String::from("12"), get_rt()),
        Ok(vec![LexSym::TsNbr(12)])
    );
}

#[test]
fn simple_operation() {
    assert_eq!(
        lexer(String::from("12+3"), get_rt()),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsPlus, LexSym::TsNbr(3)])
    );
    assert_eq!(
        lexer(String::from("12-1234"), get_rt()),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsLess, LexSym::TsNbr(1234)])
    );
    assert_eq!(
        lexer(String::from("4*32"), get_rt()),
        Ok(vec![LexSym::TsNbr(4), LexSym::TsTimes, LexSym::TsNbr(32)])
    );
    assert_eq!(
        lexer(String::from("12/4"), get_rt()),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsDivide, LexSym::TsNbr(4)])
    );
    assert_eq!(
        lexer(String::from("12%4"), get_rt()),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsModulo, LexSym::TsNbr(4)])
    );
}

#[test]
fn multiple_operations() {
    assert_eq!(
        lexer(String::from("10+15-10*13+6/3%4"), get_rt()),
        Ok(vec![
            LexSym::TsNbr(10),
            LexSym::TsPlus,
            LexSym::TsNbr(15),
            LexSym::TsLess,
            LexSym::TsNbr(10),
            LexSym::TsTimes,
            LexSym::TsNbr(13),
            LexSym::TsPlus,
            LexSym::TsNbr(6),
            LexSym::TsDivide,
            LexSym::TsNbr(3),
            LexSym::TsModulo,
            LexSym::TsNbr(4),
        ])
    );
}

#[test]
fn simple_brackets() {
    assert_eq!(
        lexer(String::from("(1)"), get_rt()),
        Ok(vec![
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsRBracket
        ])
    );
    assert_eq!(
        lexer(String::from("(1+2)"), get_rt()),
        Ok(vec![
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsPlus,
            LexSym::TsNbr(2),
            LexSym::TsRBracket,
        ])
    );
    assert_eq!(
        lexer(String::from("(1+2)+3"), get_rt()),
        Ok(vec![
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsPlus,
            LexSym::TsNbr(2),
            LexSym::TsRBracket,
            LexSym::TsPlus,
            LexSym::TsNbr(3),
        ])
    );
    assert_eq!(
        lexer(String::from("3+(1+2)"), get_rt()),
        Ok(vec![
            LexSym::TsNbr(3),
            LexSym::TsPlus,
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsPlus,
            LexSym::TsNbr(2),
            LexSym::TsRBracket,
        ])
    );
}

#[test]
fn complicated_brackets() {
    assert_eq!(
        lexer(String::from("(1+2)+(2*(3)+(5-6))"), get_rt()),
        Ok(vec![
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsPlus,
            LexSym::TsNbr(2),
            LexSym::TsRBracket,
            LexSym::TsPlus,
            LexSym::TsLBracket,
            LexSym::TsNbr(2),
            LexSym::TsTimes,
            LexSym::TsLBracket,
            LexSym::TsNbr(3),
            LexSym::TsRBracket,
            LexSym::TsPlus,
            LexSym::TsLBracket,
            LexSym::TsNbr(5),
            LexSym::TsLess,
            LexSym::TsNbr(6),
            LexSym::TsRBracket,
            LexSym::TsRBracket,
        ])
    );
}

#[test]
fn error_no_end_bracket() {
    assert_eq!(
        lexer(String::from("(1"), get_rt()),
        Err(String::from("Error: Incomplete syntax"))
    );
}

#[test]
fn error_empty_brackets() {
    assert_eq!(
        lexer(String::from("()"), get_rt()),
        Err(String::from("Error: Invalid syntax"))
    );
}

#[test]
fn error_no_2nd_operand() {
    assert_eq!(
        lexer(String::from("1+"), get_rt()),
        Err(String::from("Error: Incomplete syntax"))
    );
}

#[test]
fn error_too_large_number() {
    assert_eq!(
        lexer(String::from("12345678901234567890"), get_rt()),
        Err(String::from("Error: Too large number"))
    );
}
