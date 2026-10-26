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

use bilang_core::mathtext::{Node, flat, parse};
use gtk::prelude::*;


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
