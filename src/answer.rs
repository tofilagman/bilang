//! What a correct answer looks like, how a kid's typing is read, and the
//! gentle "almost" feedback when the value is right but the form is not.

#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    Int(i64),
    /// Always stored in simplest form with a positive denominator.
    Frac(i64, i64),
    /// Hundredths, so 2.75 is Dec(275). Kept as an integer to dodge float noise.
    Dec(i64),
    /// Quotient and remainder: "7 r 2".
    Remainder(i64, i64),
    /// Two solutions in any order (quadratics).
    Roots(i64, i64),
    /// An ordered pair (x, y) for systems of equations.
    Pair(i64, i64),
}

#[derive(Debug, PartialEq)]
pub enum Verdict {
    Right,
    /// Close: a nudge, and the attempt does not count as wrong.
    Almost(String),
    Wrong,
    /// Couldn't read it at all; tells the kid how to type it.
    Unreadable(String),
}

pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

/// n/d in simplest form with a positive denominator.
pub fn reduce(n: i64, d: i64) -> (i64, i64) {
    let g = gcd(n, d).max(1);
    let s = if d < 0 { -1 } else { 1 };
    (s * n / g, s * d / g)
}

impl Answer {
    pub fn frac(n: i64, d: i64) -> Self {
        let (n, d) = reduce(n, d);
        Answer::Frac(n, d)
    }

    /// One line telling the kid how to type this kind of answer.
    pub fn format_tip(&self) -> &'static str {
        match self {
            Answer::Int(_) => "Type a whole number.",
            Answer::Frac(..) => "Type a fraction like 3/4, in simplest form.",
            Answer::Dec(_) => "Type a decimal like 2.75",
            Answer::Remainder(..) => "Type the answer and remainder like 7 r 2",
            Answer::Roots(..) => "Type both answers like 2, -5",
            Answer::Pair(..) => "Type x then y like 3, -1",
        }
    }

    /// Which extra keys the on-screen keypad needs.
    pub fn keys(&self) -> &'static [&'static str] {
        match self {
            Answer::Int(_) => &["−"],
            Answer::Frac(..) => &["−", "/"],
            Answer::Dec(_) => &["−", "."],
            Answer::Remainder(..) => &["r"],
            Answer::Roots(..) | Answer::Pair(..) => &["−", ","],
        }
    }

    pub fn show(&self) -> String {
        match *self {
            Answer::Int(n) => minus(n),
            Answer::Frac(n, 1) => minus(n),
            Answer::Frac(n, d) => format!("{}/{}", minus(n), d),
            Answer::Dec(h) => show_dec(h),
            Answer::Remainder(q, 0) => q.to_string(),
            Answer::Remainder(q, r) => format!("{q} r {r}"),
            Answer::Roots(a, b) => format!("{}, {}", minus(a.min(b)), minus(a.max(b))),
            Answer::Pair(x, y) => format!("x = {}, y = {}", minus(x), minus(y)),
        }
    }

    pub fn check(&self, input: &str) -> Verdict {
        let s = normalize(input);
        if s.is_empty() {
            return Verdict::Unreadable("Type your answer first!".into());
        }
        let unreadable = || Verdict::Unreadable(format!("Hmm, I can't read that. {}", self.format_tip()));
        match *self {
            Answer::Int(n) => match parse_rational(&s) {
                Some((a, b)) if a == n * b => Verdict::Right,
                Some(_) => Verdict::Wrong,
                None => unreadable(),
            },
            Answer::Frac(n, d) => {
                let Some((a, b)) = parse_rational(&s) else { return unreadable() };
                if a * d != n * b {
                    return Verdict::Wrong;
                }
                if s.contains('.') && d != 1 {
                    return Verdict::Almost("That's the right amount! Now write it as a fraction.".into());
                }
                let (ra, rb) = written_fraction(&s).unwrap_or((a, b));
                if gcd(ra, rb) != 1 {
                    return Verdict::Almost("That's the right amount! Can you simplify it?".into());
                }
                Verdict::Right
            }
            Answer::Dec(h) => {
                let Ok(v) = s.parse::<f64>() else {
                    return match parse_rational(&s) {
                        Some((a, b)) if a * 100 == h * b => Verdict::Almost("Right amount! Now write it as a decimal.".into()),
                        Some(_) => Verdict::Wrong,
                        None => unreadable(),
                    };
                };
                if ((v * 100.0) - h as f64).abs() < 1e-6 { Verdict::Right } else { Verdict::Wrong }
            }
            Answer::Remainder(q, r) => {
                let s = s.replace("remainder", "r").replace('R', "r");
                let mut parts = s.split('r').map(str::trim);
                let Some(Ok(a)) = parts.next().map(str::parse::<i64>) else { return unreadable() };
                let b = match parts.next() {
                    None => 0,
                    Some(p) => match p.parse::<i64>() {
                        Ok(b) => b,
                        Err(_) => return unreadable(),
                    },
                };
                if (a, b) == (q, r) {
                    Verdict::Right
                } else if a == q {
                    Verdict::Almost(if r > 0 && b == 0 {
                        "The quotient is right! Is anything left over?".into()
                    } else {
                        "The quotient is right! Check the remainder.".into()
                    })
                } else {
                    Verdict::Wrong
                }
            }
            Answer::Roots(a, b) => {
                let Some((x, y)) = parse_two(&s) else { return unreadable() };
                if (x, y) == (a, b) || (x, y) == (b, a) {
                    Verdict::Right
                } else if x == a || x == b || y == a || y == b {
                    Verdict::Almost("One of them is right! Check the other one.".into())
                } else {
                    Verdict::Wrong
                }
            }
            Answer::Pair(a, b) => {
                let Some((x, y)) = parse_two(&s) else { return unreadable() };
                if (x, y) == (a, b) {
                    Verdict::Right
                } else if (x, y) == (b, a) {
                    Verdict::Almost("So close! Put x first, then y.".into())
                } else if x == a {
                    Verdict::Almost("x is right! Now check y.".into())
                } else if y == b {
                    Verdict::Almost("y is right! Now check x.".into())
                } else {
                    Verdict::Wrong
                }
            }
        }
    }
}

/// Negative numbers with a real minus sign.
pub fn minus(n: i64) -> String {
    if n < 0 { format!("−{}", -n) } else { n.to_string() }
}

pub fn show_dec(h: i64) -> String {
    let sign = if h < 0 { "−" } else { "" };
    let h = h.abs();
    match h % 100 {
        0 => format!("{sign}{}", h / 100),
        c if c % 10 == 0 => format!("{sign}{}.{}", h / 100, c / 10),
        c => format!("{sign}{}.{c:02}", h / 100),
    }
}

fn normalize(input: &str) -> String {
    let s = input.trim().replace(['−', '–', '—'], "-").replace('÷', "/").replace(' ', " ");
    // "x = 3, y = 4" → "3, 4"
    let s = s.replace("x=", "").replace("y=", "").replace("x =", "").replace("y =", "");
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The fraction exactly as written ("6/8" stays 6/8), for the simplest-form check.
fn written_fraction(s: &str) -> Option<(i64, i64)> {
    // For a mixed number only the fraction part needs to be simplified.
    let frac = s.rsplit_once(' ').map_or(s, |(_, f)| f);
    let (a, b) = frac.split_once('/')?;
    Some((a.trim().parse::<i64>().ok()?, b.trim().parse::<i64>().ok()?))
}

/// "3", "-3", "3/4", "1 1/2", "0.75" → exact numerator/denominator.
pub fn parse_rational(s: &str) -> Option<(i64, i64)> {
    let s = s.trim();
    if let Some((w, f)) = s.split_once(' ') {
        let w = w.parse::<i64>().ok()?;
        let (a, b) = parse_rational(f)?;
        if a < 0 || b <= 0 {
            return None;
        }
        let sign = if w < 0 { -1 } else { 1 };
        return Some((w * b + sign * a, b));
    }
    if let Some((a, b)) = s.split_once('/') {
        let (a, b) = (a.trim().parse::<i64>().ok()?, b.trim().parse::<i64>().ok()?);
        return (b != 0).then_some((a, b));
    }
    if let Ok(n) = s.parse::<i64>() {
        return Some((n, 1));
    }
    let (int, dec) = s.split_once('.')?;
    if dec.is_empty() || dec.len() > 6 || !dec.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let neg = int.starts_with('-');
    let int: i64 = if int == "-" || int.is_empty() { 0 } else { int.parse().ok()? };
    let den = 10i64.pow(dec.len() as u32);
    let frac: i64 = dec.parse().ok()?;
    let n = int.abs() * den + frac;
    Some((if neg { -n } else { n }, den))
}

fn parse_two(s: &str) -> Option<(i64, i64)> {
    let s = s.replace(" and ", ",").replace(';', ",");
    let parts: Vec<&str> = if s.contains(',') { s.split(',').collect() } else { s.split(' ').collect() };
    let parts: Vec<&str> = parts.into_iter().map(str::trim).filter(|p| !p.is_empty()).collect();
    if parts.len() != 2 {
        return None;
    }
    Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_numbers() {
        assert_eq!(Answer::Int(12).check("12"), Verdict::Right);
        assert_eq!(Answer::Int(-4).check("−4"), Verdict::Right);
        assert_eq!(Answer::Int(12).check("13"), Verdict::Wrong);
        assert!(matches!(Answer::Int(12).check("twelve"), Verdict::Unreadable(_)));
    }

    #[test]
    fn fractions_want_simplest_form() {
        let a = Answer::frac(6, 8);
        assert_eq!(a, Answer::Frac(3, 4));
        assert_eq!(a.check("3/4"), Verdict::Right);
        assert!(matches!(a.check("6/8"), Verdict::Almost(_)));
        assert!(matches!(a.check("0.75"), Verdict::Almost(_)));
        assert_eq!(a.check("2/3"), Verdict::Wrong);
        assert_eq!(Answer::frac(3, 2).check("1 1/2"), Verdict::Right);
        assert_eq!(Answer::frac(-1, 2).check("-1/2"), Verdict::Right);
        assert_eq!(Answer::frac(4, 2).check("2"), Verdict::Right);
    }

    #[test]
    fn decimals_and_remainders() {
        assert_eq!(Answer::Dec(275).check("2.75"), Verdict::Right);
        assert_eq!(Answer::Dec(250).check("2.5"), Verdict::Right);
        assert_eq!(Answer::Dec(30).check(".3"), Verdict::Right);
        assert_eq!(show_dec(305), "3.05");
        assert_eq!(Answer::Remainder(7, 2).check("7 r 2"), Verdict::Right);
        assert_eq!(Answer::Remainder(7, 2).check("7R2"), Verdict::Right);
        assert!(matches!(Answer::Remainder(7, 2).check("7"), Verdict::Almost(_)));
    }

    #[test]
    fn pairs_and_roots() {
        assert_eq!(Answer::Roots(2, -5).check("-5, 2"), Verdict::Right);
        assert!(matches!(Answer::Roots(2, -5).check("2, 5"), Verdict::Almost(_)));
        assert_eq!(Answer::Pair(3, -1).check("x = 3, y = −1"), Verdict::Right);
        assert!(matches!(Answer::Pair(3, -1).check("-1, 3"), Verdict::Almost(_)));
    }
}
