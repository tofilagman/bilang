//! The scratchpad's calculator: what a student types on paper, worked out.
//! `b^2 - 4ac`, `sqrt(ans)`, `(-b + 28)/(2a)`, `sin 30`, `5!`, `d = …`.
//!
//! Results stay exact while they can (3/4 + 1/6 = 11/12, √784 = 28) and fall
//! back to decimals otherwise (√2 ≈ 1.414214). Every line is also turned back
//! into a `$…$` formula, so the pad shows it as real math (see math.rs).

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Num {
    /// numerator / denominator, in lowest terms, denominator > 0
    Exact(i128, i128),
    Approx(f64),
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

impl Num {
    fn exact(n: i128, d: i128) -> Num {
        if d == 0 {
            return Num::Approx(f64::NAN);
        }
        let g = gcd(n, d);
        let s = if d < 0 { -1 } else { 1 };
        Num::Exact(s * n / g, s * d / g)
    }

    pub fn f(self) -> f64 {
        match self {
            Num::Exact(n, d) => n as f64 / d as f64,
            Num::Approx(x) => x,
        }
    }

    /// A float that is (almost exactly) a simple fraction becomes exact again,
    /// so sin 30 gives 1/2 rather than 0.49999999999999994.
    fn snap(x: f64) -> Num {
        if x.is_finite() {
            for d in 1..=24i128 {
                let n = (x * d as f64).round();
                if (n / d as f64 - x).abs() < 1e-10 && n.abs() < 1e15 {
                    return Num::exact(n as i128, d);
                }
            }
        }
        Num::Approx(x)
    }
}

fn exact_op(a: Num, b: Num, op: char) -> Num {
    if let (Num::Exact(an, ad), Num::Exact(bn, bd)) = (a, b) {
        let r = match op {
            '+' => an.checked_mul(bd).zip(bn.checked_mul(ad)).and_then(|(x, y)| x.checked_add(y)).zip(ad.checked_mul(bd)),
            '-' => an.checked_mul(bd).zip(bn.checked_mul(ad)).and_then(|(x, y)| x.checked_sub(y)).zip(ad.checked_mul(bd)),
            '*' => an.checked_mul(bn).zip(ad.checked_mul(bd)),
            _ => an.checked_mul(bd).zip(ad.checked_mul(bn)),
        };
        if let Some((n, d)) = r {
            return Num::exact(n, d);
        }
    }
    let (x, y) = (a.f(), b.f());
    Num::Approx(match op {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        _ => x / y,
    })
}

fn isqrt(n: i128) -> Option<i128> {
    if n < 0 {
        return None;
    }
    let r = (n as f64).sqrt().round() as i128;
    (r - 1..=r + 1).find(|&k| k >= 0 && k * k == n)
}

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Num(Num, String),
    Var(String),
    Neg(Box<Expr>),
    /// op, left, right, written without a sign (2a, 4ac)
    Bin(char, Box<Expr>, Box<Expr>, bool),
    Func(String, Box<Expr>),
    Group(Box<Expr>),
    Fact(Box<Expr>),
    Abs(Box<Expr>),
}

const FUNCS: [&str; 10] = ["sqrt", "sin", "cos", "tan", "asin", "acos", "atan", "log", "ln", "abs"];

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Num(String),
    Name(String),
    Op(char),
}

fn tokens(s: &str, known: &dyn Fn(&str) -> bool) -> Result<Vec<Tok>, String> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || c == '.' {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            out.push(Tok::Num(cs[st..i].iter().collect()));
        } else if c.is_alphabetic() && c != 'π' && c != '√' {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let word: String = cs[st..i].iter().collect();
            if FUNCS.contains(&word.as_str()) || word == "pi" || word == "ans" || known(&word) {
                out.push(Tok::Name(word));
            } else if word.chars().all(char::is_alphabetic) {
                // "ac" is a × c, the way it's written in formulas
                for ch in word.chars() {
                    out.push(Tok::Name(ch.to_string()));
                }
            } else {
                out.push(Tok::Name(word));
            }
        } else {
            let op = match c {
                '×' | '·' | '*' => '*',
                '÷' | '/' => '/',
                '−' | '–' | '-' => '-',
                '²' => {
                    out.push(Tok::Op('^'));
                    out.push(Tok::Num("2".into()));
                    i += 1;
                    continue;
                }
                '³' => {
                    out.push(Tok::Op('^'));
                    out.push(Tok::Num("3".into()));
                    i += 1;
                    continue;
                }
                '+' | '^' | '(' | ')' | '!' | '|' | '√' | 'π' | '=' | ',' => c,
                _ => return Err(format!("I don't know the symbol \"{c}\".")),
            };
            out.push(Tok::Op(op));
            i += 1;
        }
    }
    Ok(out)
}

struct P {
    t: Vec<Tok>,
    i: usize,
}

impl P {
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.i)
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(&Tok::Op(c)) {
            self.i += 1;
            true
        } else {
            false
        }
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let mut e = self.term()?;
        loop {
            if self.eat('+') {
                e = Expr::Bin('+', Box::new(e), Box::new(self.term()?), false);
            } else if self.eat('-') {
                e = Expr::Bin('-', Box::new(e), Box::new(self.term()?), false);
            } else {
                return Ok(e);
            }
        }
    }

    /// Does the next token start a value (so "2a" or "3(…)" multiplies)?
    fn starts_value(&self) -> bool {
        matches!(self.peek(), Some(Tok::Num(_) | Tok::Name(_) | Tok::Op('(' | '√' | 'π')))
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut e = self.unary()?;
        loop {
            if self.eat('*') {
                e = Expr::Bin('*', Box::new(e), Box::new(self.unary()?), false);
            } else if self.eat('/') {
                e = Expr::Bin('/', Box::new(e), Box::new(self.unary()?), false);
            } else if self.starts_value() {
                e = Expr::Bin('*', Box::new(e), Box::new(self.power()?), true);
            } else {
                return Ok(e);
            }
        }
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.eat('-') {
            return Ok(Expr::Neg(Box::new(self.unary()?)));
        }
        if self.eat('+') {
            return self.unary();
        }
        self.power()
    }

    fn power(&mut self) -> Result<Expr, String> {
        let mut base = self.primary()?;
        while self.eat('!') {
            base = Expr::Fact(Box::new(base));
        }
        if self.eat('^') {
            let exp = self.unary()?;
            return Ok(Expr::Bin('^', Box::new(base), Box::new(exp), false));
        }
        Ok(base)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.peek().cloned() {
            Some(Tok::Num(s)) => {
                self.i += 1;
                let n = parse_number(&s).ok_or_else(|| format!("\"{s}\" isn't a number."))?;
                Ok(Expr::Num(n, s))
            }
            Some(Tok::Name(name)) => {
                self.i += 1;
                if FUNCS.contains(&name.as_str()) {
                    let arg = if self.eat('(') {
                        let a = self.expr()?;
                        if !self.eat(')') {
                            return Err("A \")\" is missing.".into());
                        }
                        a
                    } else {
                        self.power()?
                    };
                    return Ok(Expr::Func(name, Box::new(arg)));
                }
                Ok(Expr::Var(if name == "pi" { "π".into() } else { name }))
            }
            Some(Tok::Op('π')) => {
                self.i += 1;
                Ok(Expr::Var("π".into()))
            }
            Some(Tok::Op('√')) => {
                self.i += 1;
                Ok(Expr::Func("sqrt".into(), Box::new(self.power()?)))
            }
            Some(Tok::Op('(')) => {
                self.i += 1;
                let e = self.expr()?;
                if !self.eat(')') {
                    return Err("A \")\" is missing.".into());
                }
                Ok(Expr::Group(Box::new(e)))
            }
            Some(Tok::Op('|')) => {
                self.i += 1;
                let e = self.expr()?;
                if !self.eat('|') {
                    return Err("A closing \"|\" is missing.".into());
                }
                Ok(Expr::Abs(Box::new(e)))
            }
            Some(Tok::Op(c)) => Err(format!("Something is missing before \"{c}\".")),
            None => Err("Something is missing at the end.".into()),
        }
    }
}

fn parse_number(s: &str) -> Option<Num> {
    match s.split_once('.') {
        None => s.parse::<i128>().ok().map(|n| Num::exact(n, 1)),
        Some((a, b)) if b.len() <= 12 && !b.contains('.') => {
            let den = 10i128.pow(b.len() as u32);
            let a: i128 = if a.is_empty() { 0 } else { a.parse().ok()? };
            let b: i128 = if b.is_empty() { 0 } else { b.parse().ok()? };
            Some(Num::exact(a * den + b, den))
        }
        _ => None,
    }
}

fn eval(e: &Expr, vars: &BTreeMap<String, Num>) -> Result<Num, String> {
    Ok(match e {
        Expr::Num(n, _) => *n,
        Expr::Var(v) if v == "π" => Num::Approx(std::f64::consts::PI),
        Expr::Var(v) => *vars.get(v).ok_or_else(|| format!("I don't know \"{v}\" yet. Give it a value first, like {v} = 5"))?,
        Expr::Neg(a) => exact_op(Num::Exact(0, 1), eval(a, vars)?, '-'),
        Expr::Group(a) => eval(a, vars)?,
        Expr::Abs(a) => match eval(a, vars)? {
            Num::Exact(n, d) => Num::Exact(n.abs(), d),
            Num::Approx(x) => Num::Approx(x.abs()),
        },
        Expr::Fact(a) => match eval(a, vars)? {
            Num::Exact(n, 1) if (0..=25).contains(&n) => Num::Exact((1..=n).product::<i128>().max(1), 1),
            _ => return Err("! works on whole numbers from 0 to 25.".into()),
        },
        Expr::Bin('^', a, b, _) => {
            let (x, y) = (eval(a, vars)?, eval(b, vars)?);
            match (x, y) {
                (Num::Exact(n, d), Num::Exact(k, 1)) if k.abs() <= 60 => {
                    let p = k.unsigned_abs() as u32;
                    match (n.checked_pow(p), d.checked_pow(p)) {
                        (Some(np), Some(dp)) if k >= 0 => Num::exact(np, dp),
                        (Some(np), Some(dp)) if np != 0 => Num::exact(dp, np),
                        _ => Num::Approx(x.f().powf(y.f())),
                    }
                }
                _ => Num::snap(x.f().powf(y.f())),
            }
        }
        Expr::Bin(op, a, b, _) => {
            let (x, y) = (eval(a, vars)?, eval(b, vars)?);
            if *op == '/' && y.f() == 0.0 {
                return Err("You can't divide by 0.".into());
            }
            exact_op(x, y, *op)
        }
        Expr::Func(f, a) => {
            let x = eval(a, vars)?;
            let deg = std::f64::consts::PI / 180.0;
            match f.as_str() {
                "sqrt" => match x {
                    Num::Exact(n, d) if n >= 0 => match (isqrt(n), isqrt(d)) {
                        (Some(a), Some(b)) => Num::exact(a, b),
                        _ => Num::Approx(x.f().sqrt()),
                    },
                    _ if x.f() < 0.0 => return Err("There's no real square root of a negative number.".into()),
                    _ => Num::Approx(x.f().sqrt()),
                },
                "sin" => Num::snap((x.f() * deg).sin()),
                "cos" => Num::snap((x.f() * deg).cos()),
                "tan" => {
                    if (x.f() % 180.0).abs() == 90.0 {
                        return Err("tan isn't defined at 90°.".into());
                    }
                    Num::snap((x.f() * deg).tan())
                }
                "asin" => Num::snap(x.f().asin() / deg),
                "acos" => Num::snap(x.f().acos() / deg),
                "atan" => Num::snap(x.f().atan() / deg),
                "log" | "ln" if x.f() <= 0.0 => return Err("Logarithms need a number bigger than 0.".into()),
                "log" => Num::snap(x.f().log10()),
                "ln" => Num::Approx(x.f().ln()),
                _ => match x {
                    Num::Exact(n, d) => Num::Exact(n.abs(), d),
                    Num::Approx(v) => Num::Approx(v.abs()),
                },
            }
        }
    })
}

fn strip(e: &Expr) -> &Expr {
    if let Expr::Group(a) = e { strip(a) } else { e }
}

fn num_markup(n: Num) -> String {
    match n {
        Num::Exact(n, 1) => minus(n.to_string()),
        Num::Exact(n, d) if n < 0 => format!("−\\frac{{{}}}{{{d}}}", -n),
        Num::Exact(n, d) => format!("\\frac{{{n}}}{{{d}}}"),
        Num::Approx(x) => minus(decimal(x)),
    }
}

fn minus(s: String) -> String {
    s.replacen('-', "−", 1)
}

fn decimal(x: f64) -> String {
    if !x.is_finite() {
        return "undefined".into();
    }
    let s = format!("{:.6}", x);
    let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    if s == "-0" { "0".into() } else { s }
}

/// The expression as a `$…$` formula body.
fn markup(e: &Expr) -> String {
    match e {
        Expr::Num(_, s) => s.clone(),
        Expr::Var(v) => {
            // x1 → x₁
            let (head, tail): (String, String) = (v.chars().take_while(|c| !c.is_ascii_digit()).collect(), v.chars().skip_while(|c| !c.is_ascii_digit()).collect());
            if tail.is_empty() || head.is_empty() { v.clone() } else { format!("{head}_{{{tail}}}") }
        }
        Expr::Neg(a) => format!("−{}", markup(a)),
        Expr::Group(a) => format!("({})", markup(a)),
        Expr::Abs(a) => format!("|{}|", markup(a)),
        Expr::Fact(a) => format!("{}!", markup(a)),
        Expr::Bin('/', a, b, _) => format!("\\frac{{{}}}{{{}}}", markup(strip(a)), markup(strip(b))),
        Expr::Bin('^', a, b, _) => format!("{}^{{{}}}", markup(a), markup(strip(b))),
        Expr::Bin('*', a, b, true) => format!("{}{}", markup(a), markup(b)),
        Expr::Bin(op, a, b, _) => format!("{} {} {}", markup(a), if *op == '*' { '×' } else if *op == '-' { '−' } else { *op }, markup(b)),
        Expr::Func(f, a) if f == "sqrt" => format!("\\sqrt{{{}}}", markup(strip(a))),
        Expr::Func(f, a) => format!("{f}({})", markup(strip(a))),
    }
}

/// One worked line.
#[derive(Clone, Debug)]
pub struct Line {
    /// What was typed, as a formula: "$b^{2} − 4ac$".
    pub shown: String,
    /// The answer as a formula, and its decimal when it's a fraction.
    pub result: String,
    pub approx: Option<String>,
    /// The answer as plain text, to drop into the answer box.
    pub plain: String,
    pub value: Num,
}

/// The pad's memory: the problem's given values, the student's own, and `ans`.
#[derive(Default)]
pub struct Pad {
    pub vars: BTreeMap<String, Num>,
}

impl Pad {
    /// Give a name a value from text like "-12" or "2.5".
    pub fn set(&mut self, name: &str, value: &str) {
        if let Ok(line) = self.run_expr(value) {
            self.vars.insert(name.to_string(), line.value);
        }
    }

    fn run_expr(&self, src: &str) -> Result<Line, String> {
        let known = |w: &str| self.vars.contains_key(w);
        let toks = tokens(src, &known)?;
        let mut p = P { t: toks, i: 0 };
        let e = p.expr()?;
        if let Some(t) = p.peek() {
            return Err(match t {
                Tok::Op(')') => "There's an extra \")\".".into(),
                _ => "I couldn't read all of that.".into(),
            });
        }
        let value = eval(&e, &self.vars)?;
        if value.f().is_nan() {
            return Err("That doesn't give a number.".into());
        }
        let approx = match value {
            Num::Exact(_, d) if d != 1 => Some(decimal(value.f())),
            _ => None,
        };
        let plain = match value {
            Num::Exact(n, 1) => n.to_string(),
            Num::Exact(n, d) => format!("{n}/{d}"),
            Num::Approx(x) => decimal(x),
        };
        Ok(Line { shown: format!("${}$", markup(&e)), result: format!("${}$", num_markup(value)), approx, plain, value })
    }

    /// Work out one line; `name = …` also remembers it. The answer becomes `ans`.
    pub fn run(&mut self, src: &str) -> Result<Line, String> {
        let src = src.trim();
        if src.is_empty() {
            return Err("Type something to work out, like b^2 - 4ac".into());
        }
        let (name, body) = match src.split_once('=') {
            Some((l, r)) if !r.contains('=') && is_name(l.trim()) => (Some(l.trim().to_string()), r),
            Some(_) => return Err("Use = only to name a value, like d = b^2 - 4ac".into()),
            None => (None, src),
        };
        let mut line = self.run_expr(body)?;
        if let Some(n) = name {
            if FUNCS.contains(&n.as_str()) || n == "pi" || n == "ans" {
                return Err(format!("\"{n}\" is already a math word; pick another name."));
            }
            self.vars.insert(n.clone(), line.value);
            line.shown = format!("${} = {}", n, &line.shown[1..]);
        }
        self.vars.insert("ans".into(), line.value);
        Ok(line)
    }
}

fn is_name(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(char::is_alphabetic) && cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(pad: &mut Pad, s: &str) -> String {
        pad.run(s).map(|l| l.plain).unwrap_or_else(|e| format!("ERR {e}"))
    }

    #[test]
    fn formulas_with_given_values() {
        let mut pad = Pad::default();
        pad.set("a", "4");
        pad.set("b", "-12");
        pad.set("c", "-40");
        assert_eq!(run(&mut pad, "b^2 - 4ac"), "784");
        assert_eq!(run(&mut pad, "sqrt(ans)"), "28");
        assert_eq!(run(&mut pad, "(-b + 28)/(2a)"), "5");
        assert_eq!(run(&mut pad, "(-b - 28) / (2a)"), "-2");
        assert_eq!(run(&mut pad, "d = b² − 4ac"), "784");
        assert_eq!(run(&mut pad, "√d"), "28");
    }

    #[test]
    fn exact_and_approximate() {
        let mut pad = Pad::default();
        assert_eq!(run(&mut pad, "3/4 + 1/6"), "11/12");
        assert_eq!(run(&mut pad, "2^-3"), "1/8");
        assert_eq!(run(&mut pad, "sqrt 2"), "1.414214");
        assert_eq!(run(&mut pad, "sin 30"), "1/2");
        assert_eq!(run(&mut pad, "cos(60) * 10"), "5");
        assert_eq!(run(&mut pad, "5!/(2!3!)"), "10");
        assert_eq!(run(&mut pad, "2(3 + 4)"), "14");
        assert_eq!(run(&mut pad, "1.5 × 4"), "6");
        assert_eq!(run(&mut pad, "|3 - 10|"), "7");
        assert_eq!(run(&mut pad, "log 1000"), "3");
        assert!(run(&mut pad, "q + 1").starts_with("ERR I don't know \"q\""));
        assert!(run(&mut pad, "4/0").starts_with("ERR"));
        assert!(run(&mut pad, "(1 + 2").starts_with("ERR"));
    }

    #[test]
    fn shown_as_real_math() {
        let mut pad = Pad::default();
        pad.set("a", "2");
        pad.set("b", "3");
        let l = pad.run("(-b + 5)/(2a)").unwrap();
        assert_eq!(l.shown, "$\\frac{−b + 5}{2a}$");
        assert_eq!(pad.run("sqrt(b^2 + 16)").unwrap().shown, "$\\sqrt{b^{2} + 16}$");
        assert_eq!(pad.run("3/4 + 1/6").unwrap().result, "$\\frac{11}{12}$");
        pad.set("x1", "4");
        assert_eq!(pad.run("x1 + 1").unwrap().shown, "$x_{1} + 1$");
    }
}
