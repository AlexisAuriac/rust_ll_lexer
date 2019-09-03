use ll_lexer::rule::{Rule, RuleTable};

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

fn get_symbol_nbr(s: &String) -> Result<(LexSym, GramSym, usize), String> {
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

    return Ok((LexSym::TsNbr(nb), GramSym::TsNbr, size));
}

pub fn get_symbol(s: &String) -> Result<(LexSym, GramSym, usize), String> {
    if s.len() == 0 {
        return Ok((LexSym::TsEos, GramSym::TsEos, 0));
    }

    let c = s.chars().next().unwrap();

    return match c {
        '(' => Ok((LexSym::TsLBracket, GramSym::TsLBracket, 1)),
        ')' => Ok((LexSym::TsRBracket, GramSym::TsRBracket, 1)),
        '+' => Ok((LexSym::TsPlus, GramSym::TsPlus, 1)),
        '-' => Ok((LexSym::TsLess, GramSym::TsLess, 1)),
        '*' => Ok((LexSym::TsTimes, GramSym::TsTimes, 1)),
        '/' => Ok((LexSym::TsDivide, GramSym::TsDivide, 1)),
        '%' => Ok((LexSym::TsModulo, GramSym::TsModulo, 1)),
        '0'...'9' => get_symbol_nbr(s),
        _ => Ok((LexSym::TsInvalid, GramSym::TsInvalid, 1)),
    };
}

pub fn get_rt() -> RuleTable<GramSym> {
    return RuleTable::new(
        vec![(GramSym::NtsExpr, false)],
        GramSym::TsEos,
        vec![
            Rule::new(
                GramSym::NtsExpr,
                GramSym::TsNbr,
                vec![(GramSym::NtsSign, true)],
            ),
            Rule::new(
                GramSym::NtsExpr,
                GramSym::TsLBracket,
                vec![
                    (GramSym::NtsExpr, false),
                    (GramSym::TsRBracket, false),
                    (GramSym::NtsSign, true),
                ],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsPlus,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsLess,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsTimes,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsDivide,
                vec![(GramSym::NtsExpr, false)],
            ),
            Rule::new(
                GramSym::NtsSign,
                GramSym::TsModulo,
                vec![(GramSym::NtsExpr, false)],
            ),
        ],
    );
}
