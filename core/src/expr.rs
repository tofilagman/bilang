//! Arithmetic expressions solved one step at a time, the way a teacher writes
//! them on the board: each step names the rule (parentheses, exponents,
//! × ÷ before + −, left to right) and shows what the expression becomes.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tok {
    Num(i64),
    /// One of + − × ÷ ^
    Op(char),
    L,
    R,
}

pub struct Step {
    pub rule: String,
    /// The rule alone ("Multiply first (× and ÷ go before + and −)"), for the tutorial.
    pub why: String,
    /// The calculation as a question ("2 × 3", or "5³ (5 × 5 × 5)").
    pub ask: String,
    pub before: String,
    pub after: String,
    /// The number this step produced, so generators can avoid negatives for young kids.
    pub value: i64,
}

pub fn sup(n: i64) -> String {
    const D: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let mut s: String = n.abs().to_string().chars().map(|c| D[c.to_digit(10).unwrap() as usize]).collect();
    if n < 0 {
        s.insert(0, '⁻');
    }
    s
}

/// Superscript form of a short exponent like "2", "-3" or "n", if every
/// character has one.
pub fn superscript(s: &str) -> Option<String> {
    const D: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    s.chars()
        .map(|c| match c {
            '0'..='9' => Some(D[c.to_digit(10)? as usize]),
            '-' | '−' => Some('⁻'),
            '+' => Some('⁺'),
            'n' => Some('ⁿ'),
            'x' => Some('ˣ'),
            _ => None,
        })
        .collect()
}

/// A number as it sits inside an expression: negatives get parentheses
/// unless they lead the expression or a group.
fn num(n: i64, wrap: bool) -> String {
    match (n < 0, wrap) {
        (true, true) => format!("(−{})", -n),
        (true, false) => format!("−{}", -n),
        _ => n.to_string(),
    }
}

pub fn show(toks: &[Tok]) -> String {
    let mut out = String::new();
    for (i, t) in toks.iter().enumerate() {
        let prev = i.checked_sub(1).map(|j| toks[j]);
        match *t {
            Tok::Num(n) => {
                if prev == Some(Tok::Op('^')) {
                    out.push_str(&sup(n));
                } else {
                    // −3² would read as −(3²), so a negative base always gets parentheses.
                    let base = toks.get(i + 1) == Some(&Tok::Op('^'));
                    out.push_str(&num(n, base || !matches!(prev, None | Some(Tok::L))));
                }
            }
            Tok::Op('^') => {}
            Tok::Op(c) => {
                out.push(' ');
                out.push(c);
                out.push(' ');
            }
            Tok::L => out.push('('),
            Tok::R => out.push(')'),
        }
    }
    out
}

fn apply(a: i64, op: char, b: i64) -> Option<i64> {
    match op {
        '+' => a.checked_add(b),
        '−' => a.checked_sub(b),
        '×' => a.checked_mul(b),
        '÷' => (b != 0 && a % b == 0).then(|| a / b),
        '^' => (0..=6).contains(&b).then(|| a.checked_pow(b as u32)).flatten(),
        _ => None,
    }
}

/// Drop parentheses that hold just one number: "(7)" → "7".
fn unwrap_single(toks: &mut Vec<Tok>) {
    let mut i = 0;
    while i + 2 < toks.len() {
        if toks[i] == Tok::L && matches!(toks[i + 1], Tok::Num(_)) && toks[i + 2] == Tok::R {
            toks.remove(i + 2);
            toks.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Solve fully. None when a division isn't exact or a number gets out of hand,
/// so generators can just try again.
pub fn solve(toks: &[Tok]) -> Option<(i64, Vec<Step>)> {
    let mut toks = toks.to_vec();
    unwrap_single(&mut toks);
    let mut steps = Vec::new();
    loop {
        if let [Tok::Num(n)] = toks[..] {
            return Some((n, steps));
        }
        // The innermost group: an "(" whose next bracket is ")".
        let mut range = (0, toks.len());
        let mut in_parens = false;
        let mut open = None;
        for (i, t) in toks.iter().enumerate() {
            match t {
                Tok::L => open = Some(i),
                Tok::R => {
                    range = (open? + 1, i);
                    in_parens = true;
                    break;
                }
                _ => {}
            }
        }
        let ops: Vec<(usize, char)> =
            (range.0..range.1).filter_map(|i| if let Tok::Op(c) = toks[i] { Some((i, c)) } else { None }).collect();
        let pick = |set: &[char]| ops.iter().find(|(_, c)| set.contains(c)).copied();
        let (i, op) = pick(&['^']).or_else(|| pick(&['×', '÷'])).or_else(|| pick(&['+', '−']))?;
        let (Tok::Num(a), Tok::Num(b)) = (toks[i - 1], toks[i + 1]) else { return None };
        let v = apply(a, op, b)?;
        if v.abs() > 100_000 {
            return None;
        }
        let has = |set: &[char]| ops.iter().any(|(_, c)| set.contains(c));
        let (ask, calc) = if op == '^' {
            let times = vec![num(a, true); b as usize].join(" × ");
            (format!("{}{} ({times})", num(a, true), sup(b)), format!("{}{} = {} = {}", num(a, true), sup(b), times, num(v, false)))
        } else {
            let q = format!("{} {} {}", num(a, false), op, num(b, true));
            (q.clone(), format!("{q} = {}", num(v, false)))
        };
        let mut why = match op {
            '^' => "Exponents come first".to_string(),
            '×' | '÷' if has(&['+', '−']) => format!("{} first (× and ÷ go before + and −)", if op == '×' { "Multiply" } else { "Divide" }),
            '×' => "Multiply".to_string(),
            '÷' => "Divide".to_string(),
            _ if ops.len() > 1 => format!("{}, left to right", if op == '+' { "Add" } else { "Subtract" }),
            '+' => "Add".to_string(),
            _ => "Subtract".to_string(),
        };
        if in_parens {
            why = format!("Inside the parentheses first. {why}");
        }
        let rule = format!("{why}: {calc}");
        let before = show(&toks);
        toks.splice(i - 1..=i + 1, [Tok::Num(v)]);
        unwrap_single(&mut toks);
        steps.push(Step { rule, why, ask, before, after: show(&toks), value: v });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tok::*;

    #[test]
    fn mdas_order() {
        // 5 + 3 × 4 − 2 = 15
        let e = [Num(5), Op('+'), Num(3), Op('×'), Num(4), Op('−'), Num(2)];
        let (v, steps) = solve(&e).unwrap();
        assert_eq!(v, 15);
        assert_eq!(steps.len(), 3);
        assert!(steps[0].rule.starts_with("Multiply first"));
        assert_eq!(steps[0].after, "5 + 12 − 2");
    }

    #[test]
    fn parentheses_and_exponents() {
        // (2 + 3)^2 − 4 ÷ 2 = 23
        let e = [L, Num(2), Op('+'), Num(3), R, Op('^'), Num(2), Op('−'), Num(4), Op('÷'), Num(2)];
        assert_eq!(show(&e), "(2 + 3)² − 4 ÷ 2");
        let (v, steps) = solve(&e).unwrap();
        assert_eq!(v, 23);
        assert!(steps[0].rule.starts_with("Inside the parentheses"));
        assert_eq!(steps[0].after, "5² − 4 ÷ 2");
    }

    #[test]
    fn inexact_division_is_rejected() {
        assert!(solve(&[Num(7), Op('÷'), Num(2)]).is_none());
        assert_eq!(show(&[Num(-3), Op('×'), Num(-2)]), "−3 × (−2)");
    }
}
