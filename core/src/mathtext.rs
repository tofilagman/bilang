//! The text side of formulas: anything between `$…$` in prompts, hints and
//! lessons is written in a tiny LaTeX-like syntax (`\frac{a}{b}`,
//! `\sqrt{x}`, `^{…}`, `_{…}`). Apps parse it here and draw it their own way;
//! `plain` turns it back into one line for places that can't draw (score
//! sheet, CSV).

#[derive(Debug, PartialEq)]
pub enum Node {
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

pub fn parse(s: &str) -> Vec<Node> {
    Parser { chars: s.chars().peekable() }.nodes()
}

pub fn flat(nodes: &[Node]) -> String {
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
