//! The scratchpad beside a problem: a formula pad that works out one line at
//! a time (with the problem's values ready to use), and a scribble page on
//! graph paper. Both start fresh with each new question.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gtk::{cairo, glib, prelude::*};

use crate::{
    calc::{Line, Pad},
    label, math,
};

const KEYS: [&str; 10] = ["√", "^", "(", ")", "π", "sin", "cos", "tan", "!", "ans"];

pub struct Scratch {
    pub root: gtk::Box,
    pad: RefCell<Pad>,
    given: gtk::FlowBox,
    given_row: gtk::Box,
    history: gtk::Box,
    history_scroll: gtk::ScrolledWindow,
    entry: gtk::Entry,
    message: gtk::Label,
    paper: Scribble,
    on_use: RefCell<Option<Box<dyn Fn(&str)>>>,
}

fn small(text: &str) -> gtk::Button {
    gtk::Button::builder().label(text).focus_on_click(false).css_classes(["pad-key"]).build()
}

impl Scratch {
    pub fn new() -> Rc<Self> {
        // ── Formula pad
        let given = gtk::FlowBox::builder().selection_mode(gtk::SelectionMode::None).max_children_per_line(6).column_spacing(6).row_spacing(6).build();
        let given_row = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(4).build();
        let gl = label("Given (click to use):", "tip");
        gl.set_xalign(0.0);
        given_row.append(&gl);
        given_row.append(&given);
        let history = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(8).build();
        let start = label("Work it out here, one line at a time.\nTry: b^2 - 4ac,  sqrt(ans),  3/4 + 1/6,  sin 30\nName a value with d = …", "tip");
        start.set_xalign(0.0);
        start.set_wrap(true);
        history.append(&start);
        let history_scroll = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Automatic).vexpand(true).min_content_height(160).child(&history).build();
        let message = label("", "pad-message");
        message.set_wrap(true);
        message.set_xalign(0.0);
        message.set_visible(false);
        let keys = gtk::FlowBox::builder().selection_mode(gtk::SelectionMode::None).max_children_per_line(5).min_children_per_line(5).homogeneous(true).column_spacing(4).row_spacing(4).build();
        let entry = gtk::Entry::builder().placeholder_text("Type, then Enter").css_classes(["pad-entry"]).build();
        let clear = gtk::Button::builder().label("Clear").css_classes(["soft"]).tooltip_text("Start a clean pad (the given values stay)").build();
        let entry_row = gtk::Box::builder().spacing(6).build();
        entry.set_hexpand(true);
        entry_row.append(&entry);
        entry_row.append(&clear);
        let formula = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(8).build();
        formula.append(&given_row);
        formula.append(&history_scroll);
        formula.append(&message);
        formula.append(&entry_row);
        formula.append(&keys);

        // ── Scribble
        let paper = Scribble::new();
        let pen = gtk::ToggleButton::builder().label("✏️ Pen").active(true).css_classes(["soft"]).build();
        let eraser = gtk::ToggleButton::builder().label("🧽 Eraser").css_classes(["soft"]).group(&pen).build();
        let wipe = gtk::Button::builder().label("🗑️ Clear").css_classes(["soft"]).build();
        let tools = gtk::Box::builder().spacing(6).halign(gtk::Align::Center).build();
        tools.append(&pen);
        tools.append(&eraser);
        tools.append(&wipe);
        let scribble = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(8).build();
        scribble.append(&paper.area);
        scribble.append(&tools);

        // ── Switch between them
        let stack = gtk::Stack::builder().transition_type(gtk::StackTransitionType::Crossfade).vexpand(true).build();
        stack.add_named(&formula, Some("pad"));
        stack.add_named(&scribble, Some("paper"));
        let t_pad = gtk::ToggleButton::builder().label("🧮 Formula pad").active(true).css_classes(["soft"]).build();
        let t_paper = gtk::ToggleButton::builder().label("✏️ Scribble").css_classes(["soft"]).group(&t_pad).build();
        let tabs = gtk::Box::builder().spacing(6).homogeneous(true).build();
        tabs.append(&t_pad);
        tabs.append(&t_paper);
        let root = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).margin_top(14).margin_bottom(14).margin_start(14).margin_end(14).css_classes(["scratch"]).build();
        root.append(&tabs);
        root.append(&stack);

        let me = Rc::new(Scratch {
            root,
            pad: RefCell::new(Pad::default()),
            given,
            given_row,
            history,
            history_scroll,
            entry,
            message,
            paper,
            on_use: RefCell::default(),
        });

        {
            let stack = stack.clone();
            t_pad.connect_toggled(move |b| {
                if b.is_active() {
                    stack.set_visible_child_name("pad");
                }
            });
        }
        t_paper.connect_toggled(move |b| {
            if b.is_active() {
                stack.set_visible_child_name("paper");
            }
        });
        {
            let p = me.paper.clone();
            eraser.connect_toggled(move |b| p.erasing.set(b.is_active()));
        }
        {
            let p = me.paper.clone();
            wipe.connect_clicked(move |_| p.clear());
        }
        for k in KEYS {
            let b = small(k);
            let w = Rc::downgrade(&me);
            b.connect_clicked(move |_| {
                if let Some(me) = w.upgrade() {
                    me.insert(match k {
                        "√" | "sin" | "cos" | "tan" => format!("{k}("),
                        _ => k.to_string(),
                    });
                }
            });
            keys.append(&b);
        }
        let w = Rc::downgrade(&me);
        me.entry.connect_activate(move |e| {
            if let Some(me) = w.upgrade() {
                me.run(&e.text());
            }
        });
        let w = Rc::downgrade(&me);
        clear.connect_clicked(move |_| {
            if let Some(me) = w.upgrade() {
                me.clear_history();
            }
        });
        me
    }

    pub fn focus(&self) {
        self.entry.grab_focus();
    }

    /// Called with a result's text when "send to answer" is clicked.
    pub fn connect_use(&self, f: impl Fn(&str) + 'static) {
        *self.on_use.borrow_mut() = Some(Box::new(f));
    }

    fn insert(&self, s: String) {
        let pos = self.entry.position();
        let mut p = pos;
        self.entry.insert_text(&s, &mut p);
        self.entry.grab_focus_without_selecting();
        self.entry.set_position(p);
    }

    fn say(&self, text: &str) {
        self.message.set_text(text);
        self.message.set_visible(!text.is_empty());
    }

    fn clear_history(&self) {
        while let Some(c) = self.history.first_child() {
            self.history.remove(&c);
        }
        let given: Vec<_> = self.pad.borrow().vars.iter().filter(|(k, _)| *k != "ans").map(|(k, v)| (k.clone(), *v)).collect();
        let mut pad = Pad::default();
        pad.vars.extend(given);
        *self.pad.borrow_mut() = pad;
        self.say("");
    }

    /// A fresh pad and page for a new question, with its given values.
    pub fn reset(self: &Rc<Self>, given: &[(String, String)]) {
        *self.pad.borrow_mut() = Pad::default();
        while let Some(c) = self.given.first_child() {
            self.given.remove(&c);
        }
        for (name, value) in given {
            self.pad.borrow_mut().set(name, value);
            let chip = small(&format!("{name} = {}", value.replace('-', "−")));
            chip.add_css_class("given");
            let w = Rc::downgrade(self);
            let name = name.clone();
            chip.connect_clicked(move |_| {
                if let Some(me) = w.upgrade() {
                    me.insert(name.clone());
                }
            });
            self.given.append(&chip);
        }
        self.given_row.set_visible(!given.is_empty());
        while let Some(c) = self.history.first_child() {
            self.history.remove(&c);
        }
        let hint = label("Work it out here, one line at a time.\nTry: b^2 - 4ac,  sqrt(ans),  3/4 + 1/6,  sin 30\nName a value with d = …", "tip");
        hint.set_xalign(0.0);
        hint.set_wrap(true);
        self.history.append(&hint);
        self.entry.set_text("");
        self.say("");
        self.paper.clear();
    }

    fn run(self: &Rc<Self>, src: &str) {
        let result = self.pad.borrow_mut().run(src);
        match result {
            Ok(line) => {
                if self.history.first_child().is_some_and(|c| c.has_css_class("tip")) {
                    while let Some(c) = self.history.first_child() {
                        self.history.remove(&c);
                    }
                }
                self.history.append(&self.row(&line));
                self.entry.set_text("");
                self.say("");
                let adj = self.history_scroll.vadjustment();
                glib::idle_add_local_once(move || adj.set_value(adj.upper()));
            }
            Err(msg) => self.say(&format!("🤔 {msg}")),
        }
    }

    fn row(self: &Rc<Self>, line: &Line) -> gtk::Box {
        let row = gtk::Box::builder().spacing(6).css_classes(["pad-line"]).build();
        let work = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).hexpand(true).build();
        work.append(&math::rich(&line.shown, gtk::Align::Start));
        let res = gtk::Box::builder().spacing(4).build();
        res.append(&math::rich(&format!("= {}", line.result), gtk::Align::Start));
        if let Some(a) = &line.approx {
            res.append(&label(&format!("≈ {a}"), "tip"));
        }
        work.append(&res);
        row.append(&work);
        let send = gtk::Button::builder().label("↪ Use").valign(gtk::Align::Center).tooltip_text("Put this in my answer box").css_classes(["pad-key"]).build();
        let w = Rc::downgrade(self);
        let plain = line.plain.clone();
        send.connect_clicked(move |_| {
            if let Some(me) = w.upgrade()
                && let Some(f) = me.on_use.borrow().as_ref()
            {
                f(&plain);
            }
        });
        row.append(&send);
        row
    }
}

/// Graph paper to draw on with the mouse or a pen.
#[derive(Clone)]
struct Scribble {
    area: gtk::DrawingArea,
    /// Strokes as (eraser?, points).
    strokes: Rc<RefCell<Vec<(bool, Vec<(f64, f64)>)>>>,
    erasing: Rc<Cell<bool>>,
}

impl Scribble {
    fn new() -> Self {
        let area = gtk::DrawingArea::builder().vexpand(true).content_height(380).css_classes(["paper"]).build();
        let strokes: Rc<RefCell<Vec<(bool, Vec<(f64, f64)>)>>> = Rc::default();
        let erasing = Rc::new(Cell::new(false));
        let s = strokes.clone();
        area.set_draw_func(move |_, cr, w, h| draw_paper(cr, w as f64, h as f64, &s.borrow()));
        let drag = gtk::GestureDrag::new();
        {
            let (s, e, a) = (strokes.clone(), erasing.clone(), area.clone());
            drag.connect_drag_begin(move |_, x, y| {
                s.borrow_mut().push((e.get(), vec![(x, y)]));
                a.queue_draw();
            });
        }
        {
            let (s, a) = (strokes.clone(), area.clone());
            drag.connect_drag_update(move |g, dx, dy| {
                if let Some((x, y)) = g.start_point()
                    && let Some(last) = s.borrow_mut().last_mut()
                {
                    last.1.push((x + dx, y + dy));
                }
                a.queue_draw();
            });
        }
        area.add_controller(drag);
        Scribble { area, strokes, erasing }
    }

    fn clear(&self) {
        self.strokes.borrow_mut().clear();
        self.area.queue_draw();
    }
}

fn draw_paper(cr: &cairo::Context, w: f64, h: f64, strokes: &[(bool, Vec<(f64, f64)>)]) {
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.paint().ok();
    // graph paper
    cr.set_source_rgb(0.86, 0.9, 0.97);
    cr.set_line_width(1.0);
    let mut x = 0.5;
    while x < w {
        cr.move_to(x, 0.0);
        cr.line_to(x, h);
        x += 20.0;
    }
    let mut y = 0.5;
    while y < h {
        cr.move_to(0.0, y);
        cr.line_to(w, y);
        y += 20.0;
    }
    cr.stroke().ok();
    // the ink goes on its own layer, so the eraser removes ink but not the grid
    cr.push_group();
    cr.set_line_cap(cairo::LineCap::Round);
    cr.set_line_join(cairo::LineJoin::Round);
    for (erase, pts) in strokes {
        if *erase {
            cr.set_operator(cairo::Operator::Clear);
            cr.set_line_width(22.0);
        } else {
            cr.set_operator(cairo::Operator::Over);
            cr.set_source_rgb(0.18, 0.18, 0.35);
            cr.set_line_width(3.0);
        }
        if let Some(&(x0, y0)) = pts.first() {
            cr.move_to(x0, y0);
            if pts.len() == 1 {
                cr.line_to(x0 + 0.1, y0);
            }
            for &(x, y) in &pts[1..] {
                cr.line_to(x, y);
            }
            cr.stroke().ok();
        }
    }
    cr.set_operator(cairo::Operator::Over);
    cr.pop_group_to_source().ok();
    cr.paint().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ink lands on the paper; the eraser removes ink but leaves the grid.
    #[test]
    fn scribble_and_erase() {
        let surf = cairo::ImageSurface::create(cairo::Format::ARgb32, 200, 100).unwrap();
        {
            let cr = cairo::Context::new(&surf).unwrap();
            let strokes = vec![(false, vec![(10.0, 50.0), (190.0, 50.0)]), (true, vec![(100.0, 50.0), (101.0, 50.0)])];
            draw_paper(&cr, 200.0, 100.0, &strokes);
        }
        let stride = surf.stride() as usize;
        let data = surf.take_data().unwrap();
        // BGRA: blue channel of a pixel
        let blue = |x: usize, y: usize| data[y * stride + x * 4];
        let red = |x: usize, y: usize| data[y * stride + x * 4 + 2];
        assert!(red(40, 50) < 100, "pen ink is dark");
        assert!(red(105, 50) > 200 && blue(105, 50) > 200, "erased back to paper");
        assert!(red(40, 30) > 200, "paper elsewhere");
    }
}
