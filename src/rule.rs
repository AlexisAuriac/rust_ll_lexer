use std::collections::HashMap;

use crate::symbol::ExpectSym;

pub struct Rule {
    pub sym1: ExpectSym,
    pub sym2: ExpectSym,
    pub res: Vec<(ExpectSym, bool)>,
}

impl Rule {
    pub fn new(sym1: ExpectSym, sym2: ExpectSym, res: Vec<(ExpectSym, bool)>) -> Rule {
        Rule { sym1, sym2, res }
    }
}

pub struct RuleTable {
    table: HashMap<(ExpectSym, ExpectSym), Vec<(ExpectSym, bool)>>,
}

impl RuleTable {
    pub fn new(rules: Vec<Rule>) -> RuleTable {
        let mut table: HashMap<(ExpectSym, ExpectSym), Vec<(ExpectSym, bool)>> = HashMap::new();

        for rule in rules {
            table.insert((rule.sym1, rule.sym2), rule.res);
        }

        return RuleTable { table };
    }

    pub fn get_res(
        self: &RuleTable,
        sym1: ExpectSym,
        sym2: ExpectSym,
    ) -> Option<&Vec<(ExpectSym, bool)>> {
        return self.table.get(&(sym1, sym2));
    }
}
