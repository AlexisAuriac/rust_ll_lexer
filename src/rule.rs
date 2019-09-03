use std::collections::HashMap;

pub struct Rule<GS>
where
    GS: Eq + std::hash::Hash,
{
    pub genre: GS,
    pub version: GS,
    pub next: Vec<(GS, bool)>,
}

impl<GS> Rule<GS>
where
    GS: Eq + std::hash::Hash,
{
    pub fn new(genre: GS, version: GS, next: Vec<(GS, bool)>) -> Rule<GS> {
        Rule {
            genre,
            version,
            next,
        }
    }
}

pub struct RuleTable<GS>
where
    GS: Eq + std::hash::Hash,
{
    pub start: Vec<(GS, bool)>,
    pub end: GS,
    table: HashMap<(GS, GS), Vec<(GS, bool)>>,
}

impl<GS> RuleTable<GS>
where
    GS: Eq + std::hash::Hash,
{
    pub fn new(start: Vec<(GS, bool)>, end: GS, rules: Vec<Rule<GS>>) -> RuleTable<GS> {
        let mut table: HashMap<(GS, GS), Vec<(GS, bool)>> = HashMap::new();

        for rule in rules {
            table.insert((rule.genre, rule.version), rule.next);
        }

        return RuleTable { start, end, table };
    }

    pub fn get_res(self: &RuleTable<GS>, genre: GS, version: GS) -> Option<&Vec<(GS, bool)>> {
        return self.table.get(&(genre, version));
    }
}
