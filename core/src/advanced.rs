//! High school levels: Algebra II, Geometry & Trigonometry, and Statistics &
//! Probability. Every problem uses numbers that work out cleanly (no
//! calculator needed), and its hints end in a worked solution like the
//! younger worlds.

use rand::Rng;

use crate::answer::{Answer, minus, show_dec};
use crate::expr::sup;
use crate::figure::Figure;
use crate::levels::{G, Problem, coef, given, right, with, fm, frac, mfr, nonzero, pick, problem, ri, signed, simplify_hint, sqrt, term, wrap};

fn sub(n: i64) -> String {
    const D: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
    n.to_string().chars().map(|c| D[c.to_digit(10).unwrap_or(0) as usize]).collect()
}

/// "2x² − 4x − 6", leaving out zero terms.
fn quad(a: i64, b: i64, c: i64) -> String {
    format!("{}{}{}", coef(a, "x²"), term(b, "x"), if c == 0 { String::new() } else { signed(c) })
}

fn is_square(n: i64) -> bool {
    let r = (n as f64).sqrt().round() as i64;
    r * r == n
}

fn list(xs: &[i64]) -> String {
    xs.iter().map(|&x| minus(x)).collect::<Vec<_>>().join(", ")
}

// ───────────────────────── Equation Galaxy (Algebra II) ─────────────────────────

pub fn exponents(g: &mut G) -> Problem {
    match ri(g, 0, 4) {
        0 => {
            let (a, b) = (ri(g, 2, 9), ri(g, 2, 9));
            problem(
                format!("x{} · x{} = xⁿ\nWhat is n?", sup(a), sup(b)),
                Answer::Int(a + b),
                vec![
                    "Multiplying powers with the same base? Add the exponents.".to_string(),
                    format!("x{} · x{} is {a} x's multiplied by {b} more x's.", sup(a), sup(b)),
                    format!("{a} + {b} = {}, so x{} · x{} = x{}", a + b, sup(a), sup(b), sup(a + b)),
                ],
            )
        }
        1 => {
            let (a, b) = (ri(g, 2, 6), ri(g, 2, 5));
            problem(
                format!("(x{}){} = xⁿ\nWhat is n?", sup(a), sup(b)),
                Answer::Int(a * b),
                vec![
                    "A power of a power? Multiply the exponents.".to_string(),
                    format!("(x{}){} is x{} written {b} times, so {b} groups of {a}.", sup(a), sup(b), sup(a)),
                    format!("{a} × {b} = {}, so the answer is x{}", a * b, sup(a * b)),
                ],
            )
        }
        2 => {
            let a = ri(g, 5, 12);
            let b = ri(g, 2, a - 1);
            problem(
                format!("x{} ÷ x{} = xⁿ\nWhat is n?", sup(a), sup(b)),
                Answer::Int(a - b),
                vec![
                    "Dividing powers with the same base? Subtract the exponents.".to_string(),
                    format!("{b} of the x's on top cancel with the {b} on the bottom."),
                    format!("{a} − {b} = {}, so the answer is x{}", a - b, sup(a - b)),
                ],
            )
        }
        3 => {
            let (b, k) = loop {
                let (b, k) = (ri(g, 2, 5), ri(g, 1, 3));
                if b.pow(k as u32) <= 125 {
                    break (b, k);
                }
            };
            let p = b.pow(k as u32);
            problem(
                format!("{b}{} = ?", sup(-k)),
                Answer::frac(1, p),
                vec![
                    format!("A negative exponent means \"one over\":\n${b}^{{-{k}}} = {}$", frac(1, format!("{b}^{k}"))),
                    format!("{b}{} = {p}", sup(k)),
                    format!("So {b}{} = {}", sup(-k), mfr(1, p)),
                ],
            )
        }
        _ => {
            let n = ri(g, 2, 99);
            problem(
                format!("{n}⁰ = ?"),
                Answer::Int(1),
                vec![
                    "Any number except 0, to the power 0, is 1.".to_string(),
                    format!("Why? {n}³ ÷ {n}³ = {n}⁰, and anything divided by itself is 1."),
                    format!("So {n}⁰ = 1"),
                ],
            )
        }
    }
}

pub fn radicals(g: &mut G) -> Problem {
    if g.random_bool(0.6) {
        let k = ri(g, 2, 9);
        let b = pick(g, &[2, 3, 5, 6, 7, 10]);
        let n = k * k * b;
        problem(
            format!("Simplify ${} = a{}$\nWhat is a?", sqrt(n), sqrt(b)),
            Answer::Int(k),
            vec![
                "Find the biggest perfect square that divides the number: 4, 9, 16, 25, 36, 49, 64, 81 …".to_string(),
                format!("{n} = {} × {b}, and {} = {k} × {k}.", k * k, k * k),
                format!("${} = {} × {} = {k}{}$", sqrt(n), sqrt(k * k), sqrt(b), sqrt(b)),
            ],
        )
    } else {
        loop {
            let s = ri(g, 2, 12);
            let sq = s * s;
            let ds: Vec<i64> = (2..sq).filter(|&d| sq % d == 0 && !is_square(d) && !is_square(sq / d)).collect();
            if ds.is_empty() {
                continue;
            }
            let a = pick(g, &ds);
            let b = sq / a;
            return problem(
                format!("${} × {}$ = ?", sqrt(a), sqrt(b)),
                Answer::Int(s),
                vec![
                    "You can multiply under one square root:\n$\\sqrt{a} × \\sqrt{b} = \\sqrt{a × b}$".to_string(),
                    format!("${} × {} = {} = {}$", sqrt(a), sqrt(b), sqrt(format!("{a} × {b}")), sqrt(sq)),
                    format!("{s} × {s} = {sq}, so ${}$ = {s}", sqrt(sq)),
                ],
            );
        }
    }
}

pub fn expand(g: &mut G) -> Problem {
    let (a, b) = (nonzero(g, -9, 9), nonzero(g, -9, 9));
    let (mid, last) = (a + b, a * b);
    let bin = |n: i64| format!("(x{})", signed(n));
    let ask_mid = g.random_bool(0.5);
    let outer_inner = if mid == 0 { "0x (they cancel!)".to_string() } else { coef(mid, "x") };
    problem(
        format!("Expand {}{} = x² + bx + c\nWhat is {}?", bin(a), bin(b), if ask_mid { "b" } else { "c" }),
        Answer::Int(if ask_mid { mid } else { last }),
        vec![
            "FOIL: multiply the First, Outer, Inner and Last terms, then combine like terms.".to_string(),
            format!("Outer + Inner: x × {} + {} × x = {outer_inner}", wrap(b), wrap(a)),
            format!("Last: {} × {} = {}. So it's x²{}{}", minus(a), wrap(b), minus(last), term(mid, "x"), signed(last)),
        ],
    )
}

pub fn quadformula(g: &mut G) -> Problem {
    let a = pick(g, &[2, 3, 4, -2]);
    let (r1, r2) = loop {
        let (r1, r2) = (nonzero(g, -6, 6), nonzero(g, -6, 6));
        if r1 != r2 {
            break (r1, r2);
        }
    };
    let (b, c) = (-a * (r1 + r2), a * r1 * r2);
    let d = b * b - 4 * a * c;
    let s = (a * (r1 - r2)).abs();
    let (x1, x2) = ((-b + s) / (2 * a), (-b - s) / (2 * a));
    given(
problem(
        format!("Solve with the quadratic formula:\n{} = 0", quad(a, b, c)),
        Answer::Roots(r1, r2),
        vec![
            "The quadratic formula:\n$x = \\frac{−b ± \\sqrt{b^2 − 4ac}}{2a}$".to_string(),
            format!("a = {}, b = {}, c = {}: b² − 4ac = {} − {} = {d}, and ${}$ = {s}", minus(a), minus(b), minus(c), b * b, wrap(4 * a * c), sqrt(d)),
            format!(
                "$x = {}$ = {}\nor $x = {}$ = {}",
                frac(format!("{} + {s}", minus(-b)), minus(2 * a)),
                minus(x1),
                frac(format!("{} − {s}", minus(-b)), minus(2 * a)),
                minus(x2)
            ),
        ],
    ),
&[("a", a), ("b", b), ("c", c)])
}

pub fn discriminant(g: &mut G) -> Problem {
    let target = ri(g, 0, 2);
    let (a, b, c, d) = loop {
        let (a, b, c) = if target == 1 {
            let r = nonzero(g, -6, 6);
            let k = pick(g, &[1, 1, 2, 3]);
            (k, -2 * k * r, k * r * r)
        } else {
            (ri(g, 1, 4), ri(g, -10, 10), nonzero(g, -12, 12))
        };
        let d = b * b - 4 * a * c;
        let n = if d > 0 { 2 } else if d == 0 { 1 } else { 0 };
        if n == target {
            break (a, b, c, d);
        }
    };
    let (word, answer) = if d > 0 {
        ("more than 0, so there are 2 real solutions", 2)
    } else if d == 0 {
        ("exactly 0, so there is 1 real solution", 1)
    } else {
        ("less than 0, so there are no real solutions", 0)
    };
    given(
problem(
        format!("How many real solutions does\n{} = 0\nhave?", quad(a, b, c)),
        Answer::Int(answer),
        vec![
            "The discriminant b² − 4ac decides: more than 0 → 2 solutions, exactly 0 → 1, less than 0 → none.".to_string(),
            format!("a = {}, b = {}, c = {}: b² − 4ac = {} − {} = {}", minus(a), minus(b), minus(c), b * b, wrap(4 * a * c), minus(d)),
            format!("{} is {word}.", minus(d)),
        ],
    ),
&[("a", a), ("b", b), ("c", c)])
}

pub fn functions(g: &mut G) -> Problem {
    match ri(g, 0, 2) {
        0 => {
            let (a, b, c) = (nonzero(g, -3, 4), ri(g, -9, 9), nonzero(g, -9, 9));
            let k = nonzero(g, -4, 5);
            let (t1, t2) = (a * k * k, b * k);
            let v = t1 + t2 + c;
            problem(
                format!("f(x) = {}\nWhat is f({})?", quad(a, b, c), minus(k)),
                Answer::Int(v),
                vec![
                    format!("Put {} in place of every x.", minus(k)),
                    format!("{} × {}² = {} and {} × {} = {}", minus(a), wrap(k), minus(t1), minus(b), wrap(k), minus(t2)),
                    format!("{}{}{} = {}", minus(t1), signed(t2), signed(c), minus(v)),
                ],
            )
        }
        1 => {
            let (m, n, p) = (nonzero(g, -4, 5), nonzero(g, -9, 9), nonzero(g, -5, 5));
            let k = ri(g, -3, 3);
            let gk = k * k + p;
            let v = m * gk + n;
            problem(
                format!("f(x) = {}{}\ng(x) = x²{}\nWhat is f(g({}))?", coef(m, "x"), signed(n), signed(p), minus(k)),
                Answer::Int(v),
                vec![
                    format!("Work from the inside out: find g({}) first.", minus(k)),
                    format!("g({}) = {}²{} = {}", minus(k), wrap(k), signed(p), minus(gk)),
                    format!("f({}) = {} × {}{} = {}", minus(gk), minus(m), wrap(gk), signed(n), minus(v)),
                ],
            )
        }
        _ => {
            let (m, n, t) = (ri(g, 2, 6), nonzero(g, -9, 9), ri(g, -6, 9));
            let y = m * t + n;
            problem(
                format!("f(x) = {m}x{}\nWhat is f⁻¹({})?", signed(n), minus(y)),
                Answer::Int(t),
                vec![
                    format!("f⁻¹ undoes f: find the x that makes f(x) = {}.", minus(y)),
                    format!("{m}x{} = {} → {m}x = {}", signed(n), minus(y), minus(y - n)),
                    format!("x = {} ÷ {m} = {}", minus(y - n), minus(t)),
                ],
            )
        }
    }
}

pub fn inequalities(g: &mut G) -> Problem {
    let a = nonzero(g, -6, 6);
    let a = if a.abs() == 1 { a * 2 } else { a };
    let b = nonzero(g, -12, 12);
    let t = ri(g, -8, 8);
    let c = a * t + b;
    let op = pick(g, &['<', '>', '≤', '≥']);
    let flip = |o: char| match o {
        '<' => '>',
        '>' => '<',
        '≤' => '≥',
        _ => '≤',
    };
    let fin = if a < 0 { flip(op) } else { op };
    let (verb, sym) = if b > 0 { ("subtract", '−') } else { ("add", '+') };
    let rhs = c - b;
    let undo = format!(
        "Undo the {}: {verb} {} on both sides. {} {sym} {} = {}, so {} {op} {}",
        signed(b).trim(),
        b.abs(),
        minus(c),
        b.abs(),
        minus(rhs),
        coef(a, "x"),
        minus(rhs)
    );
    let last = if a < 0 {
        format!("Divide both sides by {}: {} ÷ {} = {}. Dividing by a NEGATIVE flips the sign! So x {fin} {}", minus(a), minus(rhs), wrap(a), minus(t), minus(t))
    } else {
        format!("Divide both sides by {a}: {} ÷ {a} = {}, so x {fin} {}", minus(rhs), minus(t), minus(t))
    };
    problem(
        format!("Solve for x:\n{}{} {op} {}", coef(a, "x"), signed(b), minus(c)),
        Answer::Ineq(fin, t),
        vec![
            "Solve it like an equation, keeping the inequality sign. Multiplying or dividing by a negative flips it.".to_string(),
            undo,
            last,
        ],
    )
}

pub fn absval(g: &mut G) -> Problem {
    let (a, b) = (nonzero(g, -9, 9), ri(g, 1, 10));
    let (x1, x2) = (b - a, -b - a);
    problem(
        format!("Solve for x:\n|x{}| = {b}", signed(a)),
        Answer::Roots(x1, x2),
        vec![
            format!("|something| = {b} means that something is {b} or −{b}: both are {b} steps from 0."),
            format!("x{} = {b} → x = {}", signed(a), minus(x1)),
            format!("x{} = −{b} → x = {}. So x = {} or x = {}", signed(a), minus(x2), minus(x1), minus(x2)),
        ],
    )
}

pub fn growth(g: &mut G) -> Problem {
    match ri(g, 0, 2) {
        0 => {
            let s = pick(g, &[5, 10, 20, 25, 50, 100, 150, 200]);
            let n = ri(g, 2, 5);
            let what = pick(g, &["bacteria", "followers", "rabbits", "video views"]);
            let p = 1i64 << n;
            problem(
                format!("A group of {s} {what} doubles every week. How many will there be after {n} weeks?"),
                Answer::Int(s * p),
                vec![
                    "Doubling means × 2 each time, so after n weeks: start × 2ⁿ.".to_string(),
                    format!("{s} × 2{} = {s} × {p}", sup(n)),
                    format!("= {}", s * p),
                ],
            )
        }
        1 => {
            let (k, n) = (ri(g, 5, 50), ri(g, 2, 4));
            let p = 1i64 << n;
            problem(
                format!("A {} g sample loses half of its mass every hour (its half-life is 1 hour). How many grams are left after {n} hours?", k * p),
                Answer::Int(k),
                vec![
                    "Halving means ÷ 2 each time, so after n hours: start ÷ 2ⁿ.".to_string(),
                    format!("{} ÷ 2{} = {} ÷ {p}", k * p, sup(n), k * p),
                    format!("= {k} g"),
                ],
            )
        }
        _ => {
            let (p, r, n) = loop {
                let (p, r, n): (i64, i64, i64) = (pick(g, &[500, 1000, 2000, 5000, 10000]), pick(g, &[5, 10, 20, 50]), ri(g, 2, 3));
                if p * (100 + r).pow(n as u32) % 100i64.pow(n as u32) == 0 {
                    break (p, r, n);
                }
            };
            let mult = show_dec(100 + r);
            let years: Vec<String> = (1..=n)
                .map(|i| {
                    let v = p as f64 * ((100 + r) as f64 / 100.0).powi(i as i32);
                    format!("after year {i}: ₱{}", trim(v))
                })
                .collect();
            let v = p * (100 + r).pow(n as u32) / 100i64.pow(n as u32);
            given(
problem(
                format!("₱{p} is saved at {r}% interest a year, compounded yearly. How much is there after {n} years?"),
                Answer::Int(v),
                vec![
                    format!("Each year the money is multiplied by 1 + {r}% = {mult}."),
                    format!("₱{p} × {mult}{} … {}", sup(n), years.join(", ")),
                    format!("After {n} years there is ₱{v}"),
                ],
            ),
&[("P", p), ("r", r), ("n", n)])
        }
    }
}

/// 1210.0 → "1210", 1157.63 → "1157.63".
fn trim(v: f64) -> String {
    let s = format!("{v:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

pub fn logs(g: &mut G) -> Problem {
    let kind = ri(g, 0, 2);
    let (b, k) = loop {
        let (b, k): (i64, i64) = (pick(g, &[2, 3, 4, 5, 10]), ri(g, 1, 6));
        if b.pow(k as u32) <= 100_000 && (kind != 2 || k >= 2) {
            break (b, k);
        }
    };
    let n = b.pow(k as u32);
    let powers: Vec<String> = (1..=k).map(|i| format!("{b}{} = {}", sup(i), b.pow(i as u32))).collect();
    match kind {
        0 => problem(
            format!("log{} {n} = ?", sub(b)),
            Answer::Int(k),
            vec![
                format!("log{} {n} asks: {b} to what power makes {n}?", sub(b)),
                powers.join(", "),
                format!("{b}{} = {n}, so log{} {n} = {k}", sup(k), sub(b)),
            ],
        ),
        1 => problem(
            format!("Solve for x:\n{b}ˣ = {n}"),
            Answer::Int(k),
            vec![
                format!("Write {n} as a power of {b}."),
                powers.join(", "),
                format!("{b}{} = {n}, so x = {k}", sup(k)),
            ],
        ),
        _ => {
            let p = ri(g, 1, k - 1);
            let q = k - p;
            let (bp, bq) = (b.pow(p as u32), b.pow(q as u32));
            problem(
                format!("log{} {bp} + log{} {bq} = ?", sub(b), sub(b)),
                Answer::Int(k),
                vec![
                    "Adding logs with the same base: log a + log b = log(a × b). Or find each one.".to_string(),
                    format!("log{} {bp} = {p} and log{} {bq} = {q}", sub(b), sub(b)),
                    format!("{p} + {q} = {k} (and {bp} × {bq} = {n} = {b}{})", sup(k)),
                ],
            )
        }
    }
}

// ───────────────────────── Triangle Island (Geometry & Trigonometry) ─────────────────────────

pub fn angles(g: &mut G) -> Problem {
    const NAMES: [&str; 10] = ["triangle", "quadrilateral", "pentagon", "hexagon", "heptagon", "octagon", "nonagon", "decagon", "11-sided polygon", "dodecagon"];
    match ri(g, 0, 4) {
        0 => {
            let a = ri(g, 20, 100);
            let b = ri(g, 20, 160 - a - 10);
            let c = 180 - a - b;
            problem(
                format!("Two angles of a triangle are {a}° and {b}°. What is the third angle?"),
                Answer::Int(c),
                vec!["The angles of a triangle always add up to 180°.".to_string(), format!("{a} + {b} = {}", a + b), format!("180 − {} = {c}°", a + b)],
            )
        }
        1 => {
            let a = ri(g, 5, 85);
            problem(
                format!("An angle is {a}°. What is its complement (the angle that makes 90° with it)?"),
                Answer::Int(90 - a),
                vec!["Complementary angles add up to 90°, a right angle.".to_string(), format!("{a} + ? = 90"), format!("90 − {a} = {}°", 90 - a)],
            )
        }
        2 => {
            let a = ri(g, 10, 170);
            problem(
                format!("An angle is {a}°. What is its supplement (the angle that makes a straight line, 180°, with it)?"),
                Answer::Int(180 - a),
                vec!["Supplementary angles add up to 180°, a straight line.".to_string(), format!("{a} + ? = 180"), format!("180 − {a} = {}°", 180 - a)],
            )
        }
        3 => {
            let n = ri(g, 3, 12);
            let sum = (n - 2) * 180;
            problem(
                format!("What do the inside angles of a {} add up to?", NAMES[(n - 3) as usize]),
                Answer::Int(sum),
                vec![
                    "From one corner, a polygon with n sides can be cut into n − 2 triangles, and each triangle has 180°.".to_string(),
                    format!("{n} sides → {n} − 2 = {} triangles", n - 2),
                    format!("{} × 180° = {sum}°", n - 2),
                ],
            )
        }
        _ => {
            let n = pick(g, &[3, 4, 5, 6, 8, 9, 10, 12]);
            let sum = (n - 2) * 180;
            problem(
                format!("A regular {} has {n} equal angles. How big is each one?", NAMES[(n - 3) as usize]),
                Answer::Int(sum / n),
                vec![
                    "First find the total of the inside angles: (n − 2) × 180°.".to_string(),
                    format!("({n} − 2) × 180° = {sum}°"),
                    format!("{sum}° ÷ {n} = {}°", sum / n),
                ],
            )
        }
    }
}

const IN_PI: &str = "Answer in terms of π: type the number in front of π.";

pub fn circles(g: &mut G) -> Problem {
    let r = ri(g, 2, 12);
    match ri(g, 0, 2) {
        0 => given(
problem(
            format!("A circle has a radius of {r} cm. What is its area?\n{IN_PI}"),
            Answer::Int(r * r),
            vec!["Area of a circle = π × r².".to_string(), format!("{r}² = {}", r * r), format!("Area = {}π cm²", r * r)],
        ),
&[("r", r)]),
        1 => given(
problem(
            format!("A circle has a radius of {r} cm. What is its circumference (the distance around)?\n{IN_PI}"),
            Answer::Int(2 * r),
            vec!["Circumference = 2 × π × r, which is π × the diameter.".to_string(), format!("2 × {r} = {}", 2 * r), format!("Circumference = {}π cm", 2 * r)],
        ),
&[("r", r)]),
        _ => given(
problem(
            format!("A circle has a diameter of {} cm. What is its area?\n{IN_PI}", 2 * r),
            Answer::Int(r * r),
            vec![
                format!("Area uses the radius, which is half the diameter: {} ÷ 2 = {r}.", 2 * r),
                format!("Area = π × r², and {r}² = {}", r * r),
                format!("Area = {}π cm²", r * r),
            ],
        ),
&[("d", 2 * r)]),
    }
}

pub fn volume(g: &mut G) -> Problem {
    match ri(g, 0, 4) {
        0 => {
            let (l, w, h) = (ri(g, 2, 12), ri(g, 2, 10), ri(g, 2, 10));
            given(
problem(
                format!("A box is {l} cm long, {w} cm wide and {h} cm tall. What is its volume in cubic cm?"),
                Answer::Int(l * w * h),
                vec!["Volume of a box = length × width × height.".to_string(), format!("{l} × {w} = {}", l * w), format!("{} × {h} = {} cm³", l * w, l * w * h)],
            ),
&[("l", l), ("w", w), ("h", h)])
        }
        1 => {
            let (l, w, h) = (ri(g, 2, 10), ri(g, 2, 8), ri(g, 2, 8));
            let (a, b, c) = (l * w, l * h, w * h);
            given(
problem(
                format!("A box is {l} cm long, {w} cm wide and {h} cm tall. What is its surface area in square cm?"),
                Answer::Int(2 * (a + b + c)),
                vec![
                    "A box has 6 faces in 3 matching pairs: top & bottom, front & back, left & right.".to_string(),
                    format!("{l} × {w} = {a}, {l} × {h} = {b}, {w} × {h} = {c}"),
                    format!("2 × ({a} + {b} + {c}) = 2 × {} = {} cm²", a + b + c, 2 * (a + b + c)),
                ],
            ),
&[("l", l), ("w", w), ("h", h)])
        }
        2 => {
            let (r, h) = (ri(g, 2, 8), ri(g, 2, 12));
            given(
problem(
                format!("A cylinder has a radius of {r} cm and a height of {h} cm. What is its volume?\n{IN_PI}"),
                Answer::Int(r * r * h),
                vec!["Volume of a cylinder = π × r² × h (the circle's area times the height).".to_string(), format!("{r}² = {}", r * r), format!("{} × {h} = {}, so V = {}π cm³", r * r, r * r * h, r * r * h)],
            ),
&[("r", r), ("h", h)])
        }
        3 => {
            let (r, h) = loop {
                let (r, h) = (ri(g, 2, 9), ri(g, 2, 15));
                if r * r * h % 3 == 0 {
                    break (r, h);
                }
            };
            let v = r * r * h / 3;
            given(
problem(
                format!("A cone has a radius of {r} cm and a height of {h} cm. What is its volume?\n{IN_PI}"),
                Answer::Int(v),
                vec!["Volume of a cone, one third of a cylinder:\n$V = \\frac{1}{3}πr^2h$".to_string(), format!("{r}² × {h} = {}", r * r * h), format!("{} ÷ 3 = {v}, so V = {v}π cm³", r * r * h)],
            ),
&[("r", r), ("h", h)])
        }
        _ => {
            let r = pick(g, &[3, 6, 9]);
            let v = 4 * r * r * r / 3;
            given(
problem(
                format!("A ball (sphere) has a radius of {r} cm. What is its volume?\n{IN_PI}"),
                Answer::Int(v),
                vec!["Volume of a sphere:\n$V = \\frac{4}{3}πr^3$".to_string(), format!("{r}³ = {}", r * r * r), format!("4 × {} ÷ 3 = {v}, so V = {v}π cm³", r * r * r)],
            ),
&[("r", r)])
        }
    }
}

pub fn coords(g: &mut G) -> Problem {
    let pt = |x: i64, y: i64| format!("({}, {})", minus(x), minus(y));
    if g.random_bool(0.55) {
        let (dx, dy, d) = pick(g, &[(3, 4, 5), (4, 3, 5), (6, 8, 10), (8, 6, 10), (5, 12, 13), (12, 5, 13), (9, 12, 15), (8, 15, 17)]);
        let (sx, sy) = (if g.random_bool(0.5) { 1 } else { -1 }, if g.random_bool(0.5) { 1 } else { -1 });
        let (x1, y1) = (ri(g, -6, 6), ri(g, -6, 6));
        let (x2, y2) = (x1 + sx * dx, y1 + sy * dy);
        given(
problem(
            format!("How far apart are the points {} and {}?", pt(x1, y1), pt(x2, y2)),
            Answer::Int(d),
            vec![
                "It's Pythagoras on a grid:\n$d = \\sqrt{(x_2 − x_1)^2 + (y_2 − y_1)^2}$".to_string(),
                format!("Across: {dx}, up or down: {dy}. {dx}² + {dy}² = {} + {} = {}", dx * dx, dy * dy, d * d),
                format!("${}$ = {d}", sqrt(d * d)),
            ],
        ),
&[("x1", x1), ("y1", y1), ("x2", x2), ("y2", y2)])
    } else {
        let (x1, y1) = (ri(g, -8, 8), ri(g, -8, 8));
        let (x2, y2) = (x1 + 2 * ri(g, -6, 6), y1 + 2 * ri(g, -6, 6));
        let (mx, my) = ((x1 + x2) / 2, (y1 + y2) / 2);
        given(
problem(
            format!("What is the midpoint of {} and {}?", pt(x1, y1), pt(x2, y2)),
            Answer::Pair(mx, my),
            vec![
                "The midpoint is the average of the x's and of the y's:\n$(\\frac{x_1 + x_2}{2}, \\frac{y_1 + y_2}{2})$".to_string(),
                format!("x: ${}$ = {}", frac(format!("{}{}", minus(x1), signed(x2)), 2), minus(mx)),
                format!("y: ${}$ = {}, so the midpoint is {}", frac(format!("{}{}", minus(y1), signed(y2)), 2), minus(my), pt(mx, my)),
            ],
        ),
&[("x1", x1), ("y1", y1), ("x2", x2), ("y2", y2)])
    }
}

pub fn similar(g: &mut G) -> Problem {
    let (a, b, k) = (ri(g, 2, 9), ri(g, 2, 9), ri(g, 2, 4));
    if g.random_bool(0.5) {
        problem(
            format!("Triangles ABC and DEF are similar. AB = {a} and DE = {}. If BC = {b}, how long is EF?", a * k),
            Answer::Int(b * k),
            vec![
                "Similar shapes have all their sides in the same ratio (the scale factor).".to_string(),
                format!("DE ÷ AB = {} ÷ {a} = {k}, so DEF is {k} times as big.", a * k),
                format!("EF = BC × {k} = {b} × {k} = {}", b * k),
            ],
        )
    } else {
        problem(
            format!("Triangles ABC and DEF are similar. AB = {} and DE = {a}. If BC = {}, how long is EF?", a * k, b * k),
            Answer::Int(b),
            vec![
                "Similar shapes have all their sides in the same ratio (the scale factor).".to_string(),
                format!("AB ÷ DE = {} ÷ {a} = {k}, so DEF is {k} times smaller.", a * k),
                format!("EF = BC ÷ {k} = {} ÷ {k} = {b}", b * k),
            ],
        )
    }
}

pub fn trigratios(g: &mut G) -> Problem {
    let (o, a, h) = pick(g, &[(3, 4, 5), (4, 3, 5), (5, 12, 13), (12, 5, 13), (8, 15, 17), (15, 8, 17), (7, 24, 25), (24, 7, 25)]);
    if g.random_bool(0.6) {
        let (which, n, d, top, bottom) = match ri(g, 0, 2) {
            0 => ("sin", o, h, "opposite", "hypotenuse"),
            1 => ("cos", a, h, "adjacent", "hypotenuse"),
            _ => ("tan", o, a, "opposite", "adjacent"),
        };
        with(
right(a as f64, o as f64, "A", a.to_string(), o.to_string(), h.to_string()),
given(
problem(
            format!("In a right triangle, the side opposite angle A is {o}, the side next to it (adjacent) is {a}, and the hypotenuse is {h}.\nWhat is {which} A?"),
            Answer::frac(n, d),
            vec![
                "SOH-CAH-TOA: Sin = Opposite/Hypotenuse, Cos = Adjacent/Hypotenuse, Tan = Opposite/Adjacent.".to_string(),
                format!("${which} A = {} = {}$", frac(top, bottom), frac(n, d)),
                simplify_hint(n, d),
            ],
        ),
&[("opp", o), ("adj", a), ("hyp", h)]))
    } else {
        let k = ri(g, 2, 4);
        with(
right(a as f64, o as f64, "A", "", "?", (h * k).to_string()),
given(
problem(
            format!("In a right triangle,\n$\\sin A = {}$\nand the hypotenuse is {}. How long is the side opposite A?", frac(o, h), h * k),
            Answer::Int(o * k),
            vec![
                "$\\sin A = \\frac{opposite}{hypotenuse}$\nso opposite = sin A × hypotenuse.".to_string(),
                format!("${} × {}$ = {o} × ({} ÷ {h})", frac(o, h), h * k, h * k),
                format!("= {o} × {k} = {}", o * k),
            ],
        ),
&[("hyp", h * k)]))
    }
}

pub fn specialangles(g: &mut G) -> Problem {
    match ri(g, 0, 2) {
        0 => {
            let (f, n, d, why) = pick(
                g,
                &[
                    ("sin 30°", 1, 2, "In a 30-60-90 triangle, the side opposite 30° is half the hypotenuse."),
                    ("cos 60°", 1, 2, "The side next to 60° is the short side, half the hypotenuse."),
                    ("tan 45°", 1, 1, "A 45-45-90 triangle has two equal legs, so opposite ÷ adjacent = 1."),
                    ("sin 90°", 1, 1, "On the unit circle, 90° points straight up to (0, 1), and sin is the y."),
                    ("cos 0°", 1, 1, "On the unit circle, 0° points right to (1, 0), and cos is the x."),
                    ("sin 0°", 0, 1, "0° points to (1, 0); sin is the y, which is 0."),
                    ("cos 90°", 0, 1, "90° points to (0, 1); cos is the x, which is 0."),
                    ("sin 150°", 1, 2, "150° is 30° short of 180°: the same height as 30°, so sin 150° = sin 30°."),
                    ("cos 120°", -1, 2, "120° is on the left side, where x is negative: cos 120° = −cos 60°."),
                    ("tan 135°", -1, 1, "135° is in the top-left: x and y are the same size, but x is negative."),
                ],
            );
            with(
Figure::Unit { deg: f.split(' ').nth(1).and_then(|d| d.trim_end_matches('°').parse().ok()).unwrap_or(0.0), label: f.split(' ').nth(1).unwrap_or("").to_string() },
problem(
                format!("{f} = ?"),
                Answer::frac(n, d),
                vec!["Remember the special triangles: 30-60-90 has sides 1 : √3 : 2, and 45-45-90 has sides 1 : 1 : √2.".to_string(), why.to_string(), format!("So {f} = {}", mfr(n, d))],
            ))
        }
        1 => {
            let (r, top, bottom, deg) = pick(
                g,
                &[("π", 1, 6, 30), ("π", 1, 4, 45), ("π", 1, 3, 60), ("π", 1, 2, 90), ("2π", 2, 3, 120), ("3π", 3, 4, 135), ("5π", 5, 6, 150), ("π", 1, 1, 180), ("3π", 3, 2, 270), ("2π", 2, 1, 360)],
            );
            with(
Figure::Unit { deg: deg as f64, label: if bottom == 1 { r.to_string() } else { format!("{r}/{bottom}") } },
problem(
                format!("How many degrees is this angle?\n${}$ radians", if bottom == 1 { r.to_string() } else { frac(r, bottom) }),
                Answer::Int(deg),
                vec![
                    "π radians is half a turn: 180°.".to_string(),
                    match (top, bottom) {
                        (1, 1) => "So π radians = 180°".to_string(),
                        (1, _) => format!("So replace π with 180°: 180 ÷ {bottom}"),
                        (_, 1) => format!("So replace π with 180°: {top} × 180"),
                        _ => format!("So replace π with 180°: {top} × 180 ÷ {bottom}"),
                    },
                    format!("= {deg}°"),
                ],
            ))
        }
        _ => {
            let h = 2 * ri(g, 2, 15);
            with(
right(3f64.sqrt(), 1.0, "30°", "", "?", h.to_string()),
problem(
                format!("In a right triangle, angle A is 30° and the hypotenuse is {h}. How long is the side opposite A?"),
                Answer::Int(h / 2),
                vec!["sin 30° = $\\frac{1}{2}$, and sin A = opposite ÷ hypotenuse.".to_string(), format!("opposite = ${} × {h}$", frac(1, 2)), format!("= {}", h / 2)],
            ))
        }
    }
}

pub fn lawsincos(g: &mut G) -> Problem {
    if g.random_bool(0.65) {
        let wide = g.random_bool(0.4);
        let (a, b, c) = loop {
            let (a, b) = (ri(g, 2, 20), ri(g, 2, 20));
            let c2 = a * a + b * b + if wide { a * b } else { -a * b };
            if a != b && is_square(c2) {
                break (a, b, (c2 as f64).sqrt().round() as i64);
            }
        };
        let (deg, op) = if wide { (120, '+') } else { (60, '−') };
        with(
Figure::Sas { a: a as f64, b: b as f64, deg: deg as f64, a_label: a.to_string(), b_label: b.to_string(), angle: format!("{deg}°"), third: "?".to_string() },
given(
problem(
            format!("In a triangle, two sides are {a} and {b}, and the angle between them is {deg}°. How long is the third side?"),
            Answer::Int(c),
            vec![
                "Law of cosines:\n$c^2 = a^2 + b^2 − 2ab \\cos C$".to_string(),
                format!(
                    "cos {deg}° = {}, so 2ab cos C = {}{}.\n$c^2 = {a}^2 + {b}^2 {op} {}$ = {}",
                    if wide { format!("−{}", mfr(1, 2)) } else { mfr(1, 2) },
                    if wide { "−" } else { "" },
                    a * b,
                    a * b,
                    c * c
                ),
                format!("$c = {}$ = {c}", sqrt(c * c)),
            ],
        ),
&[("a", a), ("b", b), ("C", deg)]))
    } else {
        let a = ri(g, 2, 15);
        with(
right(3f64.sqrt(), 1.0, "30°", "", &format!("a = {a}"), "b = ?"),
given(
problem(
            format!("In triangle ABC, angle A = 30° and angle B = 90°. Side a (opposite A) is {a}. How long is side b (opposite B)?"),
            Answer::Int(2 * a),
            vec![
                "Law of sines:\n$\\frac{a}{\\sin A} = \\frac{b}{\\sin B}$".to_string(),
                format!("sin 30° = {} and sin 90° = 1, so\n${} = {}$", mfr(1, 2), frac(a, frac(1, 2)), frac("b", 1)),
                format!("{a} × 2 = {}, so b = {}", 2 * a, 2 * a),
            ],
        ),
&[("a", a), ("A", 30), ("B", 90)]))
    }
}

// ───────────────────────── Chance Carnival (Statistics & Probability) ─────────────────────────

pub fn average(g: &mut G) -> Problem {
    let n = pick(g, &[5usize, 7]);
    let what = pick(g, &["test scores", "goals scored", "hours of sleep", "mangoes picked", "books read"]);
    let (ask, xs, answer, hints): (&str, Vec<i64>, i64, Vec<String>) = match ri(g, 0, 3) {
        0 => loop {
            let xs: Vec<i64> = (0..n).map(|_| ri(g, 2, 20)).collect();
            let sum: i64 = xs.iter().sum();
            if sum % n as i64 == 0 {
                let mean = sum / n as i64;
                let adds = xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" + ");
                break (
                    "mean (average)",
                    xs,
                    mean,
                    vec!["Mean = add them all up, then divide by how many there are.".to_string(), format!("{adds} = {sum}"), format!("{sum} ÷ {n} = {mean}")],
                );
            }
        },
        1 => {
            let xs: Vec<i64> = (0..n).map(|_| ri(g, 2, 20)).collect();
            let mut sorted = xs.clone();
            sorted.sort();
            let m = sorted[n / 2];
            (
                "median",
                xs,
                m,
                vec!["Median = the middle number once they're in order.".to_string(), format!("In order: {}", list(&sorted)), format!("The middle one (number {} of {n}) is {m}", n / 2 + 1)],
            )
        }
        2 => loop {
            let m = ri(g, 2, 20);
            let mut xs: Vec<i64> = vec![m, m, m];
            while xs.len() < n {
                xs.push(ri(g, 2, 20));
            }
            let count = |xs: &[i64], v: i64| xs.iter().filter(|&&x| x == v).count();
            if xs.iter().all(|&x| x == m || count(&xs, x) < count(&xs, m)) {
                for i in (1..xs.len()).rev() {
                    let j = g.random_range(0..=i);
                    xs.swap(i, j);
                }
                let c = count(&xs, m);
                break ("mode", xs, m, vec!["Mode = the number that shows up most often.".to_string(), format!("Count them: {m} appears {c} times."), format!("So the mode is {m}")]);
            }
        },
        _ => {
            let xs: Vec<i64> = (0..n).map(|_| ri(g, 2, 30)).collect();
            let (hi, lo) = (*xs.iter().max().unwrap(), *xs.iter().min().unwrap());
            ("range", xs, hi - lo, vec!["Range = biggest − smallest.".to_string(), format!("Biggest: {hi}, smallest: {lo}"), format!("{hi} − {lo} = {}", hi - lo)])
        }
    };
    problem(format!("Here are some {what}:\n{}\nWhat is the {ask}?", list(&xs)), Answer::Int(answer), hints)
}

pub fn probability(g: &mut G) -> Problem {
    match ri(g, 0, 2) {
        0 => {
            let (r, b, gr) = (ri(g, 1, 8), ri(g, 1, 8), ri(g, 1, 8));
            let total = r + b + gr;
            let (color, count) = pick(g, &[("red", r), ("blue", b), ("green", gr)]);
            let not = g.random_bool(0.35);
            let fav = if not { total - count } else { count };
            problem(
                format!("A bag has {r} red, {b} blue and {gr} green marbles. You pick one without looking. What is the probability that it is {}{color}?", if not { "NOT " } else { "" }),
                Answer::frac(fav, total),
                vec![
                    "Probability:\n$P = \\frac{ways you want}{all the ways}$".to_string(),
                    if not { format!("{total} marbles, and {total} − {count} = {fav} are not {color}: {}", mfr(fav, total)) } else { format!("{count} {color} out of {total} marbles: {}", mfr(count, total)) },
                    simplify_hint(fav, total),
                ],
            )
        }
        1 => {
            let (event, which, fav) = pick(
                g,
                &[
                    ("an even number", "2, 4, 6", 3),
                    ("a number greater than 4", "5, 6", 2),
                    ("a 6", "6", 1),
                    ("a prime number", "2, 3, 5", 3),
                    ("a number less than 3", "1, 2", 2),
                    ("a multiple of 3", "3, 6", 2),
                ],
            );
            problem(
                format!("You roll one die. What is the probability of rolling {event}?"),
                Answer::frac(fav, 6),
                vec![
                    "A die has 6 sides, all equally likely.".to_string(),
                    format!("The ones that count: {which}. That's {fav} out of 6: {}", mfr(fav, 6)),
                    simplify_hint(fav, 6),
                ],
            )
        }
        _ => {
            let word = pick(g, &["BANANA", "MATHEMATICS", "PILIPINAS", "ELEPHANT", "CALCULATOR", "PROBABILITY"]);
            let vowels = word.chars().filter(|c| "AEIOU".contains(*c)).count() as i64;
            let total = word.len() as i64;
            problem(
                format!("One letter is picked at random from the word {word}. What is the probability that it is a vowel (A, E, I, O, U)?"),
                Answer::frac(vowels, total),
                vec![
                    "Count the letters you want, and all the letters.".to_string(),
                    format!("{word} has {total} letters, and {vowels} of them are vowels: {}", mfr(vowels, total)),
                    simplify_hint(vowels, total),
                ],
            )
        }
    }
}

pub fn compound(g: &mut G) -> Problem {
    match ri(g, 0, 3) {
        0 => {
            let n = ri(g, 2, 4);
            let d = 1i64 << n;
            problem(
                format!("You flip a coin {n} times. What is the probability of getting heads every time?"),
                Answer::frac(1, d),
                vec![
                    "Each flip doesn't care about the others (they're independent), so multiply the chances.".to_string(),
                    format!("${} = {}$", vec![frac(1, 2); n as usize].join(" × "), frac(1, d)),
                    format!("So the probability is {}", mfr(1, d)),
                ],
            )
        }
        1 => {
            let s = ri(g, 2, 12);
            let pairs: Vec<String> = (1..=6).filter(|a| (1..=6).contains(&(s - a))).map(|a| format!("{a}+{}", s - a)).collect();
            let c = pairs.len() as i64;
            problem(
                format!("You roll two dice. What is the probability that the total is {s}?"),
                Answer::frac(c, 36),
                vec!["Two dice make 6 × 6 = 36 equally likely pairs.".to_string(), format!("Pairs that make {s}: {}. That's {c} of them: {}", pairs.join(", "), mfr(c, 36)), simplify_hint(c, 36)],
            )
        }
        2 => {
            let (r, b) = (ri(g, 1, 6), ri(g, 1, 6));
            let t = r + b;
            problem(
                format!("A bag has {r} red and {b} blue marbles. You pick one, put it back, then pick again. What is the probability that both are red?"),
                Answer::frac(r * r, t * t),
                vec![
                    "Putting it back means both picks have the same chance. Multiply them.".to_string(),
                    format!("${} × {} = {}$", frac(r, t), frac(r, t), frac(r * r, t * t)),
                    simplify_hint(r * r, t * t),
                ],
            )
        }
        _ => {
            let (r, b) = (ri(g, 2, 6), ri(g, 1, 6));
            let t = r + b;
            problem(
                format!("A bag has {r} red and {b} blue marbles. You pick two without putting the first back. What is the probability that both are red?"),
                Answer::frac(r * (r - 1), t * (t - 1)),
                vec![
                    "Without putting it back, the second pick has one fewer red and one fewer marble.".to_string(),
                    format!("${} × {} = {}$", frac(r, t), frac(r - 1, t - 1), frac(r * (r - 1), t * (t - 1))),
                    simplify_hint(r * (r - 1), t * (t - 1)),
                ],
            )
        }
    }
}

pub fn counting(g: &mut G) -> Problem {
    let fact = |n: i64| (1..=n).product::<i64>();
    match ri(g, 0, 3) {
        0 => {
            let n = ri(g, 3, 7);
            let chain = (1..=n).rev().map(|k| k.to_string()).collect::<Vec<_>>().join(" × ");
            problem(
                format!("In how many different orders can {n} friends stand in a line?"),
                Answer::Int(fact(n)),
                vec![
                    format!("The first spot has {n} choices, the next {}, and so on down to 1.", n - 1),
                    format!("{n}! = {chain}"),
                    format!("= {}", fact(n)),
                ],
            )
        }
        1 => {
            let (n, r) = (ri(g, 5, 10), ri(g, 2, 3));
            let p: i64 = (0..r).map(|i| n - i).product();
            let chain = (0..r).map(|i| (n - i).to_string()).collect::<Vec<_>>().join(" × ");
            given(
problem(
                format!("{n} runners are in a race. In how many ways can the top {r} places (1st, 2nd{}) be given out?", if r == 3 { ", 3rd" } else { "" }),
                Answer::Int(p),
                vec![
                    "Order matters here (1st is not 2nd), so it's a permutation: count the choices for each place.".to_string(),
                    format!("1st: {n} choices, then one fewer each time: {chain}"),
                    format!("= {p}"),
                ],
            ),
&[("n", n), ("r", r)])
        }
        2 => {
            let (n, r) = (ri(g, 4, 10), ri(g, 2, 3));
            let p: i64 = (0..r).map(|i| n - i).product();
            let c = p / fact(r);
            given(
problem(
                format!("In how many ways can you choose {r} kids out of {n} for a team? (Order doesn't matter.)"),
                Answer::Int(c),
                vec![
                    "Order doesn't matter, so it's a combination:\n$nCr = \\frac{n!}{r!(n − r)!}$, or (ordered picks) ÷ r!".to_string(),
                    format!("Ordered picks: {}. Each team was counted {r}! = {} times.", p, fact(r)),
                    format!("{p} ÷ {} = {c}", fact(r)),
                ],
            ),
&[("n", n), ("r", r)])
        }
        _ => {
            let (s, p, h) = (ri(g, 2, 6), ri(g, 2, 5), ri(g, 2, 4));
            problem(
                format!("You have {s} shirts, {p} pairs of pants and {h} hats. How many different outfits (one of each) can you make?"),
                Answer::Int(s * p * h),
                vec![
                    "Each shirt can go with each pair of pants and each hat: multiply the choices.".to_string(),
                    format!("{s} × {p} = {} shirt-and-pants combos", s * p),
                    format!("{} × {h} = {}", s * p, s * p * h),
                ],
            )
        }
    }
}

pub fn expected(g: &mut G) -> Problem {
    match ri(g, 0, 2) {
        0 => {
            let n = pick(g, &[10, 20, 30, 50, 100]);
            problem(
                format!("You flip a fair coin {n} times. How many heads do you expect?"),
                Answer::Int(n / 2),
                vec!["Expected count = number of tries × chance each time.".to_string(), format!("{n} × 1/2"), format!("= {}", n / 2)],
            )
        }
        1 => {
            let prizes: Vec<i64> = (0..4).map(|_| pick(g, &[0, 0, 5, 10, 20, 50])).collect();
            let sum: i64 = prizes.iter().sum();
            let shown = prizes.iter().map(|p| format!("₱{p}")).collect::<Vec<_>>().join(", ");
            problem(
                format!("A spinner has 4 equal parts with prizes {shown}. What is the expected prize for one spin (in pesos)?"),
                Answer::Dec(sum * 25),
                vec![
                    "Expected value = each prize × its chance, all added up.".to_string(),
                    format!("Each part has a {} chance:\n$({}) × {}$", mfr(1, 4), prizes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" + "), frac(1, 4)),
                    format!("{sum} ÷ 4 = ₱{}", show_dec(sum * 25)),
                ],
            )
        }
        _ => {
            let (w, l) = loop {
                let (w, l) = (ri(g, 6, 30), ri(g, 1, 4));
                if (w - 5 * l) % 6 == 0 {
                    break (w, l);
                }
            };
            let e = (w - 5 * l) / 6;
            problem(
                format!("In a game you roll a die. Rolling a 6 wins ₱{w}; anything else loses ₱{l}. What is the expected amount you win per game (in pesos, negative if you lose)?"),
                Answer::Dec(e * 100),
                vec![
                    "Expected value = (win × chance of winning) + (loss × chance of losing).".to_string(),
                    format!("${w} × {} + (−{l}) × {} = {} − {}$", frac(1, 6), frac(5, 6), frac(w, 6), frac(5 * l, 6)),
                    format!("$= {}$ = {}", fm(w - 5 * l, 6), minus(e)),
                ],
            )
        }
    }
}
