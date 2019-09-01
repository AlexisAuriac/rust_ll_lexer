use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Symbol {
    TsLParens,
    TsRParens,
    TsPlus,
    TsLess,
    TsTimes,
    TsNbr(u32),
    TsEos,
    TsInvalid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ExpectSym {
    TsLParens,
    TsRParens,
    TsPlus,
    TsLess,
    TsTimes,
    TsNbr,
    TsEos,
    TsInvalid,
    NtsExpr,
    NtsSign,
}

fn get_symbol_nbr(s: &mut String) -> (Symbol, usize) {
    let mut nb = 0;
    let mut size = 0;

    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }

        nb = nb * 10 + c.to_digit(10).unwrap();
        size += 1;
    }

    return (Symbol::TsNbr(nb), size);
}

fn get_symbol(s: &mut String) -> (Symbol, usize) {
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
        '0'...'9' => get_symbol_nbr(s),
        _ => (Symbol::TsInvalid, 1),
    };
}

struct Rule {
    pub sym1: ExpectSym,
    pub sym2: ExpectSym,
    pub res: Vec<(ExpectSym, bool)>,
}

impl Rule {
    fn new(sym1: ExpectSym, sym2: ExpectSym, res: Vec<(ExpectSym, bool)>) -> Rule {
        Rule { sym1, sym2, res }
    }
}

fn get_arg() -> String {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("usage:\texpr");
        std::process::exit(1);
    }

    return args.remove(1);
}

fn sym_to_expect(sym: &Symbol) -> ExpectSym {
    return match sym {
        Symbol::TsLParens => ExpectSym::TsLParens,
        Symbol::TsRParens => ExpectSym::TsRParens,
        Symbol::TsPlus => ExpectSym::TsPlus,
        Symbol::TsLess => ExpectSym::TsLess,
        Symbol::TsTimes => ExpectSym::TsTimes,
        Symbol::TsNbr { .. } => ExpectSym::TsNbr,
        Symbol::TsEos => ExpectSym::TsEos,
        Symbol::TsInvalid => ExpectSym::TsInvalid,
    };
}

struct RuleTable {
    table: HashMap<(ExpectSym, ExpectSym), Vec<(ExpectSym, bool)>>,
}

impl RuleTable {
    fn new(rules: Vec<Rule>) -> RuleTable {
        let mut table: HashMap<(ExpectSym, ExpectSym), Vec<(ExpectSym, bool)>> = HashMap::new();

        for rule in rules {
            table.insert((rule.sym1, rule.sym2), rule.res);
        }

        return RuleTable { table };
    }

    fn get_res(
        self: &RuleTable,
        sym1: ExpectSym,
        sym2: ExpectSym,
    ) -> Option<&Vec<(ExpectSym, bool)>> {
        return self.table.get(&(sym1, sym2));
    }
}

fn get_rt() -> RuleTable {
    return RuleTable::new(vec![
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsNbr,
            vec![(ExpectSym::NtsSign, true), (ExpectSym::TsNbr, false)],
        ),
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsLParens,
            vec![
                (ExpectSym::NtsSign, true),
                (ExpectSym::TsRParens, false),
                (ExpectSym::NtsExpr, false),
                (ExpectSym::TsLParens, false),
            ],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsPlus,
            vec![(ExpectSym::NtsExpr, false), (ExpectSym::TsPlus, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsLess,
            vec![(ExpectSym::NtsExpr, false), (ExpectSym::TsLess, false)],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsTimes,
            vec![(ExpectSym::NtsExpr, false), (ExpectSym::TsTimes, false)],
        ),
    ]);
}

fn lexer(mut s: String, rt: RuleTable) -> Vec<Symbol> {
    let mut syms: Vec<Symbol> = vec![];
    let mut sym_stack: Vec<(ExpectSym, bool)> = vec![(ExpectSym::NtsExpr, false)];

    while s.len() != 0 {
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

                    for res in res_syms {
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
                        eprintln!("error");
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
