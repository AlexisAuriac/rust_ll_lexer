mod rule;
mod symbol;

use rule::{Rule, RuleTable};
use symbol::{get_symbol, sym_to_expect, ExpectSym, Symbol};

fn get_arg() -> String {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("usage:\texpr");
        std::process::exit(1);
    }

    return args.remove(1);
}

fn get_rt() -> RuleTable {
    return RuleTable::new(vec![
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsNbr,
            vec![(ExpectSym::TsNbr, false), (ExpectSym::NtsSign, true)],
        ),
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsLParens,
            vec![
                (ExpectSym::TsLParens, false),
                (ExpectSym::NtsExpr, false),
                (ExpectSym::TsRParens, false),
                (ExpectSym::NtsSign, true),
            ],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsPlus,
            vec![(ExpectSym::TsPlus, false), (ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsLess,
            vec![(ExpectSym::TsLess, false), (ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsTimes,
            vec![(ExpectSym::TsTimes, false), (ExpectSym::NtsExpr, false)],
        ),
    ]);
}

fn lexer(mut s: String, rt: RuleTable) -> Vec<Symbol> {
    let mut syms: Vec<Symbol> = vec![];
    let mut sym_stack: Vec<(ExpectSym, bool)> = vec![(ExpectSym::NtsExpr, false)];

    while sym_stack.len() != 0 {
        let (sym, size) = get_symbol(&mut s);
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

                    sym_stack.pop();
                    s.replace_range(..size, "");
                    syms.push(sym);
                }
                None => {
                    if opt {
                        sym_stack.pop();
                    } else {
                        if expect == ExpectSym::TsEos {
                            eprintln!("Error: Incomplete syntax");
                        } else {
                            eprintln!("Error: Invalid syntax");
                        }
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    return syms;
}

fn main() {
    println!("{:?}", lexer(get_arg(), get_rt()));
}
