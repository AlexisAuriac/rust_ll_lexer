use ll_lexer::get_rt;
use ll_lexer::lexer;
use ll_lexer::symbol::Symbol;

#[test]
fn single_value() {
    assert_eq!(
        lexer(String::from("1"), get_rt()),
        Ok(vec![Symbol::TsNbr(1)])
    );
}

#[test]
fn multi_digit_nbr() {
    assert_eq!(
        lexer(String::from("12"), get_rt()),
        Ok(vec![Symbol::TsNbr(12)])
    );
}

#[test]
fn simple_operation() {
    assert_eq!(
        lexer(String::from("12+3"), get_rt()),
        Ok(vec![Symbol::TsNbr(12), Symbol::TsPlus, Symbol::TsNbr(3)])
    );
    assert_eq!(
        lexer(String::from("12-1234"), get_rt()),
        Ok(vec![Symbol::TsNbr(12), Symbol::TsLess, Symbol::TsNbr(1234)])
    );
    assert_eq!(
        lexer(String::from("4*32"), get_rt()),
        Ok(vec![Symbol::TsNbr(4), Symbol::TsTimes, Symbol::TsNbr(32)])
    );
    assert_eq!(
        lexer(String::from("12/4"), get_rt()),
        Ok(vec![Symbol::TsNbr(12), Symbol::TsDivide, Symbol::TsNbr(4)])
    );
    assert_eq!(
        lexer(String::from("12%4"), get_rt()),
        Ok(vec![Symbol::TsNbr(12), Symbol::TsModulo, Symbol::TsNbr(4)])
    );
}

#[test]
fn multiple_operations() {
    assert_eq!(
        lexer(String::from("10+15-10*13+6/3%4"), get_rt()),
        Ok(vec![
            Symbol::TsNbr(10),
            Symbol::TsPlus,
            Symbol::TsNbr(15),
            Symbol::TsLess,
            Symbol::TsNbr(10),
            Symbol::TsTimes,
            Symbol::TsNbr(13),
            Symbol::TsPlus,
            Symbol::TsNbr(6),
            Symbol::TsDivide,
            Symbol::TsNbr(3),
            Symbol::TsModulo,
            Symbol::TsNbr(4),
        ])
    );
}

#[test]
fn simple_brackets() {
    assert_eq!(
        lexer(String::from("(1)"), get_rt()),
        Ok(vec![Symbol::TsLParens, Symbol::TsNbr(1), Symbol::TsRParens])
    );
    assert_eq!(
        lexer(String::from("(1+2)"), get_rt()),
        Ok(vec![
            Symbol::TsLParens,
            Symbol::TsNbr(1),
            Symbol::TsPlus,
            Symbol::TsNbr(2),
            Symbol::TsRParens,
        ])
    );
    assert_eq!(
        lexer(String::from("(1+2)+3"), get_rt()),
        Ok(vec![
            Symbol::TsLParens,
            Symbol::TsNbr(1),
            Symbol::TsPlus,
            Symbol::TsNbr(2),
            Symbol::TsRParens,
            Symbol::TsPlus,
            Symbol::TsNbr(3),
        ])
    );
    assert_eq!(
        lexer(String::from("3+(1+2)"), get_rt()),
        Ok(vec![
            Symbol::TsNbr(3),
            Symbol::TsPlus,
            Symbol::TsLParens,
            Symbol::TsNbr(1),
            Symbol::TsPlus,
            Symbol::TsNbr(2),
            Symbol::TsRParens,
        ])
    );
}

#[test]
fn complicated_brackets() {
    assert_eq!(
        lexer(String::from("(1+2)+(2*(3)+(5-6))"), get_rt()),
        Ok(vec![
            Symbol::TsLParens,
            Symbol::TsNbr(1),
            Symbol::TsPlus,
            Symbol::TsNbr(2),
            Symbol::TsRParens,
            Symbol::TsPlus,
            Symbol::TsLParens,
            Symbol::TsNbr(2),
            Symbol::TsTimes,
            Symbol::TsLParens,
            Symbol::TsNbr(3),
            Symbol::TsRParens,
            Symbol::TsPlus,
            Symbol::TsLParens,
            Symbol::TsNbr(5),
            Symbol::TsLess,
            Symbol::TsNbr(6),
            Symbol::TsRParens,
            Symbol::TsRParens,
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
