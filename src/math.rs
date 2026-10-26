//! Real-looking math in prompts, hints and lessons. Inside text, anything
//! between `$…$` is a formula in a tiny LaTeX-like syntax:
//!
//! - `\frac{a}{b}` — a stacked fraction
//! - `\sqrt{x}`    — a square root with the bar over x
//! - `^{…}` / `^x` — raised (exponent), `_{…}` / `_x` — lowered (subscript)
//!
//! Everything else is written as is (the levels already use × ÷ − ± π …).
//! Formulas are built from GTK widgets, so they follow the surrounding font
//! and size. `plain` turns the same text back into one line for places that
//! can't draw (the score sheet, CSV).

use gtk::prelude::*;

#[derive(Debug, PartialEq)]
enum Node {
    Text(String),
    Frac(Vec<Node>, Vec<Node>),
    Sqrt(Vec<Node>),
    Sup(Vec<Node>),
    Sub(Vec<Node>),
}

struct Parser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Parser<'_> {
    /// Nodes up to the end, or up to the `}` closing the current group.
    fn nodes(&mut self) -> Vec<Node> {
        let mut out = Vec::new();
        let mut text = String::new();
        let flush = |text: &mut String, out: &mut Vec<Node>| {
            if !text.is_empty() {
                out.push(Node::Text(std::mem::take(text)));
            }
        };
        while let Some(&c) = self.chars.peek() {
            match c {
                '}' => {
                    self.chars.next();
                    break;
                }
                '\\' => {
                    self.chars.next();
                    let mut name = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if !c.is_ascii_alphabetic() {
                            break;
                        }
                        name.push(c);
                        self.chars.next();
                    }
                    match name.as_str() {
                        "frac" => {
                            flush(&mut text, &mut out);
                            let (a, b) = (self.group(), self.group());
                            out.push(Node::Frac(a, b));
                        }
                        "sqrt" => {
                            flush(&mut text, &mut out);
                            let a = self.group();
                            out.push(Node::Sqrt(a));
                        }
                        "" => {
                            // "\{" style escapes: keep the next character
                            if let Some(c) = self.chars.next() {
                                text.push(c);
                            }
                        }
                        other => text.push_str(other),
                    }
                }
                '^' | '_' => {
                    self.chars.next();
                    flush(&mut text, &mut out);
                    let a = self.group();
                    out.push(if c == '^' { Node::Sup(a) } else { Node::Sub(a) });
                }
                _ => {
                    text.push(c);
                    self.chars.next();
                }
            }
        }
        flush(&mut text, &mut out);
        out
    }

    /// `{…}`, or a single character.
    fn group(&mut self) -> Vec<Node> {
        match self.chars.peek() {
            Some('{') => {
                self.chars.next();
                self.nodes()
            }
            Some(_) => vec![Node::Text(self.chars.next().unwrap().to_string())],
            None => Vec::new(),
        }
    }
}

fn parse(s: &str) -> Vec<Node> {
    Parser { chars: s.chars().peekable() }.nodes()
}

/// Text only (no fractions or roots inside), so it can go in Pango markup.
fn simple(nodes: &[Node]) -> Option<String> {
    let mut s = String::new();
    for n in nodes {
        match n {
            Node::Text(t) => s.push_str(&gtk::glib::markup_escape_text(t)),
            // Real superscript characters (², ³, ⁻¹) read better than Pango's tiny <sup>
            Node::Sup(a) => match crate::expr::superscript(&flat(a)) {
                Some(sup) if a.iter().all(|n| matches!(n, Node::Text(_))) => s.push_str(&sup),
                _ => s.push_str(&format!("<sup>{}</sup>", simple(a)?)),
            },
            Node::Sub(a) => s.push_str(&format!("<sub>{}</sub>", simple(a)?)),
            _ => return None,
        }
    }
    Some(s)
}

/// A row of math, vertically centered so fractions sit on the text's middle.
fn row(nodes: &[Node]) -> gtk::Box {
    let b = gtk::Box::builder().valign(gtk::Align::Center).build();
    let mut markup = String::new();
    let flush = |markup: &mut String, b: &gtk::Box| {
        if !markup.is_empty() {
            let l = gtk::Label::builder().use_markup(true).label(std::mem::take(markup)).valign(gtk::Align::Center).build();
            b.append(&l);
        }
    };
    for n in nodes {
        if let Some(m) = simple(std::slice::from_ref(n)) {
            markup.push_str(&m);
            continue;
        }
        flush(&mut markup, &b);
        match n {
            Node::Frac(top, bottom) => {
                let f = gtk::Box::builder().orientation(gtk::Orientation::Vertical).valign(gtk::Align::Center).css_classes(["frac"]).build();
                let t = row(top);
                t.set_halign(gtk::Align::Center);
                let d = row(bottom);
                d.set_halign(gtk::Align::Center);
                f.append(&t);
                f.append(&gtk::Separator::builder().css_classes(["frac-bar"]).build());
                f.append(&d);
                b.append(&f);
            }
            Node::Sqrt(inner) => {
                let r = gtk::Box::builder().valign(gtk::Align::Center).build();
                r.append(&gtk::Label::builder().label("√").css_classes(["root-sign"]).valign(gtk::Align::Center).build());
                let under = row(inner);
                under.add_css_class("radicand");
                r.append(&under);
                b.append(&r);
            }
            Node::Sup(inner) | Node::Sub(inner) => {
                let s = row(inner);
                s.add_css_class("script");
                s.set_valign(if matches!(n, Node::Sup(_)) { gtk::Align::Start } else { gtk::Align::End });
                b.append(&s);
            }
            Node::Text(_) => unreachable!("plain text goes through markup"),
        }
    }
    flush(&mut markup, &b);
    b
}

/// One line with `$…$` formulas in it, laid out left to right.
fn math_line(line: &str, align: gtk::Align) -> gtk::Box {
    let b = gtk::Box::builder().halign(align).valign(gtk::Align::Center).build();
    for (i, part) in line.split('$').enumerate() {
        if part.is_empty() {
            continue;
        }
        if i % 2 == 1 {
            let r = row(&parse(part));
            r.add_css_class("math");
            b.append(&r);
        } else {
            b.append(&gtk::Label::builder().label(part).valign(gtk::Align::Center).build());
        }
    }
    b
}

/// Text that may hold formulas, as a widget: runs of ordinary lines become
/// one wrapped label; lines with `$…$` are drawn as math.
pub fn rich(text: &str, align: gtk::Align) -> gtk::Box {
    let out = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(6).halign(if align == gtk::Align::Center { align } else { gtk::Align::Fill }).build();
    let mut para: Vec<&str> = Vec::new();
    let flush = |para: &mut Vec<&str>, out: &gtk::Box| {
        if para.is_empty() {
            return;
        }
        let l = gtk::Label::builder().label(para.join("\n")).wrap(true).build();
        l.set_xalign(if align == gtk::Align::Center { 0.5 } else { 0.0 });
        l.set_justify(if align == gtk::Align::Center { gtk::Justification::Center } else { gtk::Justification::Left });
        out.append(&l);
        para.clear();
    };
    for line in text.lines() {
        if line.contains('$') {
            flush(&mut para, &out);
            out.append(&math_line(line, align));
        } else {
            para.push(line);
        }
    }
    flush(&mut para, &out);
    out
}

/// Replace a container's content with `text`, drawn with `rich`.
pub fn set(container: &gtk::Box, text: &str, align: gtk::Align) {
    while let Some(c) = container.first_child() {
        container.remove(&c);
    }
    container.append(&rich(text, align));
}

fn flat(nodes: &[Node]) -> String {
    let wrap = |s: String| if s.chars().count() > 1 && s.contains([' ', '+', '−', '-', '×', '÷']) { format!("({s})") } else { s };
    nodes
        .iter()
        .map(|n| match n {
            Node::Text(t) => t.clone(),
            Node::Frac(a, b) => format!("{}/{}", wrap(flat(a)), wrap(flat(b))),
            Node::Sqrt(a) => format!("√{}", wrap(flat(a))),
            Node::Sup(a) => {
                let s = flat(a);
                crate::expr::superscript(&s).unwrap_or_else(|| format!("^{}", wrap(s)))
            }
            Node::Sub(a) => flat(a),
        })
        .collect()
}

/// The same text on plain lines: `$\frac{3}{4}$` → `3/4`.
pub fn plain(text: &str) -> String {
    text.split('$').enumerate().map(|(i, part)| if i % 2 == 1 { flat(&parse(part)) } else { part.to_string() }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_formulas() {
        let n = parse(r"x = \frac{−b ± \sqrt{b^2 − 4ac}}{2a}");
        assert!(matches!(&n[1], Node::Frac(top, _) if matches!(top[1], Node::Sqrt(_))));
    }

    #[test]
    fn plain_text() {
        assert_eq!(plain(r"$\frac{3}{4} + \frac{1}{2}$ = ?"), "3/4 + 1/2 = ?");
        assert_eq!(plain(r"$x = \frac{−b ± \sqrt{b^2 − 4ac}}{2a}$"), "x = (−b ± √(b² − 4ac))/2a");
        assert_eq!(plain(r"Simplify $\sqrt{72}$"), "Simplify √72");
        assert_eq!(plain(r"$2^{-3}$"), "2⁻³");
        assert_eq!(plain("no math here"), "no math here");
    }
}
