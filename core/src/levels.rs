//! The adventure map: worlds of levels from counting to quadratics. Every
//! level has a short lesson, and every problem it makes carries its own hints,
//! from a nudge to a fully worked solution with the problem's own numbers.

use crate::answer::{Answer, gcd, lcm, minus, reduce, show_dec};
use crate::advanced;
use crate::figure::Figure;
use crate::expr::{self, Tok, show, solve, sup};
use rand::{Rng, rngs::StdRng};

pub struct Problem {
    pub prompt: String,
    /// Pictures to count, for the youngest levels.
    pub visual: Option<String>,
    pub answer: Answer,
    /// From gentle to complete; the last one works the whole problem.
    pub hints: Vec<String>,
    /// Set for problems worked stacked on paper ("two-layer"), digit by digit.
    pub column: Option<Column>,
    /// The expression, for order-of-operations problems, so the tutorial can walk it.
    pub expr: Option<Vec<Tok>>,
    /// A labeled diagram (triangles, the unit circle).
    pub figure: Option<Figure>,
    /// Values the scratchpad starts with, like a = 4, b = −12 (name, number).
    pub given: Vec<(String, String)>,
}

impl Problem {
    /// The question on one line, for the score sheet ("345 + 278", "814 ÷ 7").
    pub fn describe(&self) -> String {
        match self.column {
            Some(c) => {
                let op = match c.op {
                    ColOp::Add => "+",
                    ColOp::Sub => "−",
                    ColOp::Mul => "×",
                    ColOp::Div => "÷",
                };
                format!("{} {op} {}", c.top, c.bottom)
            }
            None => crate::mathtext::plain(&self.prompt).replace('\n', " ").trim_end_matches(" = ?").to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// A problem written in columns: top over bottom, or dividend inside the
/// long-division bracket with the divisor outside.
#[derive(Clone, Copy, Debug)]
pub struct Column {
    pub op: ColOp,
    pub top: i64,
    pub bottom: i64,
}

pub struct Level {
    pub id: &'static str,
    pub title: &'static str,
    pub lesson: &'static str,
    pub make: fn(&mut StdRng) -> Problem,
}

pub struct World {
    pub name: &'static str,
    pub emoji: &'static str,
    pub grades: &'static str,
    pub levels: &'static [Level],
}

pub(crate) type G = StdRng;

pub(crate) fn ri(g: &mut G, lo: i64, hi: i64) -> i64 {
    g.random_range(lo..=hi)
}

pub(crate) fn pick<T: Copy>(g: &mut G, xs: &[T]) -> T {
    xs[g.random_range(0..xs.len())]
}

pub(crate) fn nonzero(g: &mut G, lo: i64, hi: i64) -> i64 {
    loop {
        let n = ri(g, lo, hi);
        if n != 0 {
            return n;
        }
    }
}

pub(crate) const NAMES: [&str; 10] = ["Ana", "Ben", "Lia", "Miko", "Jun", "Bea", "Paolo", "Tala", "Gabby", "Nico"];
const FRUITS: [&str; 6] = ["🍎", "🍊", "🍓", "🥭", "🍌", "🍇"];

pub(crate) fn problem(prompt: impl Into<String>, answer: Answer, hints: Vec<String>) -> Problem {
    Problem { prompt: prompt.into(), visual: None, answer, hints, column: None, expr: None, figure: None, given: Vec::new() }
}

/// " + 5" or " − 5", for writing terms after the first.
pub(crate) fn signed(n: i64) -> String {
    if n < 0 { format!(" − {}", -n) } else { format!(" + {n}") }
}

/// A coefficient on x: 1 → "x", −1 → "−x", 3 → "3x".
pub(crate) fn coef(n: i64, var: &str) -> String {
    match n {
        1 => var.to_string(),
        -1 => format!("−{var}"),
        _ => format!("{}{var}", minus(n)),
    }
}

/// A later term with its sign: " + 3x", " − x", or nothing for 0.
pub(crate) fn term(n: i64, var: &str) -> String {
    match n {
        0 => String::new(),
        _ if n < 0 => format!(" − {}", coef(-n, var)),
        _ => format!(" + {}", coef(n, var)),
    }
}

pub(crate) fn wrap(n: i64) -> String {
    if n < 0 { format!("(−{})", -n) } else { n.to_string() }
}

/// The problem, with values ready in the scratchpad.
pub(crate) fn given(p: Problem, values: &[(&str, i64)]) -> Problem {
    Problem { given: values.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(), ..p }
}

/// The problem, with a diagram.
pub(crate) fn with(figure: Figure, p: Problem) -> Problem {
    Problem { figure: Some(figure), ..p }
}

/// A right triangle: angle at the bottom left, right angle at the bottom right.
pub(crate) fn right(adj: f64, opp: f64, angle: &str, adj_label: impl Into<String>, opp_label: impl Into<String>, hyp_label: impl Into<String>) -> Figure {
    Figure::Right { adj, opp, angle: angle.into(), adj_label: adj_label.into(), opp_label: opp_label.into(), hyp_label: hyp_label.into() }
}

/// A stacked fraction for inside a `$…$` formula (drawn by math.rs).
pub(crate) fn frac(top: impl std::fmt::Display, bottom: impl std::fmt::Display) -> String {
    format!("\\frac{{{top}}}{{{bottom}}}")
}

pub(crate) fn sqrt(x: impl std::fmt::Display) -> String {
    format!("\\sqrt{{{x}}}")
}

/// n/d inside a formula: a fraction (minus sign in front), or a whole number.
pub(crate) fn fm(n: i64, d: i64) -> String {
    match (n, d) {
        (_, 1) => minus(n),
        _ if n < 0 => format!("−{}", frac(-n, d)),
        _ => frac(n, d),
    }
}

/// n/d as a formula on its own: "$\frac{3}{4}$", or just the whole number.
pub(crate) fn mfr(n: i64, d: i64) -> String {
    if d == 1 { minus(n) } else { format!("${}$", fm(n, d)) }
}

/// "Simplify: 6/8 = 3/4" or a note that it already is.
pub(crate) fn simplify_hint(n: i64, d: i64) -> String {
    let (a, b) = reduce(n, d);
    if (a, b) == (n, d) {
        format!("{} is already in simplest form, so that's the answer!", mfr(n, d))
    } else if b == 1 {
        format!("{} is a whole number: {n} ÷ {d} = {}", mfr(n, d), minus(a))
    } else {
        let g = gcd(n, d);
        format!("Simplify: divide top and bottom by {g}.\n${} = {}$", fm(n, d), fm(a, b))
    }
}

/// Hints for an expression: the order rule, then each step on the board.
fn expr_problem(toks: &[Tok], v: i64, steps: &[expr::Step], ask: Option<String>) -> Problem {
    let mut hints = vec!["Order of operations: ( ) first, then exponents, then × and ÷ from left to right, then + and − from left to right.".to_string()];
    for (i, s) in steps.iter().enumerate() {
        hints.push(format!("Step {}: {}\n→ {}", i + 1, s.rule, s.after));
    }
    let mut p = problem(ask.unwrap_or_else(|| format!("{} = ?", show(toks))), Answer::Int(v), hints);
    p.expr = Some(toks.to_vec());
    p
}

// ───────────────────────── World 1: Number Garden ─────────────────────────

fn add10(g: &mut G) -> Problem {
    let a = ri(g, 1, 8);
    let b = ri(g, 1, 9 - a);
    let f = pick(g, &FRUITS);
    let seq: Vec<String> = (a + 1..=a + b).map(|n| n.to_string()).collect();
    let mut p = problem(
        format!("{a} + {b} = ?"),
        Answer::Int(a + b),
        vec![
            if a == 1 { "Count the first group. There is just 1.".to_string() } else { format!("Count the first group. There are {a}.") },
            format!("Keep {a} in your head, then count on {b} more."),
            format!("{a} … {}. You land on {}!", seq.join(", "), a + b),
        ],
    );
    p.visual = Some(format!("{}   ➕   {}", f.repeat(a as usize), f.repeat(b as usize)));
    p
}

fn add20(g: &mut G) -> Problem {
    let (a, b) = loop {
        let (a, b) = (ri(g, 3, 9), ri(g, 3, 9));
        if a + b > 10 {
            break (a, b);
        }
    };
    let (big, small) = (a.max(b), a.min(b));
    let need = 10 - big;
    problem(
        format!("{a} + {b} = ?"),
        Answer::Int(a + b),
        vec![
            format!("Make a 10! {big} needs {need} more to become 10."),
            format!("Take {need} from {small} and give it to {big}. Now you have 10 and {}.", small - need),
            format!("10 + {} = {}", small - need, a + b),
        ],
    )
}

fn sub20(g: &mut G) -> Problem {
    let a = ri(g, 5, 18);
    let b = ri(g, 1, a.min(9));
    let back: Vec<String> = (a - b..a).rev().map(|n| n.to_string()).collect();
    let mut p = problem(
        format!("{a} − {b} = ?"),
        Answer::Int(a - b),
        vec![
            format!("Start at {a} and count back {b}."),
            format!("Or think adding: {b} + ? = {a}"),
            format!("{a} … {}. You land on {}!", back.join(", "), a - b),
        ],
    );
    if a <= 12 {
        let f = pick(g, &FRUITS);
        p.visual = Some(format!("{}{}", f.repeat((a - b) as usize), "❌".repeat(b as usize)));
    }
    p
}

fn missing(g: &mut G) -> Problem {
    let a = ri(g, 2, 12);
    let x = ri(g, 1, 12);
    let c = a + x;
    problem(
        format!("{a} + ? = {c}"),
        Answer::Int(x),
        vec![
            format!("What do you add to {a} to get {c}?"),
            format!("Turn it around: {c} − {a} = ?"),
            format!("{c} − {a} = {x}. Check: {a} + {x} = {c} ✔"),
        ],
    )
}

fn add2d(g: &mut G) -> Problem {
    let a = ri(g, 15, 79);
    let b = ri(g, 12, 99 - a.min(87));
    let ones = a % 10 + b % 10;
    let carry = ones / 10;
    let tens = a / 10 + b / 10 + carry;
    let carried = if carry > 0 { " + 1 carried".to_string() } else { String::new() };
    problem(
        format!("{a} + {b} = ?"),
        Answer::Int(a + b),
        vec![
            format!(
                "Add the ones first: {} + {} = {ones}.{}",
                a % 10,
                b % 10,
                if carry > 0 { format!(" Write {} and carry 1 ten.", ones % 10) } else { String::new() }
            ),
            format!("Now the tens: {} + {}{carried} = {tens} tens.", a / 10, b / 10),
            format!("{tens} tens and {} ones = {}", ones % 10, a + b),
        ],
    )
}

fn sub2d(g: &mut G) -> Problem {
    let a = ri(g, 30, 99);
    let b = ri(g, 11, a - 5);
    let (a1, a10, b1, b10) = (a % 10, a / 10, b % 10, b / 10);
    let first = if a1 < b1 {
        format!("Ones: you can't take {b1} from {a1}, so borrow a ten. {a1} becomes {}, and {a10} tens become {}.", a1 + 10, a10 - 1)
    } else {
        format!("Ones first: {a1} − {b1} = {}.", a1 - b1)
    };
    let (o, t) = if a1 < b1 { (a1 + 10 - b1, a10 - 1 - b10) } else { (a1 - b1, a10 - b10) };
    problem(
        format!("{a} − {b} = ?"),
        Answer::Int(a - b),
        vec![
            first,
            format!("Ones: {o}. Tens: {} − {b10} = {t}.", if a1 < b1 { a10 - 1 } else { a10 }),
            format!("{t} tens and {o} ones = {}", a - b),
        ],
    )
}

// ───────────────────────── World 2: Times Jungle ─────────────────────────

fn times(g: &mut G, lo: i64, hi: i64) -> Problem {
    let a = ri(g, lo, hi);
    let b = ri(g, 1, 10);
    let skip: Vec<String> = (1..=a).map(|k| (k * b).to_string()).collect();
    let trick = match (a, b) {
        (_, 1) | (1, _) => "Anything times 1 stays the same!".to_string(),
        (9, _) | (_, 9) => {
            let o = if a == 9 { b } else { a };
            format!("Nines trick: 10 × {o} = {}, then take away one {o}.", 10 * o)
        }
        (5, _) | (_, 5) => {
            let o = if a == 5 { b } else { a };
            format!("Fives trick: 10 × {o} = {}, and half of that is 5 × {o}.", 10 * o)
        }
        _ => format!("Do you know {} × {b} = {}? Add one more group of {b}.", a - 1, (a - 1) * b),
    };
    let mut p = problem(
        format!("{a} × {b} = ?"),
        Answer::Int(a * b),
        vec![
            format!("{a} × {b} means {a} groups of {b}."),
            trick,
            format!("Count by {b}s, {a} times: {}", skip.join(", ")),
        ],
    );
    if a <= 5 && b <= 10 {
        p.visual = Some(vec!["●".repeat(b as usize); a as usize].join("\n"));
    }
    p
}

fn times2to5(g: &mut G) -> Problem {
    times(g, 2, 5)
}

fn times6to10(g: &mut G) -> Problem {
    times(g, 6, 10)
}

fn divfacts(g: &mut G) -> Problem {
    let a = ri(g, 2, 10);
    let b = ri(g, 1, 10);
    let n = a * b;
    let skip: Vec<String> = (1..=b).map(|k| (k * a).to_string()).collect();
    problem(
        format!("{n} ÷ {a} = ?"),
        Answer::Int(b),
        vec![
            format!("{n} ÷ {a} asks: how many groups of {a} make {n}?"),
            format!("Use your times tables: {a} × ? = {n}"),
            format!("Count by {a}s until you reach {n}: {}. That's {b} counts!", skip.join(", ")),
        ],
    )
}

fn mult2x1(g: &mut G) -> Problem {
    let a = loop {
        let a = ri(g, 12, 99);
        if a % 10 != 0 {
            break a;
        }
    };
    let b = ri(g, 2, 9);
    let (t, o) = (a / 10 * 10, a % 10);
    problem(
        format!("{a} × {b} = ?"),
        Answer::Int(a * b),
        vec![
            format!("Break {a} apart: {t} + {o}."),
            format!("{t} × {b} = {} and {o} × {b} = {}.", t * b, o * b),
            format!("{} + {} = {}", t * b, o * b, a * b),
        ],
    )
}

fn mult2x2(g: &mut G) -> Problem {
    let a = ri(g, 12, 59);
    let b = loop {
        let b = ri(g, 11, 29);
        if b % 10 != 0 {
            break b;
        }
    };
    let (bt, bo) = (b / 10 * 10, b % 10);
    problem(
        format!("{a} × {b} = ?"),
        Answer::Int(a * b),
        vec![
            format!("Break {b} apart: {bt} + {bo}."),
            format!("{a} × {bt} = {} and {a} × {bo} = {}.", a * bt, a * bo),
            format!("{} + {} = {}", a * bt, a * bo, a * b),
        ],
    )
}

fn longdiv(g: &mut G) -> Problem {
    let d = ri(g, 2, 9);
    let q = ri(g, 3, 30);
    let r = if g.random_bool(0.8) { ri(g, 1, d - 1) } else { 0 };
    let n = d * q + r;
    let last = if r == 0 {
        format!("{n} − {} = 0, nothing left over! The answer is {q}.", d * q)
    } else {
        format!("{n} − {} = {r} left over, so the answer is {q} r {r}.", d * q)
    };
    problem(
        format!("{n} ÷ {d} = ?"),
        Answer::Remainder(q, r),
        vec![
            format!("How many {d}s fit into {n}? Whatever doesn't fit is the remainder."),
            format!("{d} × {q} = {} fits, but {d} × {} = {} is too big.", d * q, q + 1, d * (q + 1)),
            last,
        ],
    )
}

// ───────────────────────── World: Column City (stacked, "two-layer") ─────────────────────────

const PLACES: [&str; 6] = ["Ones", "Tens", "Hundreds", "Thousands", "Ten-thousands", "Hundred-thousands"];

fn place(i: usize) -> &'static str {
    PLACES[i.min(PLACES.len() - 1)]
}

fn lower(i: usize) -> String {
    place(i).to_lowercase()
}

/// Digit i of n, counting from the ones place.
fn digit(n: i64, i: usize) -> i64 {
    n / 10i64.pow(i as u32) % 10
}

/// A random number with exactly `d` digits.
fn ndigits(g: &mut G, d: u32) -> i64 {
    ri(g, 10i64.pow(d - 1), 10i64.pow(d) - 1)
}

fn column(prompt: &str, op: ColOp, top: i64, bottom: i64, answer: Answer, hints: Vec<String>) -> Problem {
    let mut p = problem(prompt, answer, hints);
    p.column = Some(Column { op, top, bottom });
    p
}

fn col_add(g: &mut G, d: u32) -> Problem {
    let (a, b) = loop {
        let (a, b) = (ndigits(g, d), ndigits(g, d));
        // Mostly problems with at least one carry: that's the skill being practiced.
        let carries = (0..d as usize).any(|i| digit(a, i) + digit(b, i) >= 10);
        if carries || g.random_bool(0.15) {
            break (a, b);
        }
    };
    let mut hints = vec!["Start on the right, in the ones column. Add down each column. If a column makes 10 or more, write the ones digit and carry 1 to the next column.".to_string()];
    let mut carry = 0;
    for i in 0..d as usize {
        let (x, y) = (digit(a, i), digit(b, i));
        let s = x + y + carry;
        let carried = if carry > 0 { " + 1 carried" } else { "" };
        hints.push(if s >= 10 {
            format!("{}: {x} + {y}{carried} = {s} → write {}, carry 1 to the {}.", place(i), s % 10, lower(i + 1))
        } else {
            format!("{}: {x} + {y}{carried} = {s} → write {s}.", place(i))
        });
        carry = s / 10;
    }
    if carry > 0 {
        hints.push(format!("{}: only the carried 1 → write 1.", place(d as usize)));
    }
    if let Some(last) = hints.last_mut() {
        last.push_str(&format!("\nThe answer is {}.", a + b));
    }
    column("Add them up!", ColOp::Add, a, b, Answer::Int(a + b), hints)
}

fn col_sub(g: &mut G, d: u32) -> Problem {
    let (a, b) = loop {
        let (x, y) = (ndigits(g, d), ndigits(g, d));
        let (a, b) = (x.max(y), x.min(y));
        let borrows = (0..d as usize).any(|i| digit(a, i) < digit(b, i));
        if a != b && (borrows || g.random_bool(0.15)) {
            break (a, b);
        }
    };
    let mut hints = vec!["Start on the right, in the ones column. Take the bottom digit away from the top one. If the top digit is too small, borrow 1 from the next column: it's worth 10 here.".to_string()];
    let mut lent = 0;
    for i in 0..d as usize {
        let (orig, y) = (digit(a, i), digit(b, i));
        let mut text = format!("{}: ", place(i));
        let x = if lent > 0 && orig == 0 {
            // A 0 that had to lend borrows from its own neighbour first.
            text.push_str(&format!("this 0 had to lend 1, so it borrows from the {} too: 10 − 1 = 9. ", lower(i + 1)));
            lent = 1;
            9
        } else {
            if lent > 0 {
                text.push_str(&format!("this column lent 1, so its {orig} is now {}. ", orig - 1));
            }
            let x = orig - lent;
            lent = 0;
            if x < y {
                text.push_str(&format!("{x} is too small to take {y}, so borrow 1 from the {}: {} − {y} = {} → write {}.", lower(i + 1), x + 10, x + 10 - y, x + 10 - y));
                lent = 1;
                hints.push(text);
                continue;
            }
            x
        };
        text.push_str(&format!("{x} − {y} = {} → write {}.", x - y, x - y));
        hints.push(text);
    }
    if let Some(last) = hints.last_mut() {
        last.push_str(&format!("\nThe answer is {}. (Zeros at the front can be left blank.)", a - b));
    }
    column("Take away!", ColOp::Sub, a, b, Answer::Int(a - b), hints)
}

fn col_mul(g: &mut G, d: u32) -> Problem {
    let a = ndigits(g, d);
    let b = ri(g, 2, 9);
    let mut hints = vec![format!("Start on the right. Multiply {b} by each digit of the top number. Write the ones digit and carry the tens to the next column; add the carry after you multiply.")];
    let mut carry = 0;
    for i in 0..d as usize {
        let x = digit(a, i);
        let p = x * b + carry;
        let carried = if carry > 0 { format!(", + {carry} carried = {p}") } else { String::new() };
        hints.push(if p >= 10 && i + 1 < d as usize {
            format!("{}: {b} × {x} = {}{carried} → write {}, carry {}.", place(i), x * b, p % 10, p / 10)
        } else if p >= 10 {
            format!("{}: {b} × {x} = {}{carried} → write {} and put the {} in front.", place(i), x * b, p % 10, p / 10)
        } else {
            format!("{}: {b} × {x} = {}{carried} → write {p}.", place(i), x * b)
        });
        carry = p / 10;
    }
    if let Some(last) = hints.last_mut() {
        last.push_str(&format!("\nThe answer is {}.", a * b));
    }
    column("Multiply!", ColOp::Mul, a, b, Answer::Int(a * b), hints)
}

fn col_div(g: &mut G, d: u32) -> Problem {
    let v = ri(g, 2, 9);
    let (n, q, r) = loop {
        let n = ndigits(g, d);
        let (q, r) = (n / v, n % v);
        // Mostly with a remainder, sometimes exact.
        if q >= 2 && (r > 0 || g.random_bool(0.3)) {
            break (n, q, r);
        }
    };
    let mut hints = vec!["Long division, from the left: Divide, Multiply, Subtract, Bring down. Repeat until every digit is used.".to_string()];
    let ds: Vec<i64> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
    let mut rem = 0;
    let mut started = false;
    for (k, &dg) in ds.iter().enumerate() {
        let cur = rem * 10 + dg;
        let qd = cur / v;
        let next = ds.get(k + 1).map(|n| format!(" Bring down the {n} → {}.", (cur - qd * v) * 10 + n)).unwrap_or_default();
        if !started && qd == 0 {
            hints.push(format!("{v} doesn't fit into {cur}, so leave that spot on top blank and use the next digit too."));
            rem = cur;
            continue;
        }
        started = true;
        hints.push(format!("How many {v}s fit in {cur}? {qd}. Write {qd} on top. {qd} × {v} = {}, and {cur} − {} = {}.{next}", qd * v, qd * v, cur - qd * v));
        rem = cur - qd * v;
    }
    hints.push(if r == 0 { format!("Nothing is left over, so the answer is {q}.") } else { format!("{r} is left over. That's the remainder: {q} r {r}.") });
    column("Divide!", ColOp::Div, n, v, Answer::Remainder(q, r), hints)
}

fn col_add2(g: &mut G) -> Problem {
    col_add(g, 2)
}
fn col_sub2(g: &mut G) -> Problem {
    col_sub(g, 2)
}
fn col_mul2(g: &mut G) -> Problem {
    col_mul(g, 2)
}
fn col_div2(g: &mut G) -> Problem {
    col_div(g, 2)
}
fn col_add3(g: &mut G) -> Problem {
    col_add(g, 3)
}
fn col_sub3(g: &mut G) -> Problem {
    col_sub(g, 3)
}
fn col_mul3(g: &mut G) -> Problem {
    col_mul(g, 3)
}
fn col_div3(g: &mut G) -> Problem {
    col_div(g, 3)
}
fn col_add4(g: &mut G) -> Problem {
    col_add(g, 4)
}
fn col_sub4(g: &mut G) -> Problem {
    col_sub(g, 4)
}
fn col_mul4(g: &mut G) -> Problem {
    col_mul(g, 4)
}
fn col_div4(g: &mut G) -> Problem {
    col_div(g, 4)
}

// ───────────────────────── World 3: MDAS Castle ─────────────────────────

/// A random chain like 5 + 3 × 4 − 2 that needs both kinds of operation
/// and never goes below zero along the way.
fn mdas_chain(g: &mut G, terms: usize) -> Problem {
    loop {
        let mut toks = vec![Tok::Num(ri(g, 2, 12))];
        for _ in 1..terms {
            let op = pick(g, &['+', '−', '×', '÷']);
            let n = match op {
                '×' => ri(g, 2, 9),
                '÷' => ri(g, 2, 6),
                _ => ri(g, 1, 15),
            };
            toks.extend([Tok::Op(op), Tok::Num(n)]);
        }
        let has = |set: &[char]| toks.iter().any(|t| matches!(t, Tok::Op(c) if set.contains(c)));
        if !(has(&['×', '÷']) && has(&['+', '−'])) {
            continue;
        }
        if let Some((v, steps)) = solve(&toks)
            && v <= 150
            && steps.iter().all(|s| s.value >= 0)
        {
            return expr_problem(&toks, v, &steps, None);
        }
    }
}

fn mdas2(g: &mut G) -> Problem {
    mdas_chain(g, 3)
}

fn mdas3(g: &mut G) -> Problem {
    mdas_chain(g, 4)
}

fn parens(g: &mut G) -> Problem {
    use Tok::*;
    loop {
        let mut n = || Num(ri(g, 2, 12));
        let (a, b, c, d) = (n(), n(), n(), n());
        let toks = match ri(g, 0, 3) {
            0 => vec![L, a, Op('+'), b, R, Op('×'), c],
            1 => vec![a, Op('×'), L, b, Op('−'), c, R],
            2 => vec![L, a, Op('+'), b, R, Op('÷'), c, Op('+'), d],
            _ => vec![a, Op('−'), L, b, Op('−'), c, R, Op('×'), d],
        };
        if let Some((v, steps)) = solve(&toks)
            && v <= 200
            && steps.iter().all(|s| s.value >= 0)
        {
            return expr_problem(&toks, v, &steps, None);
        }
    }
}

fn word(g: &mut G) -> Problem {
    let name = pick(g, &NAMES);
    match ri(g, 0, 5) {
        0 => {
            let (a, b) = (ri(g, 3, 8), ri(g, 4, 12));
            let c = ri(g, 2, a * b - 2);
            let item = pick(g, &["mangoes", "candies", "marbles", "stickers"]);
            problem(
                format!("{name} has {a} bags with {b} {item} in each bag. {name} gives away {c}. How many {item} are left?"),
                Answer::Int(a * b - c),
                vec![
                    format!("First find how many {item} there are in all: {a} bags of {b}."),
                    format!("{a} × {b} = {}. Then take away the {c} given away.", a * b),
                    format!("{a} × {b} − {c} = {} − {c} = {}", a * b, a * b - c),
                ],
            )
        }
        1 => {
            let (a, b) = (ri(g, 2, 6), ri(g, 5, 25));
            let cost = a * b;
            let paid = if cost < 100 { 100 } else { (cost / 100 + 1) * 100 };
            problem(
                format!("{name} buys {a} pencils at ₱{b} each and pays with ₱{paid}. How much change does {name} get?"),
                Answer::Int(paid - cost),
                vec![
                    "Change = what you paid − what it cost.".to_string(),
                    format!("The pencils cost {a} × ₱{b} = ₱{cost}."),
                    format!("₱{paid} − ₱{cost} = ₱{}", paid - cost),
                ],
            )
        }
        2 => {
            let (b, k) = (ri(g, 2, 9), ri(g, 2, 9));
            let item = pick(g, &["cookies", "pandesal", "crayons", "cards"]);
            problem(
                format!("{} {item} are shared equally by {b} friends. How many does each friend get?", b * k),
                Answer::Int(k),
                vec![
                    "Sharing equally means dividing.".to_string(),
                    format!("{} ÷ {b} = ?  Think: {b} × ? = {}", b * k, b * k),
                    format!("{b} × {k} = {}, so each friend gets {k}.", b * k),
                ],
            )
        }
        3 => {
            let a = ri(g, 8, 20);
            let b = ri(g, 2, a - 1);
            let c = ri(g, 1, 9);
            problem(
                format!("A jeepney has {a} passengers. At the next stop, {b} get off and {c} get on. How many passengers are there now?"),
                Answer::Int(a - b + c),
                vec![
                    "Getting off means subtract. Getting on means add.".to_string(),
                    format!("{a} − {b} = {}", a - b),
                    format!("{} + {c} = {}", a - b, a - b + c),
                ],
            )
        }
        4 => {
            let (a, b, c) = (ri(g, 5, 15), ri(g, 3, 7), ri(g, 2, 20));
            problem(
                format!("{name} reads {a} pages every day for {b} days, then {c} more pages. How many pages did {name} read in all?"),
                Answer::Int(a * b + c),
                vec![
                    format!("{a} pages every day for {b} days is {b} groups of {a}."),
                    format!("{a} × {b} = {}", a * b),
                    format!("{} + {c} = {}", a * b, a * b + c),
                ],
            )
        }
        _ => {
            let (b, c) = (ri(g, 2, 5), ri(g, 5, 20));
            let have = (b * c / 50 + 1) * 50;
            problem(
                format!("{name} has ₱{have}. {name} buys {b} snacks at ₱{c} each and saves the rest. How much is saved?"),
                Answer::Int(have - b * c),
                vec![
                    "First find how much the snacks cost.".to_string(),
                    format!("{b} × ₱{c} = ₱{}", b * c),
                    format!("₱{have} − ₱{} = ₱{}", b * c, have - b * c),
                ],
            )
        }
    }
}

fn shapes(g: &mut G) -> Problem {
    let (w, h) = (ri(g, 2, 15), ri(g, 2, 12));
    match ri(g, 0, 2) {
        0 => problem(
            format!("A rectangle is {w} cm long and {h} cm wide. What is its area in square cm?"),
            Answer::Int(w * h),
            vec![
                "Area is how many squares cover the shape.".to_string(),
                "Area of a rectangle = length × width.".to_string(),
                format!("{w} × {h} = {} square cm", w * h),
            ],
        ),
        1 => problem(
            format!("A rectangle is {w} cm long and {h} cm wide. What is its perimeter in cm?"),
            Answer::Int(2 * (w + h)),
            vec![
                "Perimeter is the distance all the way around.".to_string(),
                format!("Add all four sides: {w} + {h} + {w} + {h}."),
                format!("2 × ({w} + {h}) = 2 × {} = {} cm", w + h, 2 * (w + h)),
            ],
        ),
        _ => {
            let b = 2 * ri(g, 2, 8);
            problem(
                format!("A triangle has a base of {b} cm and a height of {h} cm. What is its area in square cm?"),
                Answer::Int(b * h / 2),
                vec![
                    "A triangle is half of a rectangle.".to_string(),
                    "Area of a triangle = base × height ÷ 2.".to_string(),
                    format!("{b} × {h} = {}, and {} ÷ 2 = {}", b * h, b * h, b * h / 2),
                ],
            )
        }
    }
}

// ───────────────────────── World 4: Fraction Falls ─────────────────────────

fn simplify(g: &mut G) -> Problem {
    let (n, d) = loop {
        let d = ri(g, 2, 12);
        let n = ri(g, 1, d - 1);
        if gcd(n, d) == 1 {
            break (n, d);
        }
    };
    let k = ri(g, 2, 6);
    let (bn, bd) = (n * k, d * k);
    problem(
        format!("Simplify {}", mfr(bn, bd)),
        Answer::frac(bn, bd),
        vec![
            format!("Find the biggest number that divides both {bn} and {bd}."),
            format!("The greatest common factor of {bn} and {bd} is {k}."),
            format!("{bn} ÷ {k} = {n} and {bd} ÷ {k} = {d}, so\n${} = {}$", frac(bn, bd), frac(n, d)),
        ],
    )
}

fn addlike(g: &mut G) -> Problem {
    let d = ri(g, 3, 12);
    let (mut a, mut b) = (ri(g, 1, d - 1), ri(g, 1, d - 1));
    let plus = g.random_bool(0.6) || a == b;
    if !plus && a < b {
        (a, b) = (b, a);
    }
    let (s, op) = if plus { (a + b, '+') } else { (a - b, '−') };
    let mut p = problem(
        format!("${} {op} {}$ = ?", frac(a, d), frac(b, d)),
        Answer::frac(s, d),
        vec![
            format!("The bottoms are the same, so keep {d} on the bottom."),
            format!("{} the tops: {a} {op} {b} = {s}, so you have {}.", if plus { "Add" } else { "Subtract" }, mfr(s, d)),
            simplify_hint(s, d),
        ],
    );
    if d <= 10 && a + b <= d {
        p.visual = Some(format!("{}{}   {op}   {}{}", "🟦".repeat(a as usize), "⬜".repeat((d - a) as usize), "🟦".repeat(b as usize), "⬜".repeat((d - b) as usize)));
    }
    p
}

fn two_fracs(g: &mut G) -> (i64, i64, i64, i64) {
    let b = ri(g, 2, 10);
    let d = loop {
        let d = ri(g, 2, 10);
        if d != b {
            break d;
        }
    };
    (ri(g, 1, b - 1), b, ri(g, 1, d - 1), d)
}

fn addunlike(g: &mut G) -> Problem {
    let (mut a, mut b, mut c, mut d) = two_fracs(g);
    let plus = g.random_bool(0.6);
    if !plus && a * d < c * b {
        (a, b, c, d) = (c, d, a, b);
    }
    let l = lcm(b, d);
    let (x, y) = (a * l / b, c * l / d);
    let (s, op) = if plus { (x + y, '+') } else { (x - y, '−') };
    problem(
        format!("${} {op} {}$ = ?", frac(a, b), frac(c, d)),
        Answer::frac(s, l),
        vec![
            format!("The bottoms are different. Find a common denominator: the smallest number both {b} and {d} go into is {l}."),
            format!(
                "Rename them:\n${} = {}$ and ${} = {}$\nNow {x} {op} {y} = {s}, so you have {}.",
                frac(a, b),
                frac(x, l),
                frac(c, d),
                frac(y, l),
                mfr(s, l)
            ),
            simplify_hint(s, l),
        ],
    )
}

fn mulfrac(g: &mut G) -> Problem {
    let (a, b, c, d) = two_fracs(g);
    problem(
        format!("${} × {}$ = ?", frac(a, b), frac(c, d)),
        Answer::frac(a * c, b * d),
        vec![
            "Multiplying fractions is easy: tops times tops, bottoms times bottoms.".to_string(),
            format!("{a} × {c} = {} and {b} × {d} = {}, so {}.", a * c, b * d, mfr(a * c, b * d)),
            simplify_hint(a * c, b * d),
        ],
    )
}

fn divfrac(g: &mut G) -> Problem {
    let (a, b, c, d) = two_fracs(g);
    problem(
        format!("${} ÷ {}$ = ?", frac(a, b), frac(c, d)),
        Answer::frac(a * d, b * c),
        vec![
            "Keep, Change, Flip: keep the first fraction, change ÷ to ×, flip the second one.".to_string(),
            format!("${} × {} = {}$", frac(a, b), frac(d, c), frac(a * d, b * c)),
            simplify_hint(a * d, b * c),
        ],
    )
}

fn decaddsub(g: &mut G) -> Problem {
    let mut a = ri(g, 10, 999);
    let mut b = ri(g, 10, 999);
    if g.random_bool(0.5) {
        b = b / 10 * 10; // a tenths number next to a hundredths number
    }
    let plus = g.random_bool(0.5);
    if !plus && a < b {
        (a, b) = (b, a);
    }
    let (s, op) = if plus { (a + b, '+') } else { (a - b, '−') };
    let pad = |h: i64| format!("{}.{:02}", h / 100, h % 100);
    problem(
        format!("{} {op} {} = ?", show_dec(a), show_dec(b)),
        Answer::Dec(s),
        vec![
            "Line up the decimal points, then work like whole numbers.".to_string(),
            format!("Give them the same number of places: {} {op} {}", pad(a), pad(b)),
            format!("{} {op} {} = {}", pad(a), pad(b), show_dec(s)),
        ],
    )
}

fn decmul(g: &mut G) -> Problem {
    let a = ri(g, 2, 99); // tenths
    let (b_text, b_raw, places, hundredths) = if g.random_bool(0.5) {
        let b = ri(g, 2, 9);
        (b.to_string(), b, 1, a * b * 10)
    } else {
        let b = ri(g, 2, 9);
        (show_dec(b * 10), b, 2, a * b)
    };
    let p = a * b_raw;
    problem(
        format!("{} × {b_text} = ?", show_dec(a * 10)),
        Answer::Dec(hundredths),
        vec![
            format!("Forget the decimal points for now: {a} × {b_raw} = {p}."),
            format!("Count the decimal places in the question: {places} in all."),
            format!("Put the point back {places} place{} from the right: {}", if places > 1 { "s" } else { "" }, show_dec(hundredths)),
        ],
    )
}

fn percent(g: &mut G) -> Problem {
    let pct = pick(g, &[10, 20, 25, 50, 75, 5, 30, 40]);
    let step = 100 / gcd(pct, 100);
    let base = step * ri(g, 1, 400 / step).max(1);
    let base = if base < 20 { base * 10 } else { base };
    let v = base * pct / 100;
    let shortcut = match pct {
        10 => format!("10% is one tenth: {base} ÷ 10 = {}", base / 10),
        50 => format!("50% is one half: {base} ÷ 2 = {v}"),
        25 => format!("25% is one quarter: {base} ÷ 4 = {v}"),
        75 => format!("75% is three quarters: {base} ÷ 4 = {}, then × 3", base / 4),
        5 => format!("5% is half of 10%: 10% of {base} is {}, half of that is…", base / 10),
        _ => format!("{pct}% is {} × 10%. 10% of {base} is {}.", pct / 10, base as f64 / 10.0),
    };
    problem(
        format!("What is {pct}% of {base}?"),
        Answer::Int(v),
        vec![format!("{pct}% means {pct} out of every 100."), shortcut, format!("{base} × {pct} ÷ 100 = {v}")],
    )
}

// ───────────────────────── World 5: Integer Volcano ─────────────────────────

fn intaddsub(g: &mut G) -> Problem {
    let a = nonzero(g, -15, 15);
    let b = nonzero(g, -15, 15);
    let plus = g.random_bool(0.5);
    let (op, v) = if plus { ('+', a + b) } else { ('−', a - b) };
    let b2 = if plus { b } else { -b };
    let rule = if (a < 0) == (b2 < 0) {
        format!("Same signs: add the sizes ({} + {}) and keep the sign.", a.abs(), b2.abs())
    } else {
        format!("Different signs: subtract the sizes ({} − {}) and keep the sign of the bigger one.", a.abs().max(b2.abs()), a.abs().min(b2.abs()))
    };
    let first = if plus {
        "Picture a number line. Adding a positive moves right; adding a negative moves left.".to_string()
    } else {
        format!("Subtracting is adding the opposite: {} − {} becomes {} + {}", minus(a), wrap(b), minus(a), wrap(-b))
    };
    problem(
        format!("{} {op} {} = ?", minus(a), wrap(b)),
        Answer::Int(v),
        vec![first, rule, format!("{} + {} = {}", minus(a), wrap(b2), minus(v))],
    )
}

fn intmuldiv(g: &mut G) -> Problem {
    let a = nonzero(g, -12, 12);
    let b = nonzero(g, -12, 12);
    let (q, op, l, r) = if g.random_bool(0.5) { (a * b, '×', a, b) } else { (b, '÷', a * b, a) };
    let sign = if (l < 0) == (r < 0) { "Same signs → the answer is positive." } else { "Different signs → the answer is negative." };
    problem(
        format!("{} {op} {} = ?", minus(l), wrap(r)),
        Answer::Int(q),
        vec![
            format!("Ignore the signs first: {} {op} {} = {}", l.abs(), r.abs(), q.abs()),
            sign.to_string(),
            format!("{} {op} {} = {}", minus(l), wrap(r), minus(q)),
        ],
    )
}

fn powroot(g: &mut G) -> Problem {
    if g.random_bool(0.5) {
        let (a, n) = match ri(g, 0, 2) {
            0 => (ri(g, 2, 12), 2),
            1 => (ri(g, 2, 5), 3),
            _ => (2, ri(g, 4, 6)),
        };
        let v = a.pow(n as u32);
        let times = vec![a.to_string(); n as usize].join(" × ");
        problem(
            format!("{a}{} = ?", sup(n)),
            Answer::Int(v),
            vec![
                format!("The small {n} is an exponent: multiply {a} by itself {n} times."),
                format!("{a}{} = {times}", sup(n)),
                format!("{times} = {v}"),
            ],
        )
    } else {
        let k = ri(g, 2, 15);
        problem(
            format!("${}$ = ?", sqrt(k * k)),
            Answer::Int(k),
            vec![
                format!("√ asks: what number times itself makes {}?", k * k),
                format!("Try some: {} × {} = {}, {} × {} = {} …", k - 1, k - 1, (k - 1) * (k - 1), k + 1, k + 1, (k + 1) * (k + 1)),
                format!("{k} × {k} = {}, so √{} = {k}", k * k, k * k),
            ],
        )
    }
}

fn pemdas(g: &mut G) -> Problem {
    use Tok::*;
    loop {
        let mut n = |lo, hi| Num(nonzero(g, lo, hi));
        let (a, b, c, d) = (n(-9, 12), n(-6, 9), n(2, 9), n(2, 4));
        let e = Num(ri(g, 2, 3));
        let toks = match ri(g, 0, 4) {
            0 => vec![a, Op('+'), b, Op('^'), Num(2), Op('×'), c],
            1 => vec![L, a, Op('−'), b, R, Op('^'), Num(2), Op('÷'), c],
            2 => vec![a, Op('×'), L, b, Op('+'), c, R, Op('−'), d, Op('^'), e],
            3 => vec![L, Num(-ri(g, 2, 5)), R, Op('^'), e, Op('+'), a, Op('×'), c],
            _ => vec![a, Op('−'), c, Op('×'), L, b, Op('−'), d, R],
        };
        if let Some((v, steps)) = solve(&toks)
            && v.abs() <= 300
        {
            return expr_problem(&toks, v, &steps, None);
        }
    }
}

// ───────────────────────── World 6: Algebra Peak ─────────────────────────

fn onestep(g: &mut G) -> Problem {
    let x = ri(g, -10, 20);
    let a = ri(g, 2, 12);
    let (eq, undo, work, ans) = match ri(g, 0, 3) {
        0 => (format!("x + {a} = {}", minus(x + a)), format!("subtract {a} from both sides"), format!("x = {} − {a}", minus(x + a)), x),
        1 => (format!("x − {a} = {}", minus(x - a)), format!("add {a} to both sides"), format!("x = {} + {a}", minus(x - a)), x),
        2 => (format!("{a}x = {}", minus(a * x)), format!("divide both sides by {a}"), format!("x = {} ÷ {a}", minus(a * x)), x),
        _ => (format!("x ÷ {a} = {}", minus(x)), format!("multiply both sides by {a}"), format!("x = {} × {a}", minus(x)), x * a),
    };
    problem(
        format!("Solve for x:\n{eq}"),
        Answer::Int(ans),
        vec![
            "Get x alone. Do the opposite of what is being done to x, on both sides.".to_string(),
            format!("Here, {undo}: {work}"),
            format!("x = {}", minus(ans)),
        ],
    )
}

fn twostep(g: &mut G) -> Problem {
    let x = ri(g, -10, 10);
    let a = nonzero(g, -6, 9);
    let a = if a == 1 || a == -1 { 2 } else { a };
    let b = nonzero(g, -20, 20);
    let c = a * x + b;
    problem(
        format!("Solve for x:\n{}{} = {}", coef(a, "x"), signed(b), minus(c)),
        Answer::Int(x),
        vec![
            format!("First undo the {}: {} {} on both sides.", signed(b).trim(), if b > 0 { "subtract" } else { "add" }, b.abs()),
            format!("{} = {} {} {} = {}. Now divide both sides by {}.", coef(a, "x"), minus(c), if b > 0 { '−' } else { '+' }, b.abs(), minus(c - b), minus(a)),
            format!("x = {} ÷ {} = {}", minus(c - b), wrap(a), minus(x)),
        ],
    )
}

fn bothsides(g: &mut G) -> Problem {
    let x = ri(g, -9, 9);
    let (a, c) = loop {
        let (a, c) = (ri(g, 2, 9), ri(g, 1, 8));
        if a != c {
            break (a, c);
        }
    };
    let b = nonzero(g, -15, 15);
    let d = a * x + b - c * x;
    // Gather the x's on the side with more of them, so x keeps a positive number in front.
    let (k, gather) = if a > c { (a - c, format!("subtract {} from both sides", coef(c, "x"))) } else { (c - a, format!("subtract {} from both sides", coef(a, "x"))) };
    let (left, rhs, other, move_text) = if a > c {
        (format!("{}{}", coef(k, "x"), signed(b)), d - b, minus(d), minus(b))
    } else {
        (minus(b), b - d, format!("{}{}", coef(k, "x"), if d == 0 { String::new() } else { signed(d) }), minus(d))
    };
    problem(
        format!("Solve for x:\n{}{} = {}{}", coef(a, "x"), signed(b), coef(c, "x"), if d == 0 { String::new() } else { signed(d) }),
        Answer::Int(x),
        vec![
            format!("Get all the x's on one side: {gather}."),
            format!("{left} = {other}. Now move the {move_text} to the other side: {} = {}", coef(k, "x"), minus(rhs)),
            format!("x = {} ÷ {k} = {}", minus(rhs), minus(x)),
        ],
    )
}

fn evaluate(g: &mut G) -> Problem {
    use Tok::*;
    loop {
        let x = nonzero(g, -6, 8);
        let (a, b) = (ri(g, 2, 7), nonzero(g, -12, 12));
        let (written, toks) = match ri(g, 0, 3) {
            0 => (format!("{}x²{}", a, signed(b)), vec![Num(a), Op('×'), Num(x), Op('^'), Num(2), Op(if b < 0 { '−' } else { '+' }), Num(b.abs())]),
            1 => (format!("{a}(x{})", signed(b)), vec![Num(a), Op('×'), L, Num(x), Op(if b < 0 { '−' } else { '+' }), Num(b.abs()), R]),
            2 => (format!("x²{}{}", term(-a, "x"), signed(b)), vec![Num(x), Op('^'), Num(2), Op('−'), Num(a), Op('×'), Num(x), Op(if b < 0 { '−' } else { '+' }), Num(b.abs())]),
            _ => {
                let y = nonzero(g, -5, 6);
                (
                    format!("{}x{} when y = {}", a, term(b, "y"), minus(y)),
                    vec![Num(a), Op('×'), Num(x), Op(if b < 0 { '−' } else { '+' }), Num(b.abs()), Op('×'), Num(y)],
                )
            }
        };
        let Some((v, steps)) = solve(&toks) else { continue };
        if v.abs() > 400 {
            continue;
        }
        let (expr_text, when) = match written.split_once(" when ") {
            Some((e, w)) => (e.to_string(), format!(" and {w}")),
            None => (written.clone(), String::new()),
        };
        let mut p = expr_problem(&toks, v, &steps, Some(format!("If x = {}{when}, what is\n{expr_text} ?", minus(x))));
        p.hints[0] = format!("Put the numbers in place of the letters: {}", show(&toks));
        return p;
    }
}

fn proportion(g: &mut G) -> Problem {
    let (a, b) = loop {
        let (a, b) = (ri(g, 1, 9), ri(g, 2, 12));
        if a != b {
            break (a, b);
        }
    };
    let k = ri(g, 2, 9);
    if g.random_bool(0.5) {
        problem(
            format!("Solve for x:\n${} = {}$", frac(a, b), frac("x", b * k)),
            Answer::Int(a * k),
            vec![
                "Equal fractions: whatever you do to the bottom, do to the top.".to_string(),
                format!("{b} × ? = {}. It's {k}!", b * k),
                format!("x = {a} × {k} = {}", a * k),
            ],
        )
    } else {
        problem(
            format!("Solve for x:\n${} = {}$", frac(a, b), frac(a * k, "x")),
            Answer::Int(b * k),
            vec![
                "Equal fractions: whatever you do to the top, do to the bottom.".to_string(),
                format!("{a} × ? = {}. It's {k}!", a * k),
                format!("x = {b} × {k} = {}", b * k),
            ],
        )
    }
}

fn percentword(g: &mut G) -> Problem {
    let pct = pick(g, &[10, 20, 25, 30, 40, 50, 15]);
    let step = 100 / gcd(pct, 100);
    let price = step * 20 * ri(g, 1, 10);
    let part = price * pct / 100;
    match ri(g, 0, 2) {
        0 => {
            let item = pick(g, &["bag", "pair of shoes", "toy robot", "jacket"]);
            problem(
                format!("A ₱{price} {item} is {pct}% off. What is the sale price?"),
                Answer::Int(price - part),
                vec![
                    format!("First find the discount: {pct}% of ₱{price}."),
                    format!("₱{price} × {pct} ÷ 100 = ₱{part} off."),
                    format!("Sale price = ₱{price} − ₱{part} = ₱{}", price - part),
                ],
            )
        }
        1 => problem(
            format!("A ₱{price} meal has a {pct}% service charge added. What is the total?"),
            Answer::Int(price + part),
            vec![
                format!("Find the charge: {pct}% of ₱{price}."),
                format!("₱{price} × {pct} ÷ 100 = ₱{part}."),
                format!("Total = ₱{price} + ₱{part} = ₱{}", price + part),
            ],
        ),
        _ => {
            let whole = pick(g, &[10, 20, 25, 50]);
            let got = ri(g, 1, whole - 1);
            problem(
                format!("You got {got} out of {whole} on a quiz. What percent is that?"),
                Answer::Int(got * 100 / whole),
                vec![
                    "Percent means \"out of 100\". Turn the score into a fraction out of 100.".to_string(),
                    format!("{whole} × {} = 100, so multiply the top by {} too.", 100 / whole, 100 / whole),
                    format!("{got} × {} = {}, so {got}/{whole} = {}/100 = {}%", 100 / whole, got * 100 / whole, got * 100 / whole, got * 100 / whole),
                ],
            )
        }
    }
}

fn pythagoras(g: &mut G) -> Problem {
    let (a, b, c) = pick(g, &[(3, 4, 5), (5, 12, 13), (8, 15, 17), (7, 24, 25), (6, 8, 10), (9, 12, 15), (20, 21, 29)]);
    if g.random_bool(0.6) {
        with(
right(b as f64, a as f64, "", b.to_string(), a.to_string(), "?"),
given(
problem(
            format!("A right triangle has legs {a} and {b}. How long is the longest side (the hypotenuse)?"),
            Answer::Int(c),
            vec![
                "Pythagoras, where c is the longest side:\n$a^2 + b^2 = c^2$".to_string(),
                format!("{a}² + {b}² = {} + {} = {}", a * a, b * b, c * c),
                format!("$c = {}$ = {c}", sqrt(c * c)),
            ],
        ),
&[("a", a), ("b", b)]))
    } else {
        with(
right(b as f64, a as f64, "", "?", a.to_string(), c.to_string()),
given(
problem(
            format!("A right triangle has a hypotenuse of {c} and one leg of {a}. How long is the other leg?"),
            Answer::Int(b),
            vec![
                "Pythagoras: a² + b² = c². To find a leg:\n$b^2 = c^2 − a^2$".to_string(),
                format!("{c}² − {a}² = {} − {} = {}", c * c, a * a, b * b),
                format!("$b = {}$ = {b}", sqrt(b * b)),
            ],
        ),
&[("c", c), ("a", a)]))
    }
}

fn slope(g: &mut G) -> Problem {
    let (x1, y1, x2, y2) = loop {
        let (x1, y1, x2, y2) = (ri(g, -8, 8), ri(g, -8, 8), ri(g, -8, 8), ri(g, -8, 8));
        if x1 != x2 {
            break (x1, y1, x2, y2);
        }
    };
    let (dy, dx) = (y2 - y1, x2 - x1);
    problem(
        format!("What is the slope of the line through ({}, {}) and ({}, {})?", minus(x1), minus(y1), minus(x2), minus(y2)),
        Answer::frac(dy, dx),
        vec![
            "Slope = rise ÷ run:\n$slope = \\frac{y_2 − y_1}{x_2 − x_1}$".to_string(),
            format!(
                "${} = {}$",
                frac(format!("{} − {}", minus(y2), wrap(y1)), format!("{} − {}", minus(x2), wrap(x1))),
                frac(minus(dy), minus(dx))
            ),
            if gcd(dy, dx) == 1 && dx > 0 {
                format!("{} is already simplest, so the slope is {}", mfr(dy, dx), mfr(dy, dx))
            } else {
                let (n, d) = reduce(dy, dx);
                format!("Simplify:\n${} = {}$", frac(minus(dy), minus(dx)), fm(n, d))
            },
        ],
    )
}

fn systems(g: &mut G) -> Problem {
    let (x, y) = (ri(g, -8, 9), ri(g, -8, 9));
    if g.random_bool(0.5) {
        let (s, d) = (x + y, x - y);
        problem(
            format!("Find x and y:\nx + y = {}\nx − y = {}", minus(s), minus(d)),
            Answer::Pair(x, y),
            vec![
                "Add the two equations together: the y's cancel out!".to_string(),
                format!("2x = {} + {} = {}, so x = {}", minus(s), wrap(d), minus(s + d), minus(x)),
                format!("Put x back in: {} + y = {}, so y = {}", minus(x), minus(s), minus(y)),
            ],
        )
    } else {
        let (m, n) = loop {
            let (m, n) = (nonzero(g, -4, 5), nonzero(g, -4, 5));
            if m != n {
                break (m, n);
            }
        };
        let (b1, b2) = (y - m * x, y - n * x);
        let line = |k: i64, b: i64| format!("y = {}{}", coef(k, "x"), if b == 0 { String::new() } else { signed(b) });
        problem(
            format!("Find x and y:\n{}\n{}", line(m, b1), line(n, b2)),
            Answer::Pair(x, y),
            vec![
                "Both equal y, so set them equal to each other.".to_string(),
                format!("{}{} = {}{} → {} = {}, so x = {}", coef(m, "x"), signed(b1), coef(n, "x"), signed(b2), coef(m - n, "x"), minus(b2 - b1), minus(x)),
                format!("Put x back in: y = {} × {}{} = {}", minus(m), wrap(x), signed(b1), minus(y)),
            ],
        )
    }
}

fn quadratic(g: &mut G) -> Problem {
    let (r1, r2) = (nonzero(g, -9, 9), nonzero(g, -9, 9));
    let (b, c) = (-(r1 + r2), r1 * r2);
    let factor = |r: i64| format!("(x{})", signed(-r));
    problem(
        format!("Solve for x:\nx²{}{} = 0", term(b, "x"), signed(c)),
        Answer::Roots(r1, r2),
        vec![
            format!("Find two numbers that multiply to {} and add to {}.", minus(c), minus(b)),
            format!("They are {} and {}: {}{} = 0", minus(-r1), minus(-r2), factor(r1), factor(r2)),
            format!("A product is 0 when one part is 0: x = {} or x = {}", minus(r1), minus(r2)),
        ],
    )
}

pub static WORLDS: &[World] = &[
    World {
        name: "Number Garden",
        emoji: "🌻",
        grades: "Grades 1–2",
        levels: &[
            Level { id: "add10", title: "Adding to 10", make: add10, lesson: "Adding means putting groups together.\n\nCount the first group, then keep counting for the second group. The last number you say is the answer.\n\nTip: start from the bigger number so you count less!" },
            Level { id: "add20", title: "Make a Ten", make: add20, lesson: "Ten is a friendly number.\n\nTo add 8 + 5, give 2 from the 5 to the 8 to make 10. Now it's 10 + 3, which is 13.\n\nLook for the numbers that make 10: 1+9, 2+8, 3+7, 4+6, 5+5." },
            Level { id: "sub20", title: "Taking Away", make: sub20, lesson: "Subtracting means taking away.\n\nStart at the big number and count back. Or think of adding: 13 − 5 asks \"5 plus what makes 13?\"" },
            Level { id: "missing", title: "Mystery Number", make: missing, lesson: "A mystery number is hiding!\n\n7 + ? = 12 asks: what do I add to 7 to get 12?\n\nYou can find it by subtracting: 12 − 7 = 5." },
            Level { id: "add2d", title: "Two-Digit Adding", make: add2d, lesson: "Big numbers have tens and ones. 47 is 4 tens and 7 ones.\n\nAdd the ones first. If they make 10 or more, carry 1 ten over to the tens.\n\nThen add the tens." },
            Level { id: "sub2d", title: "Two-Digit Taking Away", make: sub2d, lesson: "Subtract the ones first, then the tens.\n\nIf the top ones digit is too small, borrow: take 1 ten and turn it into 10 ones." },
        ],
    },
    World {
        name: "Times Jungle",
        emoji: "🐒",
        grades: "Grade 3",
        levels: &[
            Level { id: "times2to5", title: "Times Tables 2–5", make: times2to5, lesson: "Multiplying is fast adding.\n\n3 × 4 means 3 groups of 4: 4 + 4 + 4 = 12.\n\nYou can also skip count: 4, 8, 12." },
            Level { id: "times6to10", title: "Times Tables 6–10", make: times6to10, lesson: "Tricks for the big tables:\n\n• × 10: add a zero.\n• × 9: do × 10, then take away one group.\n• × 5: do × 10, then take half.\n\nOrder doesn't matter: 7 × 6 is the same as 6 × 7." },
            Level { id: "divfacts", title: "Sharing (Division)", make: divfacts, lesson: "Dividing is sharing into equal groups.\n\n12 ÷ 3 asks: how many 3s fit in 12?\n\nUse your times tables backwards: 3 × 4 = 12, so 12 ÷ 3 = 4." },
            Level { id: "mult2x1", title: "Bigger Multiplying", make: mult2x1, lesson: "Break the big number into tens and ones.\n\n34 × 6 = 30 × 6 + 4 × 6 = 180 + 24 = 204." },
            Level { id: "longdiv", title: "Leftovers (Remainders)", make: longdiv, lesson: "Sometimes things don't share evenly. What is left over is the remainder.\n\n17 ÷ 5: three 5s make 15, and 2 is left. So 17 ÷ 5 = 3 r 2.\n\nThe remainder is always smaller than the number you divide by." },
            Level { id: "mult2x2", title: "Two-Digit × Two-Digit", make: mult2x2, lesson: "Break the second number into tens and ones, multiply each part, then add.\n\n23 × 14 = 23 × 10 + 23 × 4 = 230 + 92 = 322." },
        ],
    },
    World {
        name: "Column City",
        emoji: "🏙️",
        grades: "Grades 2–4",
        levels: &[
            Level { id: "col_add2", title: "Stack & Add: Tens", make: col_add2, lesson: "Write the numbers one on top of the other, lining up the ones, tens.\n\nStart on the RIGHT with the ones. Add each column. If it makes 10 or more, write the ones digit and carry the 1 to the top of the next column.\n\nType your answer from right to left. The small boxes on top are for your carries." },
            Level { id: "col_sub2", title: "Stack & Subtract: Tens", make: col_sub2, lesson: "Write the bigger number on top, lining up the places.\n\nStart on the RIGHT. Take the bottom digit away from the top digit. If the top digit is too small, borrow: take 1 from the next column, and it becomes 10 more in this one.\n\nType your answer from right to left." },
            Level { id: "col_mul2", title: "Stack & Multiply: Tens", make: col_mul2, lesson: "Write the big number on top and the small one under its ones digit.\n\nStart on the RIGHT. Multiply the bottom number by each top digit. Write the ones digit and carry the tens. Remember to add the carry AFTER you multiply!\n\nType your answer from right to left." },
            Level { id: "col_div2", title: "Long Division: Tens", make: col_div2, lesson: "Long division: the number being shared goes inside the bracket, the divisor outside.\n\nWork from the LEFT:\n1. Divide — how many fit?\n2. Multiply\n3. Subtract\n4. Bring down the next digit\nRepeat!\n\nType the answer on top, left to right. Whatever is left at the end goes in the r (remainder) box." },
            Level { id: "col_add3", title: "Stack & Add: Hundreds", make: col_add3, lesson: "Write the numbers one on top of the other, lining up the ones, tens, hundreds.\n\nStart on the RIGHT with the ones. Add each column. If it makes 10 or more, write the ones digit and carry the 1 to the top of the next column.\n\nType your answer from right to left. The small boxes on top are for your carries." },
            Level { id: "col_sub3", title: "Stack & Subtract: Hundreds", make: col_sub3, lesson: "Write the bigger number on top, lining up the places.\n\nStart on the RIGHT. Take the bottom digit away from the top digit. If the top digit is too small, borrow: take 1 from the next column, and it becomes 10 more in this one.\n\nType your answer from right to left." },
            Level { id: "col_mul3", title: "Stack & Multiply: Hundreds", make: col_mul3, lesson: "Write the big number on top and the small one under its ones digit.\n\nStart on the RIGHT. Multiply the bottom number by each top digit. Write the ones digit and carry the tens. Remember to add the carry AFTER you multiply!\n\nType your answer from right to left." },
            Level { id: "col_div3", title: "Long Division: Hundreds", make: col_div3, lesson: "Long division: the number being shared goes inside the bracket, the divisor outside.\n\nWork from the LEFT:\n1. Divide — how many fit?\n2. Multiply\n3. Subtract\n4. Bring down the next digit\nRepeat!\n\nType the answer on top, left to right. Whatever is left at the end goes in the r (remainder) box." },
            Level { id: "col_add4", title: "Stack & Add: Thousands", make: col_add4, lesson: "Write the numbers one on top of the other, lining up the ones, tens, hundreds and thousands.\n\nStart on the RIGHT with the ones. Add each column. If it makes 10 or more, write the ones digit and carry the 1 to the top of the next column.\n\nType your answer from right to left. The small boxes on top are for your carries." },
            Level { id: "col_sub4", title: "Stack & Subtract: Thousands", make: col_sub4, lesson: "Write the bigger number on top, lining up the places.\n\nStart on the RIGHT. Take the bottom digit away from the top digit. If the top digit is too small, borrow: take 1 from the next column, and it becomes 10 more in this one.\n\nType your answer from right to left." },
            Level { id: "col_mul4", title: "Stack & Multiply: Thousands", make: col_mul4, lesson: "Write the big number on top and the small one under its ones digit.\n\nStart on the RIGHT. Multiply the bottom number by each top digit. Write the ones digit and carry the tens. Remember to add the carry AFTER you multiply!\n\nType your answer from right to left." },
            Level { id: "col_div4", title: "Long Division: Thousands", make: col_div4, lesson: "Long division: the number being shared goes inside the bracket, the divisor outside.\n\nWork from the LEFT:\n1. Divide — how many fit?\n2. Multiply\n3. Subtract\n4. Bring down the next digit\nRepeat!\n\nType the answer on top, left to right. Whatever is left at the end goes in the r (remainder) box." },
        ],
    },
    World {
        name: "MDAS Castle",
        emoji: "🏰",
        grades: "Grades 4–5",
        levels: &[
            Level { id: "mdas2", title: "MDAS: Two Steps", make: mdas2, lesson: "When there's more than one operation, the order matters!\n\nMDAS: Multiply and Divide first (left to right), then Add and Subtract (left to right).\n\n5 + 3 × 2 = 5 + 6 = 11, not 16." },
            Level { id: "mdas3", title: "MDAS: Three Steps", make: mdas3, lesson: "Same rules, longer problems.\n\nGo across once doing all the × and ÷, from left to right. Then go across again doing + and −, from left to right.\n\nRewrite the problem after each step so you don't lose track." },
            Level { id: "parens", title: "Parentheses First", make: parens, lesson: "Parentheses ( ) are VIP: do what's inside them first.\n\n(2 + 3) × 4 = 5 × 4 = 20\n2 + 3 × 4 = 2 + 12 = 14\n\nSame numbers, different answers!" },
            Level { id: "word", title: "Story Problems", make: word, lesson: "Turn the story into a math sentence.\n\n• Read it twice.\n• What do we know? What do we need to find?\n• Words like \"each\" or \"every\" often mean ×; \"shared equally\" means ÷; \"left\" means −; \"in all\" means +." },
            Level { id: "shapes", title: "Area & Perimeter", make: shapes, lesson: "Perimeter: the distance around a shape. Add all the sides.\n\nArea: how many squares cover it.\n• Rectangle: length × width\n• Triangle: base × height ÷ 2" },
        ],
    },
    World {
        name: "Fraction Falls",
        emoji: "🌊",
        grades: "Grades 5–6",
        levels: &[
            Level { id: "simplify", title: "Simplest Form", make: simplify, lesson: "A fraction is a part of a whole.\n$\\frac{3}{4}$ means 3 parts out of 4.\n\nTo simplify, divide the top and bottom by the same number:\n$\\frac{6}{8} = \\frac{6 ÷ 2}{8 ÷ 2} = \\frac{3}{4}$\n\nUse the biggest number that divides both (the GCF) to finish in one step." },
            Level { id: "addlike", title: "Same Bottoms", make: addlike, lesson: "When the bottoms (denominators) are the same, just add or subtract the tops.\n\n$\\frac{2}{7} + \\frac{3}{7} = \\frac{5}{7}$\n\nThe bottom stays the same, because the pieces are the same size." },
            Level { id: "addunlike", title: "Different Bottoms", make: addunlike, lesson: "Different bottoms means different-sized pieces, so make them the same first.\n\n$\\frac{1}{2} + \\frac{1}{3}$: both 2 and 3 go into 6.\n$\\frac{1}{2} + \\frac{1}{3} = \\frac{3}{6} + \\frac{2}{6} = \\frac{5}{6}$" },
            Level { id: "mulfrac", title: "Multiplying Fractions", make: mulfrac, lesson: "Tops × tops, bottoms × bottoms.\n\n$\\frac{2}{3} × \\frac{3}{5} = \\frac{6}{15} = \\frac{2}{5}$\n\nThen simplify." },
            Level { id: "divfrac", title: "Dividing Fractions", make: divfrac, lesson: "Keep, Change, Flip!\n\n• Keep the first fraction.\n• Change ÷ into ×.\n• Flip the second fraction upside down.\n\n$\\frac{1}{2} ÷ \\frac{3}{4} = \\frac{1}{2} × \\frac{4}{3} = \\frac{4}{6} = \\frac{2}{3}$" },
            Level { id: "decaddsub", title: "Decimal + and −", make: decaddsub, lesson: "Line up the decimal points, one under the other.\n\nIf one number has fewer places, add zeros: 3.5 is the same as 3.50.\n\nThen add or subtract like whole numbers and keep the point in place." },
            Level { id: "decmul", title: "Decimal ×", make: decmul, lesson: "1. Multiply as if there were no decimal points.\n2. Count how many digits are after the points in the question.\n3. Put the point back that many places from the right.\n\n0.3 × 0.4: 3 × 4 = 12, two places → 0.12" },
            Level { id: "percent", title: "Percent", make: percent, lesson: "Percent means \"out of 100\".\n$25\\% = \\frac{25}{100} = \\frac{1}{4}$\n\nHandy ones:\n• 50% = half\n• 25% = a quarter\n• 10% = divide by 10\n\nTo find p% of a number: number × p ÷ 100." },
        ],
    },
    World {
        name: "Integer Volcano",
        emoji: "🌋",
        grades: "Grades 6–7",
        levels: &[
            Level { id: "intaddsub", title: "Negative Numbers + −", make: intaddsub, lesson: "Negative numbers are below zero, like a temperature of −3°.\n\n• Same signs: add the sizes, keep the sign. −3 + (−4) = −7\n• Different signs: subtract the sizes, keep the sign of the bigger one. −8 + 5 = −3\n• Subtracting a number is adding its opposite. 4 − (−2) = 4 + 2 = 6" },
            Level { id: "intmuldiv", title: "Negative Numbers × ÷", make: intmuldiv, lesson: "Multiply or divide the sizes, then decide the sign:\n\n• Same signs → positive. −3 × −4 = 12\n• Different signs → negative. −12 ÷ 3 = −4" },
            Level { id: "powroot", title: "Exponents & Roots", make: powroot, lesson: "An exponent says how many times to multiply a number by itself.\n\n5² = 5 × 5 = 25 (\"5 squared\")\n2³ = 2 × 2 × 2 = 8 (\"2 cubed\")\n\nA square root goes backwards:\n$\\sqrt{25}$ = 5, because 5 × 5 = 25." },
            Level { id: "pemdas", title: "PEMDAS Master", make: pemdas, lesson: "The full order of operations:\n\nP — Parentheses\nE — Exponents\nMD — Multiply & Divide, left to right\nAS — Add & Subtract, left to right\n\nCareful: (−3)² = 9, because the whole −3 is squared." },
        ],
    },
    World {
        name: "Algebra Peak",
        emoji: "🏔️",
        grades: "Grades 7–10",
        levels: &[
            Level { id: "onestep", title: "One-Step Equations", make: onestep, lesson: "An equation is a balance: both sides are equal.\n\nTo find x, undo what's done to it, and do the same thing to both sides.\n\nx + 5 = 12 → subtract 5 from both sides → x = 7" },
            Level { id: "twostep", title: "Two-Step Equations", make: twostep, lesson: "Undo things in reverse order: first the + or −, then the × or ÷.\n\n3x + 4 = 19\n3x = 15 (subtract 4)\nx = 5 (divide by 3)\n\nCheck: 3 × 5 + 4 = 19 ✔" },
            Level { id: "bothsides", title: "x on Both Sides", make: bothsides, lesson: "Gather all the x's on one side first.\n\n5x + 2 = 3x + 10\n2x + 2 = 10 (subtract 3x)\n2x = 8\nx = 4" },
            Level { id: "evaluate", title: "Plug It In", make: evaluate, lesson: "To evaluate an expression, replace each letter with its number, then use the order of operations.\n\n3x means 3 × x. x² means x × x.\n\nIf x = −2: x² = (−2) × (−2) = 4. Always put negative numbers in parentheses!" },
            Level { id: "proportion", title: "Proportions", make: proportion, lesson: "A proportion says two fractions are equal.\n\n$\\frac{2}{3} = \\frac{x}{12}$\nThe bottom went from 3 to 12 (× 4), so the top does too: x = 2 × 4 = 8.\n\nYou can also cross-multiply: 2 × 12 = 3 × x." },
            Level { id: "percentword", title: "Sales & Percents", make: percentword, lesson: "Real-life percents:\n\n• Discount: find the % of the price, then subtract it.\n• Service charge or tax: find the %, then add it.\n• Score to percent: make the fraction out of 100." },
            Level { id: "pythagoras", title: "Pythagoras", make: pythagoras, lesson: "In a right triangle, the two short sides (legs) a and b and the long side c (hypotenuse) follow:\n\na² + b² = c²\n\n3² + 4² = 9 + 16 = 25 = 5², so the hypotenuse is 5." },
            Level { id: "slope", title: "Slope of a Line", make: slope, lesson: "Slope tells how steep a line is: rise over run.\n\n$slope = \\frac{y_2 − y_1}{x_2 − x_1}$\n\nFrom (1, 2) to (3, 8):\n$\\frac{8 − 2}{3 − 1} = \\frac{6}{2}$ = 3\n\nGoing down from left to right gives a negative slope." },
            Level { id: "systems", title: "Two Equations", make: systems, lesson: "Two equations, two unknowns. Find the x and y that make both true.\n\n• Adding the equations can make one letter cancel out.\n• If both say y = …, set them equal to each other.\n\nThen put the value you found back in to get the other one." },
            Level { id: "quadratic", title: "Factoring Quadratics", make: quadratic, lesson: "To solve x² + bx + c = 0, find two numbers that multiply to c and add to b.\n\nx² + 5x + 6 = 0 → 2 and 3 (2 × 3 = 6, 2 + 3 = 5)\n(x + 2)(x + 3) = 0\nx = −2 or x = −3\n\nThere are usually two answers!" },
        ],
    },
    World {
        name: "Equation Galaxy",
        emoji: "🌌",
        grades: "Grades 9–11",
        levels: &[
            Level { id: "exponents", title: "Exponent Rules", make: advanced::exponents, lesson: "Exponents are a shortcut for repeated multiplying: x³ = x × x × x.\n\nThe rules:\n• Multiply, same base → add: x² · x³ = x⁵\n• Divide → subtract: x⁷ ÷ x³ = x⁴\n• Power of a power → multiply: (x²)³ = x⁶\n• x⁰ = 1, and a negative exponent means one over:\n$x^{-2} = \\frac{1}{x^2}$" },
            Level { id: "radicals", title: "Square Roots & Radicals", make: advanced::radicals, lesson: "A square root undoes squaring:\n$\\sqrt{49}$ = 7, because 7² = 49.\n\nTo simplify, pull out perfect squares:\n$\\sqrt{50} = \\sqrt{25} × \\sqrt{2} = 5\\sqrt{2}$\n\nRoots multiply together:\n$\\sqrt{2} × \\sqrt{8} = \\sqrt{16}$ = 4" },
            Level { id: "expand", title: "Expanding (FOIL)", make: advanced::expand, lesson: "To multiply two brackets, every term meets every term. FOIL helps you remember:\n\nFirst · Outer · Inner · Last\n\n(x + 2)(x + 3) = x² + 3x + 2x + 6 = x² + 5x + 6\n\nThe middle number is the sum (2 + 3); the last is the product (2 × 3)." },
            Level { id: "quadformula", title: "The Quadratic Formula", make: advanced::quadformula, lesson: "Any equation ax² + bx + c = 0 can be solved with:\n\n$x = \\frac{−b ± \\sqrt{b^2 − 4ac}}{2a}$\n\n1. Find a, b and c.\n2. Work out b² − 4ac and its square root.\n3. Do it once with + and once with −. Two answers!" },
            Level { id: "discriminant", title: "The Discriminant", make: advanced::discriminant, lesson: "The part under the square root, b² − 4ac, is called the discriminant. It tells you how many real solutions there are without solving:\n\n• more than 0 → 2 solutions\n• exactly 0 → 1 solution\n• less than 0 → no real solutions (you can't take the square root of a negative)" },
            Level { id: "functions", title: "Functions", make: advanced::functions, lesson: "A function is a machine: put a number in, get one number out.\n\nf(x) = 2x + 1 → f(3) = 2 × 3 + 1 = 7\n\n• f(g(x)): do g first, then put its answer into f.\n• f⁻¹ is the undo machine: if f(3) = 7, then f⁻¹(7) = 3." },
            Level { id: "inequalities", title: "Inequalities", make: advanced::inequalities, lesson: "An inequality says one side is bigger or smaller: <, >, ≤ (or equal), ≥ (or equal).\n\nSolve it like an equation, with one big rule:\nwhen you multiply or divide by a NEGATIVE number, flip the sign!\n\n−2x < 6 → x > −3" },
            Level { id: "absval", title: "Absolute Value", make: advanced::absval, lesson: "|x| is the distance from 0, so it's never negative: |−5| = 5.\n\n|x − 2| = 3 means x − 2 is 3 OR −3:\nx − 2 = 3 → x = 5\nx − 2 = −3 → x = −1\n\nUsually two answers!" },
            Level { id: "growth", title: "Growth & Interest", make: advanced::growth, lesson: "Exponential growth multiplies by the same amount each step.\n\n• Doubling: start × 2ⁿ\n• Halving (half-life): start ÷ 2ⁿ\n• Interest: money × (1 + rate) each year\n\n₱1000 at 10% for 2 years: 1000 × 1.1 × 1.1 = ₱1210" },
            Level { id: "logs", title: "Logarithms", make: advanced::logs, lesson: "A logarithm asks: what power?\n\nlog₂ 8 = 3, because 2³ = 8.\n\nTo solve 3ˣ = 81, write 81 as a power of 3: 3⁴ = 81, so x = 4.\n\nRule: log a + log b = log(a × b)." },
        ],
    },
    World {
        name: "Triangle Island",
        emoji: "📐",
        grades: "Grades 8–11",
        levels: &[
            Level { id: "angles", title: "Angles & Polygons", make: advanced::angles, lesson: "Angle facts to remember:\n• A triangle's angles add to 180°.\n• Complementary angles add to 90°; supplementary to 180°.\n• A polygon with n sides has inside angles adding to (n − 2) × 180°.\n• In a regular polygon all angles are equal: divide the total by n." },
            Level { id: "circles", title: "Circles", make: advanced::circles, lesson: "The radius goes from the center to the edge; the diameter goes all the way across (2 × radius).\n\n• Circumference (around) = 2πr\n• Area = πr²\n\nAnswers \"in terms of π\" keep π as π: a radius of 3 gives an area of 9π." },
            Level { id: "volume", title: "Area & Volume", make: advanced::volume, lesson: "Volume is how much fits inside (cubic units).\n• Box: length × width × height\n• Cylinder: πr²h\n• Cone: $\\frac{1}{3}πr^2h$ (a third of a cylinder)\n• Sphere: $\\frac{4}{3}πr^3$\n\nSurface area adds up the area of every face." },
            Level { id: "coords", title: "Distance & Midpoint", make: advanced::coords, lesson: "On a grid:\n• Distance: make a right triangle and use Pythagoras:\n$d = \\sqrt{(x_2 − x_1)^2 + (y_2 − y_1)^2}$\n• Midpoint: the average of the x's and of the y's:\n$(\\frac{x_1 + x_2}{2}, \\frac{y_1 + y_2}{2})$\n\nFrom (1, 2) to (4, 6): across 3, up 4, so the distance is 5." },
            Level { id: "similar", title: "Similar Triangles", make: advanced::similar, lesson: "Similar shapes are the same shape at different sizes. Their sides all grow by the same scale factor.\n\nIf one side goes from 4 to 12 (× 3), every side is × 3.\n\nMatching angles stay exactly the same." },
            Level { id: "trigratios", title: "SOH-CAH-TOA", make: advanced::trigratios, lesson: "In a right triangle, looking from angle A:\n• Opposite: the side across from A\n• Adjacent: the side next to A\n• Hypotenuse: the longest side\n\nSOH: $\\sin A = \\frac{opposite}{hypotenuse}$\nCAH: $\\cos A = \\frac{adjacent}{hypotenuse}$\nTOA: $\\tan A = \\frac{opposite}{adjacent}$" },
            Level { id: "specialangles", title: "Special Angles & Radians", make: advanced::specialangles, lesson: "Two special right triangles give exact values:\n• 30-60-90: sides 1 : √3 : 2, so sin 30° = 1/2\n• 45-45-90: sides 1 : 1 : √2, so tan 45° = 1\n\nRadians measure angles with π: a half turn is π radians = 180°.\n$\\frac{π}{3}$ = 180° ÷ 3 = 60°" },
            Level { id: "lawsincos", title: "Law of Sines & Cosines", make: advanced::lawsincos, lesson: "For any triangle (not just right ones):\n\nLaw of cosines, when you know two sides and the angle between them:\n$c^2 = a^2 + b^2 − 2ab \\cos C$\n\nLaw of sines, when you know an angle and its opposite side:\n$\\frac{a}{\\sin A} = \\frac{b}{\\sin B}$" },
        ],
    },
    World {
        name: "Chance Carnival",
        emoji: "🎪",
        grades: "Grades 7–12",
        levels: &[
            Level { id: "average", title: "Mean, Median, Mode & Range", make: advanced::average, lesson: "Four ways to describe a list of numbers:\n• Mean (average): add them all, divide by how many:\n$mean = \\frac{total}{how many}$\n• Median: the middle one when they're in order.\n• Mode: the one that appears most often.\n• Range: biggest − smallest." },
            Level { id: "probability", title: "Probability", make: advanced::probability, lesson: "Probability = $\\frac{ways you want}{all the ways}$\n\nIt's always between 0 (impossible) and 1 (certain).\n\nA bag with 3 red and 7 blue:\n$P(red) = \\frac{3}{10}$\n$P(not red) = 1 − \\frac{3}{10} = \\frac{7}{10}$" },
            Level { id: "compound", title: "\"And\" Probability", make: advanced::compound, lesson: "For two things to BOTH happen, multiply their chances.\n\nTwo coin flips, both heads:\n$\\frac{1}{2} × \\frac{1}{2} = \\frac{1}{4}$\n\nIf you don't put the first pick back, the second pick has one fewer to choose from." },
            Level { id: "counting", title: "Counting: Arrangements & Choices", make: advanced::counting, lesson: "• Several choices in a row: multiply (3 shirts × 2 pants = 6 outfits).\n• Arranging n things in order: n! = n × (n − 1) × … × 1.\n• Permutation (order matters): 5 × 4 × 3 for top 3 out of 5.\n• Combination (order doesn't matter):\n$nCr = \\frac{n!}{r!(n − r)!}$" },
            Level { id: "expected", title: "Expected Value", make: advanced::expected, lesson: "Expected value is the average result if you played many, many times.\n\nMultiply each result by its chance, then add them up.\n\nA spinner with ₱0, ₱0, ₱10, ₱10:\n$(0 + 0 + 10 + 10) × \\frac{1}{4}$ = ₱5 per spin" },
        ],
    },
];

/// Every level in play order, with its world index.
pub fn all() -> impl Iterator<Item = (usize, &'static Level)> {
    WORLDS.iter().enumerate().flat_map(|(w, world)| world.levels.iter().map(move |l| (w, l)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::answer::Verdict;
    use rand::SeedableRng;

    /// Every generator makes problems whose own answer passes its own check,
    /// with hints, many times over.
    #[test]
    fn every_level_makes_sound_problems() {
        let mut g = StdRng::seed_from_u64(7);
        for (_, level) in all() {
            for _ in 0..300 {
                let p = (level.make)(&mut g);
                assert!(p.hints.len() >= 2, "{}: {}", level.id, p.prompt);
                assert_eq!(p.answer.check(&p.answer.show()), Verdict::Right, "{}: {} → {:?}", level.id, p.prompt, p.answer);
                if let Answer::Frac(_, d) = p.answer {
                    assert!(d > 0);
                }
            }
        }
    }

    /// `cargo test samples -- --ignored --nocapture` prints a problem from every level to proofread.
    #[test]
    #[ignore]
    fn samples() {
        let mut g = StdRng::seed_from_u64(3);
        for (_, level) in all() {
            let p = (level.make)(&mut g);
            println!("── {} ──\n{}\n= {}", level.title, p.prompt, p.answer.show());
            for h in &p.hints {
                println!("  💡 {h}");
            }
        }
    }

    /// The worked steps of a stacked problem end on its answer.
    #[test]
    fn column_hints_reach_the_answer() {
        let mut g = StdRng::seed_from_u64(11);
        for (_, level) in all().filter(|(_, l)| l.id.starts_with("col_")) {
            for _ in 0..300 {
                let p = (level.make)(&mut g);
                assert!(p.column.is_some());
                let last = p.hints.last().unwrap();
                assert!(last.contains(&p.answer.show()), "{}: {:?} vs {last}", level.id, p.answer);
            }
        }
    }

    /// Formulas are well formed: `$` comes in pairs, and every command inside
    /// is one math.rs draws (anything else would show up as text).
    #[test]
    fn formulas_are_well_formed() {
        let check = |where_: &str, text: &str| {
            assert_eq!(text.matches('$').count() % 2, 0, "unpaired $ in {where_}: {text}");
            let plain = crate::mathtext::plain(text);
            assert!(!plain.contains('\\') && !plain.contains('{') && !plain.contains('}'), "{where_}: {text} → {plain}");
        };
        let mut g = StdRng::seed_from_u64(13);
        for (_, level) in all() {
            check(level.id, level.lesson);
            for _ in 0..200 {
                let p = (level.make)(&mut g);
                check(level.id, &p.prompt);
                for h in &p.hints {
                    check(level.id, h);
                }
                for st in crate::teach::lesson(level.id, &p) {
                    check(level.id, &st.say);
                }
            }
        }
    }

    #[test]
    fn ids_are_unique() {
        let ids: Vec<_> = all().map(|(_, l)| l.id).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len());
    }
}
