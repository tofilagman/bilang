//! The "two-layer" answer pad: the problem written in columns like on paper,
//! with a box for every answer digit. Kids fill it from the ones place
//! (right to left), or left to right on top of a long-division bracket, and
//! can jot carries in the small boxes. Checking colours each digit, so a kid
//! sees which column went wrong.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gtk::{gdk, glib, prelude::*};

use crate::{
    answer::Answer,
    levels::{ColOp, Column},
    teach::Target,
};

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Answer,
    Carry,
    Remainder,
}

struct Slot {
    label: gtk::Label,
    value: Cell<Option<char>>,
    kind: Kind,
    /// Place from the right (0 = ones); for long division, the dividend digit from the left.
    place: usize,
}

pub struct ColumnPad {
    pub root: gtk::Box,
    slots: Vec<Slot>,
    /// Answer slots left to right, then the remainder box.
    visual: Vec<usize>,
    /// The order digits are typed in.
    order: Vec<usize>,
    cursor: Cell<usize>,
    /// Where typing goes back to after jotting a carry.
    resume: Cell<usize>,
    locked: Cell<bool>,
    on_enter: RefCell<Option<Box<dyn Fn()>>>,
    on_key: RefCell<Option<Box<dyn Fn()>>>,
    /// Tutorial mode: the one box to fill and the digit that belongs there.
    guide: Cell<Option<(usize, char)>>,
    on_guided: RefCell<Option<Box<dyn Fn(bool)>>>,
    /// Everything in a column, so a tutorial can light it up: (place, widget).
    columns: Vec<(usize, gtk::Widget)>,
}

fn digits_of(n: i64) -> Vec<char> {
    n.to_string().chars().collect()
}

fn fixed(text: &str, class: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    l.add_css_class(class);
    l
}

impl ColumnPad {
    pub fn new(col: Column) -> Rc<Self> {
        let grid = gtk::Grid::builder().row_spacing(6).column_spacing(6).halign(gtk::Align::Center).build();
        grid.add_css_class("colgrid");
        let mut slots: Vec<Slot> = Vec::new();
        let mut columns: Vec<(usize, gtk::Widget)> = Vec::new();
        let mut add_slot = |grid: &gtk::Grid, kind: Kind, place: usize, c: i32, r: i32| {
            let label = gtk::Label::new(Some(" "));
            label.add_css_class(if kind == Kind::Carry { "carry" } else { "slot" });
            label.set_valign(gtk::Align::Center);
            label.set_halign(gtk::Align::Center);
            grid.attach(&label, c, r, 1, 1);
            slots.push(Slot { label, value: Cell::new(None), kind, place });
            slots.len() - 1
        };
        let mut visual = Vec::new();
        let order: Vec<usize>;

        if col.op == ColOp::Div {
            // divisor ) dividend, with the quotient boxes on top of the dividend.
            let ds = digits_of(col.top);
            let n = ds.len() as i32;
            for k in 0..n {
                visual.push(add_slot(&grid, Kind::Answer, k as usize, 2 + k, 0));
            }
            grid.attach(&fixed("r", "digit"), 2 + n, 0, 1, 1);
            let rem = add_slot(&grid, Kind::Remainder, usize::MAX, 3 + n, 0);
            visual.push(rem);
            let bar = gtk::Separator::new(gtk::Orientation::Horizontal);
            bar.add_css_class("rule");
            grid.attach(&bar, 1, 1, n + 1, 1);
            grid.attach(&fixed(&col.bottom.to_string(), "digit"), 0, 2, 1, 1);
            grid.attach(&fixed(")", "bracket"), 1, 2, 1, 1);
            for (k, d) in ds.iter().enumerate() {
                let l = fixed(&d.to_string(), "digit");
                grid.attach(&l, 2 + k as i32, 2, 1, 1);
                columns.push((k, l.upcast()));
            }
            order = visual.clone();
        } else {
            let (top, bottom) = (digits_of(col.top), digits_of(col.bottom));
            let w = match col.op {
                ColOp::Sub => top.len(),
                _ => top.len().max(bottom.len()) + 1,
            } as i32;
            // Column c (1..=w) holds place w − c, so the ones sit in column w.
            let digit_at = |ds: &[char], place: i32| -> Option<char> {
                let i = ds.len() as i32 - 1 - place;
                (i >= 0).then(|| ds[i as usize])
            };
            for c in 1..=w {
                let place = w - c;
                // Carries for + and ×; for −, the boxes hold a digit's new value after it lends.
                let carry_box = match col.op {
                    ColOp::Sub => place >= 1 && place < top.len() as i32,
                    _ => place >= 1 && place <= top.len() as i32,
                };
                if carry_box {
                    add_slot(&grid, Kind::Carry, place as usize, c, 0);
                }
                if let Some(d) = digit_at(&top, place) {
                    let l = fixed(&d.to_string(), "digit");
                    grid.attach(&l, c, 1, 1, 1);
                    columns.push((place as usize, l.upcast()));
                }
                if let Some(d) = digit_at(&bottom, place) {
                    let l = fixed(&d.to_string(), "digit");
                    grid.attach(&l, c, 2, 1, 1);
                    columns.push((place as usize, l.upcast()));
                }
                visual.push(add_slot(&grid, Kind::Answer, place as usize, c, 4));
            }
            let sign = match col.op {
                ColOp::Add => "+",
                ColOp::Sub => "−",
                _ => "×",
            };
            grid.attach(&fixed(sign, "digit"), 0, 2, 1, 1);
            let bar = gtk::Separator::new(gtk::Orientation::Horizontal);
            bar.add_css_class("rule");
            grid.attach(&bar, 0, 3, w + 1, 1);
            order = visual.iter().rev().copied().collect();
        }

        let root = gtk::Box::builder().halign(gtk::Align::Center).focusable(true).build();
        root.add_css_class("colpad");
        root.append(&grid);
        let pad = Rc::new(ColumnPad {
            root,
            slots,
            visual,
            cursor: Cell::new(order[0]),
            resume: Cell::new(0),
            order,
            locked: Cell::new(false),
            on_enter: RefCell::default(),
            on_key: RefCell::default(),
            guide: Cell::new(None),
            on_guided: RefCell::default(),
            columns,
        });

        for (i, s) in pad.slots.iter().enumerate() {
            let click = gtk::GestureClick::new();
            let p = Rc::downgrade(&pad);
            click.connect_pressed(move |_, _, _, _| {
                if let Some(p) = p.upgrade() {
                    p.select(i);
                }
            });
            s.label.add_controller(click);
        }
        let keys = gtk::EventControllerKey::new();
        // Capture, so arrow keys reach the pad before GTK uses them to move focus.
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let p = Rc::downgrade(&pad);
        keys.connect_key_pressed(move |_, key, _, _| {
            let Some(p) = p.upgrade() else { return glib::Propagation::Proceed };
            p.key(key)
        });
        pad.root.add_controller(keys);
        pad.refresh();
        pad
    }

    pub fn connect_enter(&self, f: impl Fn() + 'static) {
        *self.on_enter.borrow_mut() = Some(Box::new(f));
    }

    pub fn connect_key(&self, f: impl Fn() + 'static) {
        *self.on_key.borrow_mut() = Some(Box::new(f));
    }

    /// Called with whether the digit typed into the guided box was right.
    pub fn connect_guided(&self, f: impl Fn(bool) + 'static) {
        *self.on_guided.borrow_mut() = Some(Box::new(f));
    }

    fn slot_for(&self, t: Target) -> Option<usize> {
        self.slots.iter().position(|s| match t {
            Target::Answer(p) | Target::Quotient(p) => s.kind == Kind::Answer && s.place == p,
            Target::Carry(p) => s.kind == Kind::Carry && s.place == p,
            Target::Remainder => s.kind == Kind::Remainder,
        })
    }

    /// Tutorial mode: only this box takes a digit, and it glows. None ends it.
    pub fn guide(&self, t: Option<(Target, char)>) {
        let g = t.and_then(|(t, c)| Some((self.slot_for(t)?, c)));
        if let Some((slot, _)) = g {
            self.cursor.set(slot);
            self.slots[slot].label.remove_css_class("bad");
        }
        self.guide.set(g);
        self.refresh();
    }

    /// Light up one column (a place, or a dividend digit) while Kiko talks about it.
    pub fn point(&self, place: Option<usize>) {
        for (p, w) in &self.columns {
            if Some(*p) == place {
                w.add_css_class("pointed");
            } else {
                w.remove_css_class("pointed");
            }
        }
        for s in &self.slots {
            if s.kind != Kind::Remainder && Some(s.place) == place {
                s.label.add_css_class("pointed");
            } else {
                s.label.remove_css_class("pointed");
            }
        }
    }

    fn key(&self, key: gdk::Key) -> glib::Propagation {
        match key {
            gdk::Key::Return | gdk::Key::KP_Enter => {
                if let Some(f) = self.on_enter.borrow().as_ref() {
                    f();
                }
            }
            gdk::Key::BackSpace | gdk::Key::Delete => self.press("⌫"),
            gdk::Key::Left => self.step(-1),
            gdk::Key::Right => self.step(1),
            _ => match key.to_unicode() {
                Some(c) if c.is_ascii_digit() => self.press(&c.to_string()),
                _ => return glib::Propagation::Proceed,
            },
        }
        glib::Propagation::Stop
    }

    pub fn focus(&self) {
        self.root.grab_focus();
    }

    pub fn set_locked(&self, locked: bool) {
        self.locked.set(locked);
        self.refresh();
    }

    fn select(&self, i: usize) {
        if self.locked.get() || self.guide.get().is_some() {
            return;
        }
        self.cursor.set(i);
        if let Some(pos) = self.order.iter().position(|&o| o == i) {
            self.resume.set(pos);
        }
        self.refresh();
        self.focus();
    }

    /// Arrow keys move along the answer boxes as they appear on screen.
    fn step(&self, dir: i32) {
        let cur = self.cursor.get();
        let at = self.visual.iter().position(|&v| v == cur).unwrap_or(0) as i32;
        let next = (at + dir).clamp(0, self.visual.len() as i32 - 1) as usize;
        self.select(self.visual[next]);
    }

    /// A digit or ⌫ from the keyboard or the on-screen keypad.
    pub fn press(&self, k: &str) {
        if let Some((slot, want)) = self.guide.get() {
            let Some(c) = k.chars().next().filter(char::is_ascii_digit) else { return };
            if let Some(f) = self.on_key.borrow().as_ref() {
                f();
            }
            let s = &self.slots[slot];
            s.value.set(Some(c));
            let ok = c == want;
            s.label.add_css_class(if ok { "good" } else { "bad" });
            s.label.remove_css_class(if ok { "bad" } else { "good" });
            if ok {
                self.guide.set(None);
            }
            self.refresh();
            if let Some(f) = self.on_guided.borrow().as_ref() {
                f(ok);
            }
            return;
        }
        if self.locked.get() {
            return;
        }
        if let Some(f) = self.on_key.borrow().as_ref() {
            f();
        }
        let cur = self.cursor.get();
        let slot = &self.slots[cur];
        if k == "⌫" {
            if slot.value.get().is_some() {
                slot.value.set(None);
            } else if let Some(pos) = self.order.iter().position(|&o| o == cur)
                && pos > 0
            {
                let prev = self.order[pos - 1];
                self.slots[prev].value.set(None);
                self.cursor.set(prev);
                self.resume.set(pos - 1);
            }
        } else if let Some(c) = k.chars().next().filter(char::is_ascii_digit) {
            slot.value.set(Some(c));
            slot.label.remove_css_class("good");
            slot.label.remove_css_class("bad");
            if slot.kind == Kind::Carry {
                self.cursor.set(self.order[self.resume.get()]);
            } else if let Some(pos) = self.order.iter().position(|&o| o == cur)
                && pos + 1 < self.order.len()
            {
                self.cursor.set(self.order[pos + 1]);
                self.resume.set(pos + 1);
            }
        }
        self.refresh();
    }

    fn refresh(&self) {
        let cur = self.cursor.get();
        for (i, s) in self.slots.iter().enumerate() {
            s.label.set_text(&s.value.get().map_or(" ".to_string(), |c| c.to_string()));
            let guided = self.guide.get().is_some_and(|(g, _)| g == i);
            if guided || (i == cur && !self.locked.get() && self.guide.get().is_none()) {
                s.label.add_css_class("active");
            } else {
                s.label.remove_css_class("active");
            }
        }
    }

    /// What the kid wrote, as text for Answer::check ("1508", "110 r 1"),
    /// or a friendly reason it can't be read yet.
    pub fn text(&self) -> Result<String, String> {
        let mut digits = String::new();
        let mut gap = false;
        let mut rem = None;
        for &i in &self.visual {
            let s = &self.slots[i];
            match (s.kind, s.value.get()) {
                (Kind::Remainder, v) => rem = v,
                (_, Some(_)) if gap => return Err("There's an empty box in the middle of your answer. Fill it in (it might be 0)!".into()),
                (_, Some(c)) => digits.push(c),
                (_, None) => gap = !digits.is_empty(),
            }
        }
        if digits.is_empty() {
            return Err("Type your answer in the boxes!".into());
        }
        Ok(match rem {
            Some(r) => format!("{digits} r {r}"),
            None => digits,
        })
    }

    /// Colour each answer box green or red against the right answer.
    pub fn mark(&self, answer: &Answer) {
        let (main, rem) = match *answer {
            Answer::Int(n) => (n, None),
            Answer::Remainder(q, r) => (q, Some(r)),
            _ => return,
        };
        let want: Vec<char> = main.to_string().chars().collect();
        let boxes: Vec<usize> = self.visual.iter().copied().filter(|&i| self.slots[i].kind == Kind::Answer).collect();
        let pad = boxes.len().saturating_sub(want.len());
        for (k, &i) in boxes.iter().enumerate() {
            let expected = k.checked_sub(pad).map(|j| want[j]);
            self.paint(i, expected, false);
        }
        if let (Some(r), Some(&i)) = (rem, self.visual.last()) {
            self.paint(i, Some(char::from_digit(r as u32, 10).unwrap_or('?')), r == 0);
        }
    }

    /// `blank_is_zero`: an empty box counts as 0 (a remainder of nothing).
    fn paint(&self, i: usize, expected: Option<char>, blank_is_zero: bool) {
        let s = &self.slots[i];
        s.label.remove_css_class("good");
        s.label.remove_css_class("bad");
        let got = s.value.get();
        let blank_ok = |g: Option<char>| g.is_none() || g == Some('0');
        match expected {
            None if blank_ok(got) => {}
            None => s.label.add_css_class("bad"),
            Some(e) if got == Some(e) => s.label.add_css_class("good"),
            Some('0') if blank_is_zero && got.is_none() => {}
            Some(_) => s.label.add_css_class("bad"),
        }
    }
}
