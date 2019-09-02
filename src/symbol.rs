#[derive(Debug, PartialEq)]
pub enum Symbol {
    TsLParens,
    TsRParens,
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
    TsLParens,
    TsRParens,
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

fn get_symbol_nbr(s: &mut String) -> (Symbol, usize) {
    let mut nb: u32 = 0;
    let mut size = 0;

    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }

        if let (nb2, false) = nb.overflowing_mul(10) {
            nb = nb2;
        } else {
            panic!("Error: Too large number");
        }

        if let (nb2, false) = nb.overflowing_add(c.to_digit(10).unwrap()) {
            nb = nb2;
        } else {
            panic!("Error: Too large number");
        }

        size += 1;
    }

    return (Symbol::TsNbr(nb), size);
}

pub fn get_symbol(s: &mut String) -> (Symbol, usize) {
    if s.len() == 0 {
        return (Symbol::TsEos, 0);
    }

    let c = s.chars().next().unwrap();

    return match c {
        '(' => (Symbol::TsLParens, 1),
        ')' => (Symbol::TsRParens, 1),
        '+' => (Symbol::TsPlus, 1),
        '-' => (Symbol::TsLess, 1),
        '*' => (Symbol::TsTimes, 1),
        '/' => (Symbol::TsDivide, 1),
        '%' => (Symbol::TsModulo, 1),
        '0'...'9' => get_symbol_nbr(s),
        _ => (Symbol::TsInvalid, 1),
    };
}

pub fn sym_to_expect(sym: &Symbol) -> ExpectSym {
    return match sym {
        Symbol::TsLParens => ExpectSym::TsLParens,
        Symbol::TsRParens => ExpectSym::TsRParens,
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
