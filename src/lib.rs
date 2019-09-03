pub mod rule;
pub mod symbol;

use rule::{Rule, RuleTable};
use symbol::{get_symbol, sym_to_expect, GramSym, LexSym};

pub fn get_rt() -> RuleTable {
    return RuleTable::new(vec![
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
    ]);
}

pub fn lexer(mut s: String, rt: RuleTable) -> Result<Vec<LexSym>, String> {
    let mut syms: Vec<LexSym> = vec![];
    let mut sym_stack: Vec<(GramSym, bool)> = vec![(GramSym::NtsExpr, false)];

    while sym_stack.len() != 0 {
        let (sym, size) = get_symbol(&mut s)?;
        let expect = sym_to_expect(&sym);
        let (top, opt) = *sym_stack.last().unwrap();

        if expect == top {
            sym_stack.pop();
            s.replace_range(..size, "");
            syms.push(sym);
        } else {
            match rt.get_res(top, expect) {
                Some(res_syms) => {
                    sym_stack.pop();

                    for res in res_syms.iter().rev() {
                        sym_stack.push(*res);
                    }

                    s.replace_range(..size, "");
                    syms.push(sym);
                }
                None => {
                    if opt {
                        sym_stack.pop();
                    } else {
                        if expect == GramSym::TsEos {
                            return Err("Error: Incomplete syntax".to_string());
                        } else {
                            return Err("Error: Invalid syntax".to_string());
                        }
                    }
                }
            }
        }
    }

    return Ok(syms);
}
