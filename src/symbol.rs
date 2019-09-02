#[derive(Debug, PartialEq)]
pub enum Symbol {
    TsLBracket,
    TsRBracket,
    TsPlus,
    TsLess,
    TsTimes,
    TsDivide,
    TsModulo,
    TsNbr(u32),
    TsEos,
    TsInvalid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ExpectSym {
    TsLBracket,
    TsRBracket,
    TsPlus,
    TsLess,
    TsTimes,
    TsDivide,
    TsModulo,
    TsNbr,
    TsEos,
    TsInvalid,
    NtsExpr,
    NtsSign,
}

fn get_symbol_nbr(s: &mut String) -> Result<(Symbol, usize), String> {
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

    return Ok((Symbol::TsNbr(nb), size));
}

pub fn get_symbol(s: &mut String) -> Result<(Symbol, usize), String> {
    if s.len() == 0 {
        return Ok((Symbol::TsEos, 0));
    }

    let c = s.chars().next().unwrap();

    return match c {
        '(' => Ok((Symbol::TsLBracket, 1)),
        ')' => Ok((Symbol::TsRBracket, 1)),
        '+' => Ok((Symbol::TsPlus, 1)),
        '-' => Ok((Symbol::TsLess, 1)),
        '*' => Ok((Symbol::TsTimes, 1)),
        '/' => Ok((Symbol::TsDivide, 1)),
        '%' => Ok((Symbol::TsModulo, 1)),
        '0'...'9' => get_symbol_nbr(s),
        _ => Ok((Symbol::TsInvalid, 1)),
    };
}

pub fn sym_to_expect(sym: &Symbol) -> ExpectSym {
    return match sym {
        Symbol::TsLBracket => ExpectSym::TsLBracket,
        Symbol::TsRBracket => ExpectSym::TsRBracket,
        Symbol::TsPlus => ExpectSym::TsPlus,
        Symbol::TsLess => ExpectSym::TsLess,
        Symbol::TsTimes => ExpectSym::TsTimes,
        Symbol::TsDivide => ExpectSym::TsDivide,
        Symbol::TsModulo => ExpectSym::TsModulo,
        Symbol::TsNbr { .. } => ExpectSym::TsNbr,
        Symbol::TsEos => ExpectSym::TsEos,
        Symbol::TsInvalid => ExpectSym::TsInvalid,
    };
}
