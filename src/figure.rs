//! Labeled diagrams for the triangle and trig levels: a right triangle with
//! its angle and sides, two sides with the angle between them, and the unit
//! circle. Drawn to scale from each problem's numbers, with the side to find
//! marked "?" in orange.

use gtk::{cairo, prelude::*};

#[derive(Clone, Debug, PartialEq)]
pub enum Figure {
    /// A right triangle: angle A at the bottom left, the right angle at the
    /// bottom right. `adj` and `opp` are lengths (for the shape); labels may be "".
    Right { adj: f64, opp: f64, angle: String, adj_label: String, opp_label: String, hyp_label: String },
    /// Two sides and the angle between them (in degrees); the third side is dashed.
    Sas { a: f64, b: f64, deg: f64, a_label: String, b_label: String, angle: String, third: String },
    /// The unit circle with an angle swept from 0°, and its height (sin) dashed.
    Unit { deg: f64, label: String },
}

const INK: (f64, f64, f64) = (0.18, 0.18, 0.35);
const ACCENT: (f64, f64, f64) = (0.98, 0.52, 0.0);
const SOFT: (f64, f64, f64) = (0.25, 0.5, 0.88);

fn rgb(cr: &cairo::Context, c: (f64, f64, f64)) {
    cr.set_source_rgb(c.0, c.1, c.2);
}

/// Draw `text` beside a point: `dir` is the (unit) direction to push it, so
/// a label never sits on its line.
fn label(cr: &cairo::Context, area: &gtk::DrawingArea, text: &str, at: (f64, f64), dir: (f64, f64), gap: f64) {
    if text.is_empty() {
        return;
    }
    let layout = area.create_pango_layout(Some(text));
    let (w, h) = layout.pixel_size();
    let (w, h) = (w as f64, h as f64);
    let push = gap + 0.5 * (dir.0.abs() * w + dir.1.abs() * h);
    let (cx, cy) = (at.0 + dir.0 * push, at.1 + dir.1 * push);
    rgb(cr, if text.contains('?') { ACCENT } else { INK });
    cr.move_to(cx - w / 2.0, cy - h / 2.0);
    pangocairo::functions::show_layout(cr, &layout);
}

fn norm(v: (f64, f64)) -> (f64, f64) {
    let l = (v.0 * v.0 + v.1 * v.1).sqrt().max(1e-9);
    (v.0 / l, v.1 / l)
}

/// The outward normal of segment p→q, pointing away from `inside`.
fn outward(p: (f64, f64), q: (f64, f64), inside: (f64, f64)) -> (f64, f64) {
    let n = norm((-(q.1 - p.1), q.0 - p.0));
    let mid = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
    if n.0 * (mid.0 - inside.0) + n.1 * (mid.1 - inside.1) < 0.0 { (-n.0, -n.1) } else { n }
}

fn mid(p: (f64, f64), q: (f64, f64)) -> (f64, f64) {
    ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0)
}

/// Fit model points (y up) into the area with room for labels; returns screen points.
fn fit(pts: &[(f64, f64)], w: f64, h: f64, pad: f64) -> Vec<(f64, f64)> {
    let (minx, maxx) = pts.iter().fold((f64::MAX, f64::MIN), |(a, b), p| (a.min(p.0), b.max(p.0)));
    let (miny, maxy) = pts.iter().fold((f64::MAX, f64::MIN), |(a, b), p| (a.min(p.1), b.max(p.1)));
    let s = ((w - 2.0 * pad) / (maxx - minx).max(1e-9)).min((h - 2.0 * pad) / (maxy - miny).max(1e-9));
    let (ox, oy) = ((w - s * (maxx - minx)) / 2.0, (h - s * (maxy - miny)) / 2.0);
    pts.iter().map(|p| (ox + s * (p.0 - minx), h - oy - s * (p.1 - miny))).collect()
}

fn draw(fig: &Figure, area: &gtk::DrawingArea, cr: &cairo::Context, w: f64, h: f64) {
    cr.set_line_join(cairo::LineJoin::Round);
    cr.set_line_cap(cairo::LineCap::Round);
    match fig {
        Figure::Right { adj, opp, angle, adj_label, opp_label, hyp_label } => {
            let p = fit(&[(0.0, 0.0), (*adj, 0.0), (*adj, *opp)], w, h, 46.0);
            let (a, c, b) = (p[0], p[1], p[2]);
            let inside = ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0);
            // the triangle
            rgb(cr, (0.94, 0.96, 1.0));
            cr.move_to(a.0, a.1);
            cr.line_to(c.0, c.1);
            cr.line_to(b.0, b.1);
            cr.close_path();
            cr.fill_preserve().ok();
            rgb(cr, INK);
            cr.set_line_width(3.0);
            cr.stroke().ok();
            // the right angle
            cr.set_line_width(2.0);
            let q = 14.0;
            cr.move_to(c.0 - q, c.1);
            cr.line_to(c.0 - q, c.1 - q);
            cr.line_to(c.0, c.1 - q);
            cr.stroke().ok();
            // angle A
            if !angle.is_empty() {
                let theta = (opp / adj).atan();
                rgb(cr, ACCENT);
                cr.set_line_width(2.5);
                cr.arc_negative(a.0, a.1, 34.0, 0.0, -theta);
                cr.stroke().ok();
                label(cr, area, angle, (a.0 + 34.0 * (theta / 2.0).cos(), a.1 - 34.0 * (theta / 2.0).sin()), (((theta / 2.0).cos()), -(theta / 2.0).sin()), 6.0);
            }
            label(cr, area, adj_label, mid(a, c), (0.0, 1.0), 6.0);
            label(cr, area, opp_label, mid(c, b), (1.0, 0.0), 8.0);
            label(cr, area, hyp_label, mid(a, b), outward(a, b, inside), 8.0);
        }
        Figure::Sas { a, b, deg, a_label, b_label, angle, third } => {
            let t = deg.to_radians();
            let p = fit(&[(0.0, 0.0), (*b, 0.0), (a * t.cos(), a * t.sin())], w, h, 46.0);
            let (v, pb, pa) = (p[0], p[1], p[2]);
            let inside = ((v.0 + pa.0 + pb.0) / 3.0, (v.1 + pa.1 + pb.1) / 3.0);
            rgb(cr, (0.94, 0.96, 1.0));
            cr.move_to(v.0, v.1);
            cr.line_to(pb.0, pb.1);
            cr.line_to(pa.0, pa.1);
            cr.close_path();
            cr.fill().ok();
            rgb(cr, INK);
            cr.set_line_width(3.0);
            cr.move_to(pa.0, pa.1);
            cr.line_to(v.0, v.1);
            cr.line_to(pb.0, pb.1);
            cr.stroke().ok();
            // the side to find, dashed
            rgb(cr, ACCENT);
            cr.set_dash(&[9.0, 7.0], 0.0);
            cr.move_to(pb.0, pb.1);
            cr.line_to(pa.0, pa.1);
            cr.stroke().ok();
            cr.set_dash(&[], 0.0);
            // the angle between the two sides
            cr.set_line_width(2.5);
            cr.arc_negative(v.0, v.1, 30.0, 0.0, -t);
            cr.stroke().ok();
            label(cr, area, angle, (v.0 + 30.0 * (t / 2.0).cos(), v.1 - 30.0 * (t / 2.0).sin()), ((t / 2.0).cos(), -(t / 2.0).sin()), 6.0);
            label(cr, area, b_label, mid(v, pb), outward(v, pb, inside), 6.0);
            label(cr, area, a_label, mid(v, pa), outward(v, pa, inside), 8.0);
            label(cr, area, third, mid(pb, pa), outward(pb, pa, inside), 8.0);
        }
        Figure::Unit { deg, label: text } => {
            let r = (w.min(h) / 2.0 - 30.0).max(20.0);
            let (cx, cy) = (w / 2.0, h / 2.0);
            // axes
            rgb(cr, (0.75, 0.77, 0.85));
            cr.set_line_width(1.5);
            cr.move_to(cx - r - 18.0, cy);
            cr.line_to(cx + r + 18.0, cy);
            cr.move_to(cx, cy - r - 18.0);
            cr.line_to(cx, cy + r + 18.0);
            cr.stroke().ok();
            // the circle
            rgb(cr, INK);
            cr.set_line_width(2.5);
            cr.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
            cr.stroke().ok();
            label(cr, area, "1", (cx + r, cy), (0.6, 0.8), 3.0);
            label(cr, area, "−1", (cx - r, cy), (-0.6, 0.8), 3.0);
            // the angle and its point
            let t = deg.to_radians();
            let (px, py) = (cx + r * t.cos(), cy - r * t.sin());
            rgb(cr, SOFT);
            cr.set_dash(&[6.0, 5.0], 0.0);
            cr.set_line_width(2.0);
            cr.move_to(px, py);
            cr.line_to(px, cy);
            cr.stroke().ok();
            cr.set_dash(&[], 0.0);
            rgb(cr, INK);
            cr.set_line_width(2.5);
            cr.move_to(cx, cy);
            cr.line_to(px, py);
            cr.stroke().ok();
            rgb(cr, ACCENT);
            cr.arc_negative(cx, cy, 24.0, 0.0, -t);
            cr.stroke().ok();
            cr.arc(px, py, 6.0, 0.0, std::f64::consts::TAU);
            cr.fill().ok();
            label(cr, area, text, (cx + 24.0 * (t / 2.0).cos(), cy - 24.0 * (t / 2.0).sin()), ((t / 2.0).cos(), -(t / 2.0).sin()), 8.0);
            label(cr, area, "(cos, sin)", (px, py), norm((t.cos(), -t.sin())), 8.0);
        }
    }
}

/// A drawing of `fig`, `w` × `h` pixels, in the surrounding text's font.
pub fn widget(fig: &Figure, w: i32, h: i32) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::builder().content_width(w).content_height(h).halign(gtk::Align::Center).css_classes(["figure"]).build();
    let fig = fig.clone();
    area.set_draw_func(move |area, cr, w, h| draw(&fig, area, cr, w as f64, h as f64));
    area
}

fn s(t: &str) -> String {
    t.to_string()
}

/// The diagram beside a level's "How it works" text, if it has one.
pub fn for_lesson(level_id: &str) -> Option<Figure> {
    Some(match level_id {
        "pythagoras" => Figure::Right { adj: 4.0, opp: 3.0, angle: s(""), adj_label: s("a"), opp_label: s("b"), hyp_label: s("c") },
        "trigratios" => Figure::Right { adj: 4.0, opp: 3.0, angle: s("A"), adj_label: s("adjacent"), opp_label: s("opposite"), hyp_label: s("hypotenuse") },
        "specialangles" => Figure::Right { adj: 3f64.sqrt(), opp: 1.0, angle: s("30°"), adj_label: s("√3"), opp_label: s("1"), hyp_label: s("2") },
        "lawsincos" => Figure::Sas { a: 5.0, b: 7.0, deg: 55.0, a_label: s("a"), b_label: s("b"), angle: s("C"), third: s("c") },
        _ => return None,
    })
}
