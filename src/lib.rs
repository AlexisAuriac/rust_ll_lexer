pub mod rule;
pub mod symbol;

use rule::{Rule, RuleTable};
use symbol::GramSym;

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

pub fn lexer<GS, LS>(
    mut s: String,
    rt: RuleTable<GS>,
    get_sym: &Fn(&mut String) -> Result<(LS, GS, usize), String>,
) -> Result<Vec<LS>, String>
where
    GS: Eq + std::hash::Hash + Copy,
    LS: PartialEq,
{
    let mut syms: Vec<LS> = vec![];
    let mut sym_stack: Vec<(GS, bool)> = rt.start.clone();

    while sym_stack.len() != 0 {
        let (sym, gram, size) = get_sym(&mut s)?;
        let (top, opt) = *sym_stack.last().unwrap();

        if gram == top {
            sym_stack.pop();
            s.replace_range(..size, "");
            syms.push(sym);
        } else {
            match rt.get_res(top, gram) {
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
                        if gram == rt.end {
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
