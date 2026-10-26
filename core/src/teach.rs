//! Hands-on lessons: Kiko works one example *with* the kid, the way a parent
//! or teacher does the first time. The problem is broken into small
//! questions ("What is 9 + 9?"), each followed by what to write and where.
//! Stacked problems are done in the real column pad, one glowing box at a
//! time; order-of-operations problems are reduced one step at a time.

use crate::{
    answer::{Answer, minus},
    expr::{self, Tok},
    levels::{ColOp, Column, Problem},
};

/// A box in the column pad.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    /// Answer box at this place (0 = ones).
    Answer(usize),
    /// Carry/borrow box above this place.
    Carry(usize),
    /// Long division: quotient box above dividend digit k (from the left).
    Quotient(usize),
    Remainder,
}

#[derive(Clone, Debug)]
pub struct Step {
    pub say: String,
    /// A small question the kid answers in the entry.
    pub ask: Option<Answer>,
    /// A digit the kid writes into the column pad.
    pub write: Option<(Target, char)>,
    /// The place (or dividend digit) to light up while talking about it.
    pub point: Option<usize>,
    /// What's on the board for expression problems.
    pub board: Option<String>,
}

fn say(s: impl Into<String>) -> Step {
    Step { say: s.into(), ask: None, write: None, point: None, board: None }
}

fn ask(s: impl Into<String>, a: i64) -> Step {
    Step { ask: Some(Answer::Int(a)), ..say(s) }
}

fn write(s: impl Into<String>, t: Target, d: i64) -> Step {
    Step { write: Some((t, char::from_digit(d as u32, 10).unwrap_or('0'))), ..say(s) }
}

impl Step {
    fn at(mut self, p: usize) -> Self {
        self.point = Some(p);
        self
    }

    fn on(mut self, board: impl Into<String>) -> Self {
        self.board = Some(board.into());
        self
    }
}

const PLACES: [&str; 6] = ["ones", "tens", "hundreds", "thousands", "ten-thousands", "hundred-thousands"];

fn place(i: usize) -> &'static str {
    PLACES[i.min(PLACES.len() - 1)]
}

fn digits(n: i64) -> Vec<i64> {
    // Least significant first.
    n.to_string().chars().rev().map(|c| c.to_digit(10).unwrap_or(0) as i64).collect()
}

pub fn lesson(level_id: &str, p: &Problem) -> Vec<Step> {
    if let Some(c) = p.column {
        return match c.op {
            ColOp::Add => col_add(c),
            ColOp::Sub => col_sub(c),
            ColOp::Mul => col_mul(c),
            ColOp::Div => col_div(c),
        };
    }
    if let Some(toks) = &p.expr {
        return expression(toks, p);
    }
    match level_id {
        "add10" => count_on(p).unwrap_or_else(|| from_hints(p)),
        "add20" => make_ten(p).unwrap_or_else(|| from_hints(p)),
        _ => from_hints(p),
    }
}

fn col_add(c: Column) -> Vec<Step> {
    let (a, b) = (c.top, c.bottom);
    let (da, db) = (digits(a), digits(b));
    let w = da.len().max(db.len());
    let mut s = vec![
        say(format!("Let's add {a} + {b} the way we do it on paper. 📝 We write one number on top of the other, so the ones line up with the ones and the tens with the tens.")),
        say("We always start on the RIGHT, with the ones column. I'll make it glow for you. ✨").at(0),
    ];
    let mut carry = 0;
    for i in 0..w {
        let (x, y) = (da.get(i).copied().unwrap_or(0), db.get(i).copied().unwrap_or(0));
        let sum = x + y + carry;
        s.push(if carry > 0 {
            ask(format!("In the {} column we have {x} and {y}, plus the 1 we carried. What is {x} + {y} + 1?", place(i)), sum).at(i)
        } else if i == 0 {
            ask(format!("Look at the ones column: {x} and {y}. What is {x} + {y}?"), sum).at(i)
        } else {
            ask(format!("Now the {} column: {x} and {y}. What is {x} + {y}?", place(i)), sum).at(i)
        });
        if sum >= 10 {
            s.push(write(format!("{sum} is 1 ten and {} ones. Only ONE digit fits in a box, so write the {} in the answer box under the {}.", sum % 10, sum % 10, place(i)), Target::Answer(i), sum % 10).at(i));
            s.push(write(format!("The 1 ten moves next door! Write a little 1 in the small box on top of the {} column. That's called carrying.", place(i + 1)), Target::Carry(i + 1), 1).at(i + 1));
        } else {
            s.push(write(format!("{sum} fits in one box. Write {sum} under the {}.", place(i)), Target::Answer(i), sum).at(i));
        }
        carry = sum / 10;
    }
    if carry > 0 {
        s.push(write(format!("The {} column has nothing else, only the 1 we carried. Bring it down: write 1 in the answer box.", place(w)), Target::Answer(w), 1).at(w));
    }
    s.push(say(format!("Look at that! {a} + {b} = {}. 🎉 You added like a pro!", a + b)));
    s
}

fn col_sub(c: Column) -> Vec<Step> {
    let (a, b) = (c.top, c.bottom);
    let orig = digits(a);
    let db = digits(b);
    let mut cur = orig.clone();
    let mut s = vec![
        say(format!("Let's take {b} away from {a}. 📝 The bigger number goes on top, and we line up the ones, tens and hundreds.")),
        say("We start on the RIGHT, with the ones column. ✨").at(0),
    ];
    for i in 0..orig.len() {
        let y = db.get(i).copied().unwrap_or(0);
        let changed = cur[i] != orig[i];
        if cur[i] < y {
            s.push(say(format!("Look at the {}: {} − {y}. Uh-oh, {} is smaller than {y}, so we can't take {y} away. We need to BORROW from the column next door.", place(i), cur[i], cur[i])).at(i));
            let j = (i + 1..cur.len()).find(|&k| cur[k] > 0).unwrap_or(i + 1);
            if j == i + 1 {
                s.push(write(format!("Take 1 from the {}: the {} there becomes {}. Write {} in the small box above it.", place(j), cur[j], cur[j] - 1, cur[j] - 1), Target::Carry(j), cur[j] - 1).at(j));
            } else {
                s.push(say(format!("The {} has 0, so it has nothing to lend! We go one more column over, to the {}.", place(i + 1), place(j))).at(j));
                s.push(write(format!("The {} in the {} becomes {}. Write {} in the small box above it.", cur[j], place(j), cur[j] - 1, cur[j] - 1), Target::Carry(j), cur[j] - 1).at(j));
                for k in (i + 1..j).rev() {
                    s.push(write(format!("The 0 in the {} gets 10, then lends 1 to the right, so it becomes 9. Write 9 above it.", place(k)), Target::Carry(k), 9).at(k));
                }
            }
            cur[j] -= 1;
            for k in cur.iter_mut().take(j).skip(i + 1) {
                *k = 9;
            }
            let before = cur[i];
            cur[i] += 10;
            s.push(ask(format!("Now the {} has {before} + the 10 we borrowed = {}. What is {} − {y}?", place(i), cur[i], cur[i]), cur[i] - y).at(i));
        } else if changed {
            s.push(ask(format!("Now the {}. Remember, it lent 1, so it's {} now. What is {} − {y}?", place(i), cur[i], cur[i]), cur[i] - y).at(i));
        } else {
            s.push(ask(format!("Look at the {}: {} − {y}. What is it?", place(i), cur[i]), cur[i] - y).at(i));
        }
        let r = cur[i] - y;
        if r == 0 && i + 1 == orig.len() {
            s.push(say("It's 0, and a 0 at the very front doesn't need to be written. We can leave that box empty.").at(i));
        } else {
            s.push(write(format!("Write {r} under the {}.", place(i)), Target::Answer(i), r).at(i));
        }
    }
    s.push(say(format!("We did it! {a} − {b} = {}. 🎉 Borrowing is a super skill!", a - b)));
    s
}

fn col_mul(c: Column) -> Vec<Step> {
    let (a, b) = (c.top, c.bottom);
    let da = digits(a);
    let mut s = vec![say(format!(
        "Let's multiply {a} × {b}. 📝 We multiply {b} by EACH digit on top, one at a time, starting on the RIGHT."
    ))];
    let mut carry = 0;
    for (i, &x) in da.iter().enumerate() {
        let last = i + 1 == da.len();
        s.push(if i == 0 {
            ask(format!("Ones first: {b} × {x} = ?"), b * x).at(i)
        } else {
            ask(format!("Next, the {}: {b} × {x} = ?", place(i)), b * x).at(i)
        });
        let p = b * x + carry;
        if carry > 0 {
            s.push(ask(format!("Now add the {carry} we carried: {} + {carry} = ?", b * x), p).at(i));
        }
        if p >= 10 && !last {
            s.push(write(format!("{p} is {} tens and {} ones. Write the {} under the {}.", p / 10, p % 10, p % 10, place(i)), Target::Answer(i), p % 10).at(i));
            s.push(write(format!("Carry the {}: write it in the small box over the {} column. We'll add it after we multiply there.", p / 10, place(i + 1)), Target::Carry(i + 1), p / 10).at(i + 1));
        } else if p >= 10 {
            s.push(write(format!("This is the last digit, so all of {p} gets written. First the {} under the {}…", p % 10, place(i)), Target::Answer(i), p % 10).at(i));
            s.push(write(format!("…and the {} in front of it.", p / 10), Target::Answer(i + 1), p / 10).at(i + 1));
        } else {
            s.push(write(format!("Write {p} under the {}.", place(i)), Target::Answer(i), p).at(i));
        }
        carry = if last { 0 } else { p / 10 };
    }
    s.push(say(format!("{a} × {b} = {}. 🎉 Multiplying big numbers is easy one digit at a time!", a * b)));
    s
}

fn col_div(c: Column) -> Vec<Step> {
    let (n, v) = (c.top, c.bottom);
    let ds: Vec<i64> = digits(n).into_iter().rev().collect();
    let (q, r) = (n / v, n % v);
    let mut s = vec![
        say(format!("Let's share {n} into groups of {v}. 📝 The {n} goes inside the bracket and the {v} waits outside. We work from the LEFT, one digit at a time.")),
        say("We do 4 things, over and over: Divide ➗, Multiply ✖️, Subtract ➖, Bring down ⬇️. Ready?"),
    ];
    let mut rem = 0;
    let mut started = false;
    for (k, &d) in ds.iter().enumerate() {
        let cur = rem * 10 + d;
        if !started && cur < v && k + 1 < ds.len() {
            s.push(say(format!("Look at the first digit, {cur}. How many {v}s fit in {cur}? None, {v} is too big! So we leave the box above it empty and use the next digit too.")).at(k));
            rem = cur;
            continue;
        }
        started = true;
        let qd = cur / v;
        s.push(ask(format!("Divide: how many {v}s fit in {cur}? (Think: {v} × what is close to {cur}, but not bigger?)"), qd).at(k));
        s.push(write(format!("Write {qd} on top, right above the {d}."), Target::Quotient(k), qd).at(k));
        s.push(ask(format!("Multiply: {qd} × {v} = ?"), qd * v).at(k));
        s.push(ask(format!("Subtract: {cur} − {} = ?", qd * v), cur - qd * v).at(k));
        rem = cur - qd * v;
        if let Some(&next) = ds.get(k + 1) {
            s.push(say(format!("Bring down the next digit, {next}. Put it next to the {rem}: now we have {}.", rem * 10 + next)).at(k + 1));
        }
    }
    if r > 0 {
        s.push(write(format!("There are no more digits to bring down. The {r} left over is the remainder. Write it in the r box."), Target::Remainder, r));
        s.push(say(format!("{n} ÷ {v} = {q} r {r}. 🎉 That's long division!")));
    } else {
        s.push(say(format!("No more digits, and nothing left over! {n} ÷ {v} = {q}. 🎉 It shares perfectly!")));
    }
    s
}

fn expression(toks: &[Tok], p: &Problem) -> Vec<Step> {
    let Some((v, steps)) = expr::solve(toks) else { return from_hints(p) };
    let start = expr::show(toks);
    let mut s = Vec::new();
    if p.prompt.starts_with("If ") {
        s.push(say(format!("{} First, we swap each letter for its number. Then it looks like this. 👇", p.prompt.replace('\n', " "))).on(&start));
    }
    s.push(say("When a problem has lots of operations, we don't just go left to right. We follow the order:\n( ) first, then exponents, then × and ÷, then + and −.").on(&start));
    for st in &steps {
        s.push(ask(format!("{}. What is {}?", st.why, st.ask), st.value).on(&st.before));
        if st.after != minus(v) {
            s.push(say("So now the problem is shorter. Let's keep going!").on(&st.after));
        }
    }
    s.push(say(format!("{start} = {}. 🎉 You followed the order perfectly!", minus(v))).on(minus(v)));
    s
}

/// The two numbers of "a + b = ?".
fn two(p: &Problem) -> Option<(i64, i64)> {
    let (a, rest) = p.prompt.split_once(" + ")?;
    let b = rest.split_whitespace().next()?;
    Some((a.trim().parse().ok()?, b.parse().ok()?))
}

fn count_on(p: &Problem) -> Option<Vec<Step>> {
    let (a, b) = two(p)?;
    Some(vec![
        say(format!("Let's add {a} + {b} by counting! Adding means putting two groups together. 🧺")),
        ask("Count the first group. How many are there?", a),
        ask(format!("Now keep {a} in your head and count on {b} more, one for each in the second group. What number do you land on?"), a + b),
        say(format!("{a} + {b} = {}. 🎉 Tip: start from the bigger number, so you count less!", a + b)),
    ])
}

fn make_ten(p: &Problem) -> Option<Vec<Step>> {
    let (a, b) = two(p)?;
    let (big, small) = (a.max(b), a.min(b));
    let need = 10 - big;
    Some(vec![
        say(format!("Let's add {a} + {b}. Here's a trick: 10 is a friendly number, so let's make a 10 first! 🔟")),
        ask(format!("How many more does {big} need to become 10?"), need),
        ask(format!("Let's take those {need} from the {small}. How many are left in the {small}?"), small - need),
        ask(format!("Now we have 10 and {}. What is 10 + {}?", small - need, small - need), a + b),
        say(format!("{a} + {b} = {}. 🎉 Making a ten makes adding easy!", a + b)),
    ])
}

/// Turn "Ones: 9 + 9 = 18 → write 8" into the question "Ones: 9 + 9 = ?" and
/// its answer 18, using the first "= number" that ends a calculation.
fn mask(h: &str) -> Option<(String, i64)> {
    let mut from = 0;
    while let Some(off) = h[from..].find("= ") {
        let at = from + off + 2;
        let rest = &h[at..];
        let neg = rest.starts_with('−') || rest.starts_with('-');
        let body = if neg { &rest[rest.chars().next().unwrap().len_utf8()..] } else { rest };
        let len = body.chars().take_while(char::is_ascii_digit).count();
        if len > 0 {
            let after = body[len..].chars().collect::<Vec<_>>();
            let ends = match after.first() {
                None => true,
                // "is 6." ends a sentence; "9.88" is a decimal.
                Some('.' | ',') => !after.get(1).is_some_and(char::is_ascii_digit),
                // "−20 ÷ 5" keeps calculating.
                Some(' ') => !after.get(1).is_some_and(|c| "×÷+−-/^".contains(*c)),
                Some(';' | '!' | ')' | '\n') => true,
                // Fractions, percents, exponents.
                _ => false,
            };
            if ends {
                let n: i64 = body[..len].parse().ok()?;
                return Some((format!("{}?", &h[..at]), if neg { -n } else { n }));
            }
        }
        from = at;
    }
    None
}

/// Any level: talk through its hints, turning each calculation into a small
/// question, then ask for the whole answer.
fn from_hints(p: &Problem) -> Vec<Step> {
    let mut s = vec![say(format!("Let's work this one out together, step by step:\n{}", p.prompt))];
    let mut last_asked = None;
    for h in &p.hints {
        match mask(h) {
            Some((q, n)) => {
                s.push(ask(q.clone(), n));
                // Keep the rest of the sentence ("→ write 8, carry 1…") when there is more to it.
                if h.len() > q.len() + 6 {
                    s.push(say(h.clone()));
                }
                last_asked = Some(Answer::Int(n));
            }
            None => s.push(say(h.clone())),
        }
    }
    if last_asked.as_ref() != Some(&p.answer) {
        let mut fin = say(format!("Now you know every step! So what's the answer?\n{}", p.prompt));
        fin.ask = Some(p.answer.clone());
        s.push(fin);
    }
    s.push(say("You did it! 🎉 That's exactly how to solve it."));
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::levels;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn masks_the_first_finished_calculation() {
        assert_eq!(mask("Ones: 9 + 9 = 18 → write 8"), Some(("Ones: 9 + 9 = ?".into(), 18)));
        assert_eq!(mask("x = −20 ÷ 5 = −4"), Some(("x = −20 ÷ 5 = ?".into(), -4)));
        assert_eq!(mask("12/30 = 2/5"), None);
        assert_eq!(mask("1.88 + 8.00 = 9.88"), None);
        assert_eq!(mask("The answer is 6."), None);
    }

    /// Walk every lesson as a kid who always answers right: every written
    /// digit lands where the pad expects, and the pad ends up holding the answer.
    #[test]
    fn column_lessons_write_the_answer() {
        let mut g = StdRng::seed_from_u64(5);
        for (_, level) in levels::all().filter(|(_, l)| l.id.starts_with("col_")) {
            for _ in 0..300 {
                let p = (level.make)(&mut g);
                let c = p.column.unwrap();
                let steps = lesson(level.id, &p);
                let mut answer = std::collections::BTreeMap::new();
                let mut rem = 0;
                for st in &steps {
                    match st.write {
                        Some((Target::Answer(i), d)) => {
                            answer.insert(i as i64, d);
                        }
                        Some((Target::Quotient(k), d)) => {
                            let n = c.top.to_string().len() as i64;
                            answer.insert(n - 1 - k as i64, d);
                        }
                        Some((Target::Remainder, d)) => rem = d.to_digit(10).unwrap() as i64,
                        _ => {}
                    }
                }
                let got: i64 = answer.iter().map(|(&i, &d)| d.to_digit(10).unwrap() as i64 * 10i64.pow(i as u32)).sum();
                let want = match p.answer {
                    Answer::Int(n) => (n, 0),
                    Answer::Remainder(q, r) => (q, r),
                    _ => unreachable!(),
                };
                assert_eq!((got, rem), want, "{}: {:?}", level.id, c);
            }
        }
    }

    /// `cargo test lesson_samples -- --ignored --nocapture` prints lessons to proofread.
    #[test]
    #[ignore]
    fn lesson_samples() {
        let mut g = StdRng::seed_from_u64(21);
        for (_, level) in levels::all() {
            let p = (level.make)(&mut g);
            println!("══ {} ══", level.title);
            for st in lesson(level.id, &p) {
                let tag = match (&st.ask, st.write) {
                    (Some(a), _) => format!("  [ask → {}]", a.show()),
                    (_, Some((t, d))) => format!("  [write {d} in {t:?}]"),
                    _ => String::new(),
                };
                println!("  {}{tag}{}", st.say.replace('\n', " / "), st.board.map(|b| format!("   board: {b}")).unwrap_or_default());
            }
        }
    }

    #[test]
    fn every_level_has_a_lesson() {
        let mut g = StdRng::seed_from_u64(9);
        for (_, level) in levels::all() {
            for _ in 0..50 {
                let p = (level.make)(&mut g);
                let steps = lesson(level.id, &p);
                assert!(steps.len() >= 3, "{}", level.id);
                for st in &steps {
                    if let Some(a) = &st.ask {
                        assert_eq!(a.check(&a.show()), crate::answer::Verdict::Right);
                    }
                }
            }
        }
    }
}
