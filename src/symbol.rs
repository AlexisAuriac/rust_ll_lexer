#[derive(Debug, PartialEq)]
pub enum LexSym {
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
pub enum GramSym {
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

pub fn get_symbol(s: &mut String) -> Result<(LexSym, usize), String> {
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
