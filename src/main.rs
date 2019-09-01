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

fn get_symbol_nbr(mut c: char, s: &mut String) -> Symbol {
    let mut nb = 0;

    loop {
        nb = nb * 10 + c.to_digit(10).unwrap();

        match s.chars().next() {
            None => break,
            Some(first) => {
                if !first.is_digit(10) {
                    break;
                }
            }
        }

        c = s.remove(0);
    }

    return Symbol::TsNbr(nb);
}

fn get_symbol(s: &mut String) -> Symbol {
    if s.len() == 0 {
        return Symbol::TsEos;
    }

    let c = s.remove(0);

    return match c {
        '(' => Symbol::TsLParens,
        ')' => Symbol::TsRParens,
        '+' => Symbol::TsPlus,
        '-' => Symbol::TsLess,
        '*' => Symbol::TsTimes,
        '0'...'9' => get_symbol_nbr(c, s),
        _ => Symbol::TsInvalid,
    };
}

struct Rule {
    pub sym1: ExpectSym,
    pub sym2: ExpectSym,
    pub res: Vec<ExpectSym>,
}

impl Rule {
    fn new(sym1: ExpectSym, sym2: ExpectSym, res: Vec<ExpectSym>) -> Rule {
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
    table: HashMap<(ExpectSym, ExpectSym), Vec<ExpectSym>>,
}

impl RuleTable {
    fn new(rules: Vec<Rule>) -> RuleTable {
        let mut table: HashMap<(ExpectSym, ExpectSym), Vec<ExpectSym>> = HashMap::new();

        for rule in rules {
            table.insert((rule.sym1, rule.sym2), rule.res);
        }

        return RuleTable { table };
    }

    fn get_res(self: &RuleTable, sym1: ExpectSym, sym2: ExpectSym) -> &Vec<ExpectSym> {
        return &self.table[&(sym1, sym2)];
    }
}

fn get_rt() -> RuleTable {
    return RuleTable::new(vec![
        Rule::new(ExpectSym::NtsExpr, ExpectSym::TsNbr, vec![ExpectSym::TsNbr]),
        Rule::new(
            ExpectSym::NtsExpr,
            ExpectSym::TsLParens,
            vec![
                ExpectSym::TsRParens,
                ExpectSym::NtsExpr,
                ExpectSym::NtsSign,
                ExpectSym::NtsExpr,
                ExpectSym::TsLParens,
            ],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsPlus,
            vec![ExpectSym::TsPlus],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsLess,
            vec![ExpectSym::TsLess],
        ),
        Rule::new(
            ExpectSym::NtsSign,
            ExpectSym::TsTimes,
            vec![ExpectSym::TsTimes],
        ),
    ]);
}

fn lexer(mut s: String, rt: RuleTable) -> Vec<Symbol> {
    let mut syms: Vec<Symbol> = vec![];
    let mut sym_stack: Vec<ExpectSym> = vec![ExpectSym::NtsExpr];

    while s.len() != 0 {
        let sym = get_symbol(&mut s);
        let expect = sym_to_expect(&sym);
        let top = *sym_stack.last().unwrap();

        if expect == *sym_stack.last().unwrap() {
            sym_stack.pop();
        } else {
            let res_syms = rt.get_res(top, expect);

            sym_stack.pop();

            for res in res_syms {
                sym_stack.push(*res);
            }

            sym_stack.pop();
        }
        syms.push(sym);
    }

    return syms;
}

fn main() {
    println!("{:?}", lexer(get_arg(), get_rt()));
}
