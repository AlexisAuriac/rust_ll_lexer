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

fn get_nb_spaces(s: &str) -> usize {
    let mut nb_spaces = 0;

    for c in s.chars() {
        if !c.is_whitespace() {
            break;
        }

        nb_spaces += 1;
    }

    return nb_spaces;
}

fn get_sub_or_add_symbol(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    let mut addition = true;
    let mut size = 0;

    for c in s.chars() {
        match c {
            '+' => {}
            '-' => addition = !addition,
            _ => break,
        };

        size += 1;
    }

    if addition {
        return Ok((LexSym::TsPlus, GramSym::TsPlus, size));
    } else {
        return Ok((LexSym::TsLess, GramSym::TsLess, size));
    }
}

fn get_symbol_nbr(s: &str) -> Result<(LexSym, GramSym, usize), String> {
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

pub fn get_symbol(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    if s.len() == 0 {
        return Ok((LexSym::TsEos, GramSym::TsEos, 0));
    }

    let nb_spaces = get_nb_spaces(s);
    let c = s.chars().nth(nb_spaces).unwrap();

    let (lex, gram, size) = match c {
        '(' => (LexSym::TsLBracket, GramSym::TsLBracket, 1),
        ')' => (LexSym::TsRBracket, GramSym::TsRBracket, 1),
        '+' | '-' => get_sub_or_add_symbol(&s[nb_spaces..])?,
        '*' => (LexSym::TsTimes, GramSym::TsTimes, 1),
        '/' => (LexSym::TsDivide, GramSym::TsDivide, 1),
        '%' => (LexSym::TsModulo, GramSym::TsModulo, 1),
        '0'...'9' => get_symbol_nbr(&s[nb_spaces..])?,
        _ => (LexSym::TsInvalid, GramSym::TsInvalid, 1),
    };

    return Ok((lex, gram, size + nb_spaces));
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
