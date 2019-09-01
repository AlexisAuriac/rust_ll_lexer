#[derive(Debug)]
enum Symbol {
    TsLParens,
    TsRParens,
    TsPlus,
    TsLess,
    TsTimes,
    TsA,
    TsNbr(u32),
    TsEos,
    TsInvalid,
    NtsExpr,
    NtsValue,
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
        'a' => Symbol::TsA,
        '0'...'9' => get_symbol_nbr(c, s),
        _ => Symbol::TsInvalid,
    };
}

fn main() {
    let mut s = String::from("56");

    println!("{:?}", get_symbol(&mut s));
    println!("{:?}", s);
}
