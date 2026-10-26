//! The "Show me how" screen: Kiko teaches one example step by step, asking
//! small questions and guiding each digit into the right box, like a parent
//! sitting next to the kid. Two misses on a step and Kiko just says it, so
//! nobody gets stuck.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use adw::prelude::*;
use gtk::glib;
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::{
    App,
    answer::Verdict,
    column::ColumnPad,
    label,
    figure,
    levels::Level,
    math,
    play::key,
    sound::Cue,
    teach::{self, Step},
};

const NICE: [&str; 6] = ["Yes! ", "Great! ", "Right! ", "Perfect! ", "Good job! ", "That's it! "];

pub struct Learn {
    app: Rc<App>,
    level: &'static Level,
    pub root: gtk::ScrolledWindow,
    counter: gtk::Label,
    mascot: gtk::Label,
    bubble: gtk::Box,
    visual: gtk::Label,
    figure: gtk::Box,
    board: gtk::Box,
    pad_holder: gtk::Box,
    ask_row: gtk::Box,
    entry: gtk::Entry,
    next: gtk::Button,
    tip: gtk::Label,
    keys: gtk::Box,
    /// Keys beyond digits that the current question needs ( / . , < > r ).
    extra_keys: gtk::Box,
    rng: RefCell<StdRng>,
    steps: RefCell<Vec<Step>>,
    i: Cell<usize>,
    tries: Cell<u32>,
    /// The kid just got a step right: start the next line with a cheer.
    cheer: Cell<bool>,
    pad: RefCell<Option<Rc<ColumnPad>>>,
    on_done: RefCell<Option<Box<dyn Fn()>>>,
    on_skip: RefCell<Option<Box<dyn Fn()>>>,
}

impl Learn {
    pub fn new(app: &Rc<App>, level: &'static Level) -> Rc<Self> {
        let title = label("📖 Let's learn it together!", "side-title");
        title.set_hexpand(true);
        title.set_xalign(0.0);
        let counter = label("", "count");
        let skip = gtk::Button::builder().label("Skip ⏭").css_classes(["soft"]).tooltip_text("I already know how").build();
        let top = gtk::Box::builder().spacing(12).build();
        top.append(&title);
        top.append(&counter);
        top.append(&skip);

        let mascot = label("🦉", "mascot");
        let bubble = gtk::Box::builder().hexpand(true).valign(gtk::Align::Center).css_classes(["bubble", "teach"]).build();
        let kiko = gtk::Box::builder().spacing(12).build();
        kiko.append(&mascot);
        kiko.append(&bubble);

        let visual = label("", "visual");
        visual.set_wrap(true);
        let board = gtk::Box::builder().halign(gtk::Align::Center).css_classes(["prompt"]).build();
        let pad_holder = gtk::Box::builder().halign(gtk::Align::Center).build();
        let card = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(12).css_classes(["card"]).build();
        let figure_box = gtk::Box::builder().halign(gtk::Align::Center).build();
        card.append(&visual);
        card.append(&figure_box);
        card.append(&board);
        card.append(&pad_holder);

        let entry = gtk::Entry::builder().width_chars(8).xalign(0.5).placeholder_text("?").css_classes(["answer"]).build();
        let check = gtk::Button::builder().label("Check ✔").css_classes(["go"]).build();
        let ask_row = gtk::Box::builder().spacing(12).halign(gtk::Align::Center).build();
        ask_row.append(&entry);
        ask_row.append(&check);
        let next = gtk::Button::builder().label("Next ➜").css_classes(["go", "next"]).halign(gtk::Align::Center).build();
        let tip = label("", "tip");

        let digits = gtk::Grid::builder().row_spacing(8).column_spacing(8).build();
        let mut buttons = Vec::new();
        for (i, d) in ["7", "8", "9", "4", "5", "6", "1", "2", "3", "−", "0", "⌫"].iter().enumerate() {
            let b = key(d, matches!(*d, "−" | "⌫"));
            digits.attach(&b, (i % 3) as i32, (i / 3) as i32, 1, 1);
            buttons.push(b);
        }
        let extra_keys = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(8).build();
        let keys = gtk::Box::builder().spacing(8).halign(gtk::Align::Center).build();
        keys.append(&digits);
        keys.append(&extra_keys);

        let body = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(14).margin_top(14).margin_bottom(24).margin_start(28).margin_end(28).build();
        for w in [top.upcast_ref::<gtk::Widget>(), kiko.upcast_ref(), card.upcast_ref(), ask_row.upcast_ref(), next.upcast_ref(), tip.upcast_ref(), keys.upcast_ref()] {
            body.append(w);
        }
        let clamp = adw::Clamp::builder().maximum_size(820).child(&body).build();
        let root = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).child(&clamp).build();

        let learn = Rc::new(Learn {
            app: app.clone(),
            level,
            root,
            counter,
            mascot,
            bubble,
            visual,
            figure: figure_box,
            board,
            pad_holder,
            ask_row,
            entry,
            next,
            tip,
            keys,
            extra_keys,
            rng: RefCell::new(StdRng::from_rng(&mut rand::rng())),
            steps: RefCell::default(),
            i: Cell::new(0),
            tries: Cell::new(0),
            cheer: Cell::new(false),
            pad: RefCell::default(),
            on_done: RefCell::default(),
            on_skip: RefCell::default(),
        });
        for b in buttons {
            let l = Rc::downgrade(&learn);
            b.connect_clicked(move |b| {
                if let Some(l) = l.upgrade() {
                    l.press(&b.label().unwrap_or_default());
                }
            });
        }
        let l = Rc::downgrade(&learn);
        learn.entry.connect_activate(move |_| {
            if let Some(l) = l.upgrade() {
                l.submit();
            }
        });
        let l = Rc::downgrade(&learn);
        check.connect_clicked(move |_| {
            if let Some(l) = l.upgrade() {
                l.submit();
            }
        });
        let l = Rc::downgrade(&learn);
        learn.next.connect_clicked(move |_| {
            if let Some(l) = l.upgrade() {
                l.app.cue(Cue::Tap);
                l.advance();
            }
        });
        // Enter means "Next" on talking steps, wherever the focus is.
        let enter = gtk::EventControllerKey::new();
        enter.set_propagation_phase(gtk::PropagationPhase::Capture);
        let l = Rc::downgrade(&learn);
        enter.connect_key_pressed(move |_, key, _, _| {
            let Some(l) = l.upgrade() else { return glib::Propagation::Proceed };
            if matches!(key, gtk::gdk::Key::Return | gtk::gdk::Key::KP_Enter) && l.next.is_visible() {
                l.app.cue(Cue::Tap);
                l.advance();
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        learn.root.add_controller(enter);
        let l = Rc::downgrade(&learn);
        skip.connect_clicked(move |_| {
            if let Some(l) = l.upgrade()
                && let Some(f) = l.on_skip.borrow().as_ref()
            {
                f();
            }
        });
        learn
    }

    pub fn connect_done(&self, f: impl Fn() + 'static) {
        *self.on_done.borrow_mut() = Some(Box::new(f));
    }

    pub fn connect_skip(&self, f: impl Fn() + 'static) {
        *self.on_skip.borrow_mut() = Some(Box::new(f));
    }

    /// A fresh example, from the first step.
    pub fn start(self: &Rc<Self>) {
        let p = (self.level.make)(&mut self.rng.borrow_mut());
        *self.steps.borrow_mut() = teach::lesson(self.level.id, &p);
        self.visual.set_text(p.visual.as_deref().unwrap_or(""));
        self.visual.set_visible(p.visual.is_some());
        while let Some(c) = self.figure.first_child() {
            self.figure.remove(&c);
        }
        if let Some(f) = &p.figure {
            self.figure.append(&figure::widget(f, 340, 185));
        }
        self.figure.set_visible(p.figure.is_some());
        math::set(&self.board, &p.prompt, gtk::Align::Center);
        if p.prompt.chars().count() > 30 || p.column.is_some() {
            self.board.add_css_class("long");
        } else {
            self.board.remove_css_class("long");
        }
        while let Some(c) = self.pad_holder.first_child() {
            self.pad_holder.remove(&c);
        }
        *self.pad.borrow_mut() = None;
        if let Some(col) = p.column {
            let pad = ColumnPad::new(col);
            // Only the glowing box takes digits during a lesson.
            pad.set_locked(true);
            let me = Rc::downgrade(self);
            pad.connect_guided(move |ok| {
                if let Some(me) = me.upgrade() {
                    me.guided(ok);
                }
            });
            let app = self.app.clone();
            pad.connect_key(move || app.cue(Cue::Tap));
            self.pad_holder.append(&pad.root);
            *self.pad.borrow_mut() = Some(pad);
        }
        self.pad_holder.set_visible(p.column.is_some());
        self.i.set(0);
        self.cheer.set(false);
        self.show();
    }

    pub fn focus(&self) {
        let steps = self.steps.borrow();
        let Some(st) = steps.get(self.i.get()) else { return };
        if st.ask.is_some() {
            self.entry.grab_focus();
        } else if st.write.is_some() {
            if let Some(pad) = self.pad.borrow().as_ref() {
                pad.focus();
            }
        } else {
            self.next.grab_focus();
        }
    }

    fn show(self: &Rc<Self>) {
        let i = self.i.get();
        let steps = self.steps.borrow();
        let st = &steps[i];
        let last = i + 1 == steps.len();
        self.tries.set(0);
        self.counter.set_text(&format!("Step {} of {}", i + 1, steps.len()));
        let cheer = if self.cheer.replace(false) { NICE[self.rng.borrow_mut().random_range(0..NICE.len())] } else { "" };
        math::set(&self.bubble, &format!("{cheer}{}", st.say), gtk::Align::Start);
        self.mascot.set_text(if last { "🥳" } else { "🦉" });
        if let Some(b) = &st.board {
            math::set(&self.board, b, gtk::Align::Center);
        }
        if let Some(pad) = self.pad.borrow().as_ref() {
            pad.point(st.point);
            pad.guide(st.write);
        }
        self.ask_row.set_visible(st.ask.is_some());
        self.next.set_visible(st.ask.is_none() && st.write.is_none());
        self.next.set_label(if last { "Let's practice! ➜" } else { "Next ➜" });
        self.keys.set_visible(st.ask.is_some() || st.write.is_some());
        while let Some(c) = self.extra_keys.first_child() {
            self.extra_keys.remove(&c);
        }
        for k in st.ask.as_ref().map(|a| a.keys()).unwrap_or(&[]).iter().filter(|k| **k != "−") {
            let b = key(k, true);
            let me = Rc::downgrade(self);
            b.connect_clicked(move |b| {
                if let Some(me) = me.upgrade() {
                    me.press(&b.label().unwrap_or_default());
                }
            });
            self.extra_keys.append(&b);
        }
        self.tip.set_text(match (&st.ask, st.write) {
            (Some(_), _) => "Type your answer, then press Check.",
            (_, Some(_)) => "Type the digit in the glowing box ✨",
            _ => "",
        });
        if std::env::var_os("BILANG_DEBUG").is_some() {
            match (&st.ask, st.write) {
                (Some(a), _) => eprintln!("learn: ask {}", a.show().replace('−', "-")),
                (_, Some((_, d))) => eprintln!("learn: write {d}"),
                _ => eprintln!("learn: next"),
            }
        }
        self.entry.set_text("");
        drop(steps);
        // After layout, or the grab is lost while the step's widgets appear.
        let me = Rc::downgrade(self);
        glib::idle_add_local_once(move || {
            if let Some(me) = me.upgrade() {
                me.focus();
            }
        });
    }

    fn press(self: &Rc<Self>, k: &str) {
        let write = self.steps.borrow().get(self.i.get()).is_some_and(|s| s.write.is_some());
        if write {
            if let Some(pad) = self.pad.borrow().as_ref() {
                pad.press(k);
                pad.focus();
            }
            return;
        }
        self.app.cue(Cue::Tap);
        let mut t = self.entry.text().to_string();
        match k {
            "⌫" => {
                t.pop();
            }
            "r" => t.push_str(" r "),
            "," => t.push_str(", "),
            _ => t.push_str(k),
        }
        self.entry.set_text(&t);
        self.entry.set_position(-1);
        self.entry.grab_focus_without_selecting();
    }

    fn submit(self: &Rc<Self>) {
        let Some(answer) = self.steps.borrow().get(self.i.get()).and_then(|s| s.ask.clone()) else { return };
        match answer.check(&self.entry.text()) {
            Verdict::Right => {
                self.app.cue(Cue::Right);
                self.cheer.set(true);
                self.advance();
            }
            Verdict::Almost(msg) => {
                self.app.cue(Cue::Almost);
                self.say("🤩", &msg);
            }
            Verdict::Unreadable(msg) => self.say("🤔", &msg),
            Verdict::Wrong => {
                self.app.cue(Cue::Wrong);
                let tries = self.tries.get() + 1;
                self.tries.set(tries);
                if tries >= 2 {
                    // Like a parent would: say it, then move on together.
                    self.say("🦉", &format!("That's okay! It's {}. I put it in for you, press Check and let's keep going.", answer.show()));
                    self.entry.set_text(&answer.show().replace('−', "-"));
                } else {
                    self.say("🤔", "Hmm, not quite. Take your time and try again! 💪");
                    self.entry.select_region(0, -1);
                }
                self.entry.grab_focus();
            }
        }
    }

    fn guided(self: &Rc<Self>, ok: bool) {
        if ok {
            self.app.cue(Cue::Right);
            self.cheer.set(true);
            // A moment to see the green box before moving on.
            let me = self.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(450), move || me.advance());
            return;
        }
        self.app.cue(Cue::Wrong);
        let tries = self.tries.get() + 1;
        self.tries.set(tries);
        let want = self.steps.borrow().get(self.i.get()).and_then(|s| s.write).map(|(_, d)| d).unwrap_or('?');
        if tries >= 2 {
            self.say("🦉", &format!("Almost! The digit that goes there is {want}. Type {want} in the glowing box."));
        } else {
            self.say("🤔", "Not that one. Look at the number we just found, and try again!");
        }
    }

    fn say(&self, face: &str, text: &str) {
        self.mascot.set_text(face);
        math::set(&self.bubble, text, gtk::Align::Start);
    }

    fn advance(self: &Rc<Self>) {
        let i = self.i.get() + 1;
        if i >= self.steps.borrow().len() {
            if let Some(f) = self.on_done.borrow().as_ref() {
                f();
            }
            return;
        }
        self.i.set(i);
        self.show();
    }
}
