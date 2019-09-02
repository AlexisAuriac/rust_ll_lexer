pub mod rule;
pub mod symbol;

use rule::{Rule, RuleTable};
use symbol::{get_symbol, sym_to_expect, ExpectSym, Symbol};

pub fn get_rt() -> RuleTable {
    return RuleTable::new(vec![
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsNbr,
            vec![(ExpectSym::NtsSign, true)],
        ),
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsLBracket,
            vec![
                (ExpectSym::NtsExpr, false),
                (ExpectSym::TsRBracket, false),
                (ExpectSym::NtsSign, true),
            ],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsPlus,
            vec![(ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsLess,
            vec![(ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsTimes,
            vec![(ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsDivide,
            vec![(ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsModulo,
            vec![(ExpectSym::NtsExpr, false)],
        ),
    ]);
}

pub fn lexer(mut s: String, rt: RuleTable) -> Result<Vec<Symbol>, String> {
    let mut syms: Vec<Symbol> = vec![];
    let mut sym_stack: Vec<(ExpectSym, bool)> = vec![(ExpectSym::NtsExpr, false)];

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
                        if expect == ExpectSym::TsEos {
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
