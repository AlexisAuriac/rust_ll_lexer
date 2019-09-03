use std::collections::HashMap;

use crate::symbol::GramSym;

pub struct Rule {
    pub genre: GramSym,
    pub version: GramSym,
    pub next: Vec<(GramSym, bool)>,
}

impl Rule {
    pub fn new(genre: GramSym, version: GramSym, next: Vec<(GramSym, bool)>) -> Rule {
        Rule {
            genre,
            version,
            next,
        }
    }
}

pub struct RuleTable {
    table: HashMap<(GramSym, GramSym), Vec<(GramSym, bool)>>,
}

impl RuleTable {
    pub fn new(rules: Vec<Rule>) -> RuleTable {
        let mut table: HashMap<(GramSym, GramSym), Vec<(GramSym, bool)>> = HashMap::new();

        for rule in rules {
            table.insert((rule.genre, rule.version), rule.next);
        }

        return RuleTable { table };
    }

    pub fn get_res(
        self: &RuleTable,
        genre: GramSym,
        version: GramSym,
    ) -> Option<&Vec<(GramSym, bool)>> {
        return self.table.get(&(genre, version));
    }
}
