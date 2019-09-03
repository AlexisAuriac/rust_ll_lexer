use ll_lexer::get_rt;
use ll_lexer::lexer;
use ll_lexer::symbol::GramSym;
use ll_lexer::symbol::LexSym;

fn get_symbol_nbr(s: &mut String) -> Result<(LexSym, usize), String> {
    let mut nb: u32 = 0;
    let mut size = 0;

    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }

        if let (nb2, false) = nb.overflowing_mul(10) {
            nb = nb2;
        } else {
            return Err("Error: Too large number".to_string());
        }

        if let (nb2, false) = nb.overflowing_add(c.to_digit(10).unwrap()) {
            nb = nb2;
        } else {
            return Err("Error: Too large number".to_string());
        }

        size += 1;
    }

    return Ok((LexSym::TsNbr(nb), size));
}

fn get_symbol(s: &mut String) -> Result<(LexSym, usize), String> {
    if s.len() == 0 {
        return Ok((LexSym::TsEos, 0));
    }

    let c = s.chars().next().unwrap();

    return match c {
        '(' => Ok((LexSym::TsLBracket, 1)),
        ')' => Ok((LexSym::TsRBracket, 1)),
        '+' => Ok((LexSym::TsPlus, 1)),
        '-' => Ok((LexSym::TsLess, 1)),
        '*' => Ok((LexSym::TsTimes, 1)),
        '/' => Ok((LexSym::TsDivide, 1)),
        '%' => Ok((LexSym::TsModulo, 1)),
        '0'...'9' => get_symbol_nbr(s),
        _ => Ok((LexSym::TsInvalid, 1)),
    };
}

pub fn sym_to_expect(sym: &LexSym) -> GramSym {
    return match sym {
        LexSym::TsLBracket => GramSym::TsLBracket,
        LexSym::TsRBracket => GramSym::TsRBracket,
        LexSym::TsPlus => GramSym::TsPlus,
        LexSym::TsLess => GramSym::TsLess,
        LexSym::TsTimes => GramSym::TsTimes,
        LexSym::TsDivide => GramSym::TsDivide,
        LexSym::TsModulo => GramSym::TsModulo,
        LexSym::TsNbr { .. } => GramSym::TsNbr,
        LexSym::TsEos => GramSym::TsEos,
        LexSym::TsInvalid => GramSym::TsInvalid,
    };
}

#[test]
fn single_value() {
    assert_eq!(
        lexer(String::from("1"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![LexSym::TsNbr(1)])
    );
}

#[test]
fn multi_digit_nbr() {
    assert_eq!(
        lexer(String::from("12"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![LexSym::TsNbr(12)])
    );
}

#[test]
fn simple_operation() {
    assert_eq!(
        lexer(String::from("12+3"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsPlus, LexSym::TsNbr(3)])
    );
    assert_eq!(
        lexer(
            String::from("12-1234"),
            get_rt(),
            &get_symbol,
            &sym_to_expect
        ),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsLess, LexSym::TsNbr(1234)])
    );
    assert_eq!(
        lexer(String::from("4*32"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![LexSym::TsNbr(4), LexSym::TsTimes, LexSym::TsNbr(32)])
    );
    assert_eq!(
        lexer(String::from("12/4"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsDivide, LexSym::TsNbr(4)])
    );
    assert_eq!(
        lexer(String::from("12%4"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![LexSym::TsNbr(12), LexSym::TsModulo, LexSym::TsNbr(4)])
    );
}

#[test]
fn multiple_operations() {
    assert_eq!(
        lexer(
            String::from("10+15-10*13+6/3%4"),
            get_rt(),
            &get_symbol,
            &sym_to_expect
        ),
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
        lexer(String::from("(1)"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsRBracket
        ])
    );
    assert_eq!(
        lexer(String::from("(1+2)"), get_rt(), &get_symbol, &sym_to_expect),
        Ok(vec![
            LexSym::TsLBracket,
            LexSym::TsNbr(1),
            LexSym::TsPlus,
            LexSym::TsNbr(2),
            LexSym::TsRBracket,
        ])
    );
    assert_eq!(
        lexer(
            String::from("(1+2)+3"),
            get_rt(),
            &get_symbol,
            &sym_to_expect
        ),
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
        lexer(
            String::from("3+(1+2)"),
            get_rt(),
            &get_symbol,
            &sym_to_expect
        ),
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
        lexer(
            String::from("(1+2)+(2*(3)+(5-6))"),
            get_rt(),
            &get_symbol,
            &sym_to_expect
        ),
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
        lexer(String::from("(1"), get_rt(), &get_symbol, &sym_to_expect),
        Err(String::from("Error: Incomplete syntax"))
    );
}

#[test]
fn error_empty_brackets() {
    assert_eq!(
        lexer(String::from("()"), get_rt(), &get_symbol, &sym_to_expect),
        Err(String::from("Error: Invalid syntax"))
    );
}

#[test]
fn error_no_2nd_operand() {
    assert_eq!(
        lexer(String::from("1+"), get_rt(), &get_symbol, &sym_to_expect),
        Err(String::from("Error: Incomplete syntax"))
    );
}

#[test]
fn error_too_large_number() {
    assert_eq!(
        lexer(
            String::from("12345678901234567890"),
            get_rt(),
            &get_symbol,
            &sym_to_expect
        ),
        Err(String::from("Error: Too large number"))
    );
}
