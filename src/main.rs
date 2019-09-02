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
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsDivide,
            vec![(ExpectSym::TsDivide, false), (ExpectSym::NtsExpr, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsModulo,
            vec![(ExpectSym::TsModulo, false), (ExpectSym::NtsExpr, false)],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_value() {
        assert_eq!(lexer(String::from("1"), get_rt()), vec![Symbol::TsNbr(1)]);
    }

    #[test]
    fn multi_digit_nbr() {
        assert_eq!(lexer(String::from("12"), get_rt()), vec![Symbol::TsNbr(12)]);
    }

    #[test]
    fn simple_operation() {
        assert_eq!(
            lexer(String::from("12+3"), get_rt()),
            vec![Symbol::TsNbr(12), Symbol::TsPlus, Symbol::TsNbr(3)]
        );
        assert_eq!(
            lexer(String::from("12-1234"), get_rt()),
            vec![Symbol::TsNbr(12), Symbol::TsLess, Symbol::TsNbr(1234)]
        );
        assert_eq!(
            lexer(String::from("4*32"), get_rt()),
            vec![Symbol::TsNbr(4), Symbol::TsTimes, Symbol::TsNbr(32)]
        );
        assert_eq!(
            lexer(String::from("12/4"), get_rt()),
            vec![Symbol::TsNbr(12), Symbol::TsDivide, Symbol::TsNbr(4)]
        );
        assert_eq!(
            lexer(String::from("12%4"), get_rt()),
            vec![Symbol::TsNbr(12), Symbol::TsModulo, Symbol::TsNbr(4)]
        );
    }

    #[test]
    fn multiple_operations() {
        assert_eq!(
            lexer(String::from("10+15-10*13+6/3%4"), get_rt()),
            vec![
                Symbol::TsNbr(10),
                Symbol::TsPlus,
                Symbol::TsNbr(15),
                Symbol::TsLess,
                Symbol::TsNbr(10),
                Symbol::TsTimes,
                Symbol::TsNbr(13),
                Symbol::TsPlus,
                Symbol::TsNbr(6),
                Symbol::TsDivide,
                Symbol::TsNbr(3),
                Symbol::TsModulo,
                Symbol::TsNbr(4),
            ]
        );
    }

    #[test]
    fn simple_brackets() {
        assert_eq!(
            lexer(String::from("(1)"), get_rt()),
            vec![Symbol::TsLParens, Symbol::TsNbr(1), Symbol::TsRParens,]
        );
        assert_eq!(
            lexer(String::from("(1+2)"), get_rt()),
            vec![
                Symbol::TsLParens,
                Symbol::TsNbr(1),
                Symbol::TsPlus,
                Symbol::TsNbr(2),
                Symbol::TsRParens,
            ]
        );
        assert_eq!(
            lexer(String::from("(1+2)+3"), get_rt()),
            vec![
                Symbol::TsLParens,
                Symbol::TsNbr(1),
                Symbol::TsPlus,
                Symbol::TsNbr(2),
                Symbol::TsRParens,
                Symbol::TsPlus,
                Symbol::TsNbr(3),
            ]
        );
        assert_eq!(
            lexer(String::from("3+(1+2)"), get_rt()),
            vec![
                Symbol::TsNbr(3),
                Symbol::TsPlus,
                Symbol::TsLParens,
                Symbol::TsNbr(1),
                Symbol::TsPlus,
                Symbol::TsNbr(2),
                Symbol::TsRParens,
            ]
        );
    }

    #[test]
    fn complicated_brackets() {
        assert_eq!(
            lexer(String::from("(1+2)+(2*(3)+(5-6))"), get_rt()),
            vec![
                Symbol::TsLParens,
                Symbol::TsNbr(1),
                Symbol::TsPlus,
                Symbol::TsNbr(2),
                Symbol::TsRParens,
                Symbol::TsPlus,
                Symbol::TsLParens,
                Symbol::TsNbr(2),
                Symbol::TsTimes,
                Symbol::TsLParens,
                Symbol::TsNbr(3),
                Symbol::TsRParens,
                Symbol::TsPlus,
                Symbol::TsLParens,
                Symbol::TsNbr(5),
                Symbol::TsLess,
                Symbol::TsNbr(6),
                Symbol::TsRParens,
                Symbol::TsRParens,
            ]
        );
    }
}
