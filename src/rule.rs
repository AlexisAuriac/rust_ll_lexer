use std::collections::HashMap;

use crate::symbol::ExpectSym;

pub struct Rule {
    pub genre: ExpectSym,
    pub version: ExpectSym,
    pub next: Vec<(ExpectSym, bool)>,
}

impl Rule {
    pub fn new(genre: ExpectSym, version: ExpectSym, next: Vec<(ExpectSym, bool)>) -> Rule {
        Rule {
            genre,
            version,
            next,
        }
    }
}

pub struct RuleTable {
    table: HashMap<(ExpectSym, ExpectSym), Vec<(ExpectSym, bool)>>,
}

impl RuleTable {
    pub fn new(rules: Vec<Rule>) -> RuleTable {
        let mut table: HashMap<(ExpectSym, ExpectSym), Vec<(ExpectSym, bool)>> = HashMap::new();

        for rule in rules {
            table.insert((rule.genre, rule.version), rule.next);
        }

        return RuleTable { table };
    }

    pub fn get_res(
        self: &RuleTable,
        genre: ExpectSym,
        version: ExpectSym,
    ) -> Option<&Vec<(ExpectSym, bool)>> {
        return self.table.get(&(genre, version));
    }
}
