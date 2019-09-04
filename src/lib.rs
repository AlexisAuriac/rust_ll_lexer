pub mod rule;

use rule::RuleTable;

pub fn lexer<GS, LS>(
    mut s: String,
    rt: RuleTable<GS>,
    get_sym: &Fn(&str) -> Result<(LS, GS, usize), String>,
) -> Result<Vec<LS>, String>
where
    GS: Eq + std::hash::Hash + Copy,
    LS: PartialEq,
{
    let mut syms: Vec<LS> = vec![];
    let mut sym_stack: Vec<(GS, bool)> = rt.start.clone();

    while sym_stack.len() != 0 {
        let (sym, gram, size) = get_sym(&s)?;
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
