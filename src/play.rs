//! One level: pick how many questions (10 to 100), then Kiko's reactions,
//! the keypad or the stacked column pad, and the help panel on the side with
//! the lesson and step-by-step hints.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Instant,
};

use adw::prelude::*;
use gtk::glib;
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::{
    App, clear,
    answer::Verdict,
    column::ColumnPad,
    confetti::Confetti,
    history::{self, Missed, Run},
    label,
    figure,
    learn::Learn,
    math,
    scratch::Scratch,
    levels::{self, Level, Problem},
    sound::Cue,
    star_text,
};

/// How many questions a kid can choose: (count, name).
const COUNTS: [(usize, &str); 4] = [(10, "Quick"), (20, "Practice"), (50, "Challenge 🥈"), (100, "Marathon 🏆")];

const PRAISE: [&str; 8] = ["Great job! ⭐", "You got it! 🎉", "Awesome! 🙌", "Math wizard! 🪄", "Galing! 👏", "Correct! Keep going! 🚀", "Yes! That's it! 💯", "Super! 😄"];
const OOPS: [&str; 5] = [
    "Not quite, but you're close. Try again! 💪",
    "Oops! Mistakes help our brains grow. Peek at the hint 💡",
    "Hmm, let's think again. The hint on the side can help!",
    "Almost! Read the hint and give it another go.",
    "That's okay! Let's figure it out together 🦉",
];

struct Play {
    app: Rc<App>,
    flat: usize,
    level: &'static Level,
    rng: RefCell<StdRng>,
    problem: RefCell<Problem>,
    q: Cell<usize>,
    /// Questions in this run, chosen on the setup screen.
    total: Cell<usize>,
    chosen: Rc<Cell<usize>>,
    /// Set while the current problem is worked in columns.
    colpad: RefCell<Option<Rc<ColumnPad>>>,
    /// Questions right on the first try with at most one hint.
    clean: Cell<usize>,
    hints_shown: Cell<usize>,
    missed: Cell<bool>,
    streak: Cell<u32>,
    best_streak: Cell<u32>,
    /// Answered right; Enter or Next moves on.
    solved: Cell<bool>,
    /// For the score sheet: when this run (or lesson) started, unset once saved.
    run_at: Cell<Option<(i64, Instant)>>,
    run_hints: Cell<u32>,
    run_wrong: Cell<u32>,
    /// What the kid typed on the current question before getting it.
    tries: RefCell<Vec<String>>,
    needed_help: RefCell<Vec<Missed>>,

    stack: gtk::Stack,
    progress: gtk::ProgressBar,
    count: gtk::Label,
    streak_label: gtk::Label,
    mascot: gtk::Label,
    bubble: gtk::Label,
    card: gtk::Box,
    visual: gtk::Label,
    /// Triangle or unit-circle diagram, when the problem has one.
    figure: gtk::Box,
    /// The question, drawn with real fractions and roots (see math.rs).
    prompt: gtk::Box,
    entry: gtk::Entry,
    row: gtk::Box,
    col_holder: gtk::Box,
    check: gtk::Button,
    stop: gtk::Button,
    setup_say: gtk::Label,
    count_btns: Vec<(usize, gtk::ToggleButton)>,
    start: gtk::Button,
    learn: Rc<Learn>,
    scratch: Rc<Scratch>,
    tip: gtk::Label,
    extra_keys: gtk::Box,
    hints: gtk::Box,
    hint_btn: gtk::Button,
    confetti: Confetti,
    done_stars: gtk::Label,
    done_msg: gtk::Label,
    done_stats: gtk::Label,
    next_level: gtk::Button,
}

pub fn key(text: &str, extra: bool) -> gtk::Button {
    let b = gtk::Button::builder().label(text).focus_on_click(false).css_classes(["key"]).build();
    if extra {
        b.add_css_class("extra");
    }
    b
}

pub fn page(app: &Rc<App>, flat: usize) -> adw::NavigationPage {
    let (world, level) = levels::all().nth(flat).expect("level index");
    let w = &levels::WORLDS[world];

    // ── Top: progress and streak
    let progress = gtk::ProgressBar::builder().hexpand(true).valign(gtk::Align::Center).build();
    let count = label("", "count");
    let streak_label = label("", "streak");
    let top = gtk::Box::builder().spacing(14).build();
    let stop = gtk::Button::builder().label("🏁 Stop here").css_classes(["soft"]).tooltip_text("Finish now and get stars for what you've done").build();
    top.append(&progress);
    top.append(&count);
    top.append(&streak_label);
    top.append(&stop);

    // ── Kiko and the speech bubble
    let mascot = label("🦉", "mascot");
    let bubble = label("", "bubble");
    bubble.set_wrap(true);
    bubble.set_xalign(0.0);
    bubble.set_valign(gtk::Align::Center);
    let kiko = gtk::Box::builder().spacing(12).build();
    kiko.append(&mascot);
    kiko.append(&bubble);

    // ── The question card
    let visual = label("", "visual");
    visual.set_wrap(true);
    visual.set_justify(gtk::Justification::Center);
    let prompt = gtk::Box::builder().halign(gtk::Align::Center).css_classes(["prompt"]).build();
    let card = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(14).css_classes(["card"]).build();
    let col_holder = gtk::Box::builder().halign(gtk::Align::Center).build();
    let figure_box = gtk::Box::builder().halign(gtk::Align::Center).build();
    card.append(&visual);
    card.append(&figure_box);
    card.append(&prompt);
    card.append(&col_holder);

    // ── Answer row
    let entry = gtk::Entry::builder().hexpand(true).placeholder_text("Your answer").xalign(0.5).css_classes(["answer"]).build();
    let check = gtk::Button::builder().label("Check ✔").css_classes(["go"]).build();
    let row = gtk::Box::builder().spacing(12).build();
    row.append(&entry);
    row.append(&check);
    let tip = label("", "tip");

    // ── Keypad
    let digits = gtk::Grid::builder().row_spacing(8).column_spacing(8).build();
    for (i, d) in ["7", "8", "9", "4", "5", "6", "1", "2", "3"].iter().enumerate() {
        digits.attach(&key(d, false), (i % 3) as i32, (i / 3) as i32, 1, 1);
    }
    let zero = key("0", false);
    digits.attach(&zero, 0, 3, 2, 1);
    let back = key("⌫", true);
    digits.attach(&back, 2, 3, 1, 1);
    let extra_keys = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(8).build();
    let pad = gtk::Box::builder().spacing(8).halign(gtk::Align::Center).build();
    pad.append(&digits);
    pad.append(&extra_keys);

    let game = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(12).margin_top(14).margin_bottom(24).margin_start(28).margin_end(28).build();
    game.append(&top);
    game.append(&kiko);
    game.append(&card);
    game.append(&row);
    game.append(&tip);
    game.append(&pad);

    // ── Setup: how many questions?
    let setup_say = label("", "bubble");
    setup_say.set_wrap(true);
    let setup_kiko = gtk::Box::builder().spacing(12).halign(gtk::Align::Center).build();
    setup_kiko.append(&label("🦉", "mascot"));
    setup_kiko.append(&setup_say);
    let picks = gtk::Box::builder().spacing(14).halign(gtk::Align::Center).homogeneous(true).build();
    let chosen = Rc::new(Cell::new(10));
    let mut count_btns: Vec<(usize, gtk::ToggleButton)> = Vec::new();
    for (n, name) in COUNTS {
        let v = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).build();
        v.append(&label(&n.to_string(), "tile-num"));
        v.append(&label(name, "tile-title"));
        let b = gtk::ToggleButton::builder().child(&v).css_classes(["count-pick"]).build();
        if let Some((_, first)) = count_btns.first() {
            b.set_group(Some(first));
        }
        let chosen = chosen.clone();
        b.connect_toggled(move |b| {
            if b.is_active() {
                chosen.set(n);
            }
        });
        picks.append(&b);
        count_btns.push((n, b));
    }
    let start = gtk::Button::builder().label("Start ➜").css_classes(["go"]).build();
    let show_me = gtk::Button::builder().label("📖 Show me how").css_classes(["soft"]).tooltip_text("Kiko teaches it step by step").build();
    let setup_buttons = gtk::Box::builder().spacing(14).halign(gtk::Align::Center).build();
    setup_buttons.append(&show_me);
    setup_buttons.append(&start);
    let setup = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(26).valign(gtk::Align::Center).margin_start(28).margin_end(28).build();
    let setup_title = label(&format!("{} {}", w.emoji, level.title), "hello");
    setup_title.set_wrap(true);
    setup.append(&setup_title);
    setup.append(&setup_kiko);
    setup.append(&picks);
    setup.append(&setup_buttons);

    // ── Results
    let done_stars = label("", "big-stars");
    let done_msg = label("", "hello");
    done_msg.set_wrap(true);
    done_msg.set_justify(gtk::Justification::Center);
    let done_stats = label("", "sub");
    let again = gtk::Button::builder().label("🔁 Play again").css_classes(["soft"]).build();
    let to_map = gtk::Button::builder().label("🗺️ Map").css_classes(["soft"]).build();
    let next_level = gtk::Button::builder().label("Next level ➜").css_classes(["go", "next"]).build();
    let buttons = gtk::Box::builder().spacing(14).halign(gtk::Align::Center).build();
    buttons.append(&to_map);
    buttons.append(&again);
    buttons.append(&next_level);
    let done = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(18).valign(gtk::Align::Center).margin_start(28).margin_end(28).build();
    done.append(&label("🦉", "mascot"));
    done.append(&done_stars);
    done.append(&done_msg);
    done.append(&done_stats);
    done.append(&buttons);

    let stack = gtk::Stack::builder().transition_type(gtk::StackTransitionType::Crossfade).build();
    let game_clamp = adw::Clamp::builder().maximum_size(820).child(&game).build();
    stack.add_named(&setup, Some("setup"));
    let learn = Learn::new(app, level);
    stack.add_named(&learn.root, Some("learn"));
    stack.add_named(&gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).child(&game_clamp).build(), Some("game"));
    stack.add_named(&done, Some("done"));
    let confetti = Confetti::new();
    let overlay = gtk::Overlay::builder().child(&stack).build();
    overlay.add_overlay(&confetti.area);

    // ── Help panel
    let lesson = math::rich(level.lesson, gtk::Align::Start);
    lesson.add_css_class("lesson");
    let hints = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).build();
    let hint_btn = gtk::Button::builder().label("💡 Show a hint").css_classes(["soft"]).build();
    let side = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(12).margin_top(18).margin_bottom(18).margin_start(18).margin_end(18).build();
    let t1 = label("📘 How it works", "side-title");
    t1.set_xalign(0.0);
    side.append(&t1);
    side.append(&lesson);
    if let Some(f) = figure::for_lesson(level.id) {
        side.append(&figure::widget(&f, 280, 170));
    }
    side.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    let t2 = label("💡 Hints for this one", "side-title");
    t2.set_xalign(0.0);
    side.append(&t2);
    side.append(&hints);
    side.append(&hint_btn);
    let side_scroll = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).vexpand(true).child(&side).build();

    // The scratchpad sits behind a second tab, where the level allows it.
    let scratch = Scratch::new();
    let side_stack = gtk::Stack::builder().transition_type(gtk::StackTransitionType::Crossfade).vexpand(true).build();
    side_stack.add_named(&side_scroll, Some("help"));
    side_stack.add_named(&scratch.root, Some("scratch"));
    let t_help = gtk::ToggleButton::builder().label("💡 Help").active(true).css_classes(["soft"]).build();
    let t_scratch = gtk::ToggleButton::builder().label("🧮 Scratchpad").css_classes(["soft"]).group(&t_help).build();
    let side_tabs = gtk::Box::builder().spacing(6).homogeneous(true).margin_top(12).margin_start(14).margin_end(14).build();
    side_tabs.append(&t_help);
    side_tabs.append(&t_scratch);
    let scratch_ok = app.store.borrow().profiles[app.player.get()].scratchpad_in(world);
    side_tabs.set_visible(scratch_ok);
    // BILANG_SCRATCH=1 opens the scratchpad tab with the cursor in it, for testing
    // (only where the scratchpad is allowed).
    let scratch_test = scratch_ok && std::env::var_os("BILANG_SCRATCH").is_some();
    let sidebar = gtk::Box::builder().orientation(gtk::Orientation::Vertical).css_classes(["side"]).build();
    sidebar.append(&side_tabs);
    sidebar.append(&side_stack);

    let split = adw::OverlaySplitView::builder()
        .sidebar_position(gtk::PackType::End)
        .min_sidebar_width(300.0)
        .max_sidebar_width(380.0)
        .sidebar_width_fraction(0.32)
        .show_sidebar(true)
        .sidebar(&sidebar)
        .content(&overlay)
        .build();

    let help = gtk::ToggleButton::builder().label("💡 Help").active(true).build();
    help.bind_property("active", &split, "show-sidebar").bidirectional().build();
    if scratch_test {
        let t = t_scratch.clone();
        glib::idle_add_local_once(move || t.set_active(true));
    }
    {
        // The scratchpad wants a little more room than the help text.
        let (stack, split) = (side_stack.clone(), split.clone());
        t_help.connect_toggled(move |b| {
            let pad = !b.is_active();
            stack.set_visible_child_name(if pad { "scratch" } else { "help" });
            split.set_max_sidebar_width(if pad { 460.0 } else { 380.0 });
            split.set_sidebar_width_fraction(if pad { 0.4 } else { 0.32 });
        });
    }
    let header = adw::HeaderBar::new();
    header.pack_end(&help);
    header.pack_end(&app.sound_button());
    let tv = adw::ToolbarView::new();
    tv.add_top_bar(&header);
    tv.set_content(Some(&split));
    tv.add_css_class("bg");
    let nav_page = adw::NavigationPage::new(&tv, &format!("{} {}", w.emoji, level.title));

    let play = Rc::new(Play {
        app: app.clone(),
        flat,
        level,
        rng: RefCell::new(StdRng::from_rng(&mut rand::rng())),
        problem: RefCell::new((level.make)(&mut StdRng::seed_from_u64(0))),
        q: Cell::new(0),
        total: Cell::new(10),
        chosen,
        colpad: RefCell::default(),
        clean: Cell::new(0),
        hints_shown: Cell::new(0),
        missed: Cell::new(false),
        streak: Cell::new(0),
        best_streak: Cell::new(0),
        solved: Cell::new(false),
        run_at: Cell::new(None),
        run_hints: Cell::new(0),
        run_wrong: Cell::new(0),
        tries: RefCell::default(),
        needed_help: RefCell::default(),
        stack,
        progress,
        count,
        streak_label,
        mascot,
        bubble,
        card,
        visual,
        figure: figure_box,
        prompt,
        entry,
        row,
        col_holder,
        check,
        stop,
        setup_say,
        count_btns,
        start,
        learn,
        scratch,
        tip,
        extra_keys,
        hints,
        hint_btn,
        confetti,
        done_stars,
        done_msg,
        done_stats,
        next_level,
    });

    for child in digits_children(&digits) {
        let p = play.clone();
        child.connect_clicked(move |b| p.press(&b.label().unwrap_or_default()));
    }
    {
        let p = play.clone();
        play.entry.connect_activate(move |_| p.submit());
    }
    {
        let p = play.clone();
        play.check.connect_clicked(move |_| p.submit());
    }
    {
        let p = play.clone();
        play.start.connect_clicked(move |_| p.begin());
    }
    {
        // "↪" on a scratchpad line puts that result in the answer box.
        let p = Rc::downgrade(&play);
        play.scratch.connect_use(move |value| {
            if let Some(p) = p.upgrade()
                && !p.solved.get()
            {
                p.entry.set_text(value);
                p.entry.grab_focus();
                p.entry.set_position(-1);
            }
        });
    }
    {
        let p = play.clone();
        show_me.connect_clicked(move |_| p.open_learn());
    }
    {
        // After the lesson: practice straight away, a short round.
        let p = Rc::downgrade(&play);
        play.learn.connect_done(move || {
            if let Some(p) = p.upgrade() {
                p.mark_learned();
                p.record(true, 0, true);
                p.chosen.set(10);
                p.begin();
            }
        });
        let p = Rc::downgrade(&play);
        play.learn.connect_skip(move || {
            if let Some(p) = p.upgrade() {
                p.mark_learned();
                p.restart();
            }
        });
    }
    {
        let p = play.clone();
        play.stop.connect_clicked(move |_| p.finish());
    }
    {
        let p = play.clone();
        play.hint_btn.connect_clicked(move |_| p.show_hint(true));
    }
    {
        let p = play.clone();
        again.connect_clicked(move |_| p.restart());
    }
    {
        let p = play.clone();
        to_map.connect_clicked(move |_| {
            p.app.nav.pop_to_tag("map");
        });
    }
    {
        let p = play.clone();
        play.next_level.connect_clicked(move |_| {
            let app = p.app.clone();
            let next = p.flat + 1;
            app.nav.pop_to_tag("map");
            app.nav.push(&page(&app, next));
        });
    }
    {
        // Leaving in the middle still counts on the score sheet.
        let p = play.clone();
        nav_page.connect_hidden(move |_| {
            if p.stack.visible_child_name().as_deref() == Some("game") && p.answered() > 0 {
                p.record(false, 0, false);
            }
        });
    }
    {
        let p = play.clone();
        nav_page.connect_shown(move |_| p.focus_input());
    }
    play.restart();
    nav_page
}

fn digits_children(grid: &gtk::Grid) -> Vec<gtk::Button> {
    let mut out = Vec::new();
    let mut c = grid.first_child();
    while let Some(w) = c {
        c = w.next_sibling();
        if let Ok(b) = w.downcast::<gtk::Button>() {
            out.push(b);
        }
    }
    out
}

impl Play {
    fn say(&self, face: &str, text: &str) {
        self.mascot.set_text(face);
        self.bubble.set_text(text);
    }

    fn press(self: &Rc<Self>, k: &str) {
        if self.solved.get() {
            return;
        }
        if let Some(pad) = self.colpad.borrow().as_ref() {
            pad.press(k);
            pad.focus();
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

    /// Back to the "how many questions?" screen, with Kiko's pick selected.
    /// The very first time on a level, Kiko teaches it first.
    fn restart(self: &Rc<Self>) {
        let first_time = {
            let s = self.app.store.borrow();
            let p = &s.profiles[self.app.player.get()];
            p.stars(self.level.id) == 0 && !p.learned.contains(self.level.id)
        };
        if first_time {
            self.open_learn();
            return;
        }
        let (suggest, why) = self.app.store.borrow().profiles[self.app.player.get()].suggested_count(self.level.id);
        for (n, b) in &self.count_btns {
            if *n == suggest {
                b.set_active(true);
            }
        }
        self.setup_say.set_text(&format!("How many questions today? {why} I suggest {suggest}!"));
        clear(&self.hints);
        self.hint_btn.set_sensitive(false);
        self.hint_btn.set_label("💡 Hints come during practice");
        self.stack.set_visible_child_name("setup");
        self.start.grab_focus();
    }

    fn open_learn(self: &Rc<Self>) {
        self.scratch.reset(&[]);
        // During a lesson Kiko does the hinting; the side panel keeps just the lesson text.
        clear(&self.hints);
        self.hint_btn.set_sensitive(false);
        self.hint_btn.set_label("💡 Hints come during practice");
        self.stack.set_visible_child_name("learn");
        self.run_at.set(Some((now(), Instant::now())));
        self.learn.start();
    }

    fn mark_learned(&self) {
        let mut s = self.app.store.borrow_mut();
        let i = self.app.player.get();
        if s.profiles[i].learned.insert(self.level.id.to_string()) {
            s.save();
        }
    }

    fn begin(self: &Rc<Self>) {
        self.total.set(self.chosen.get());
        self.q.set(0);
        self.clean.set(0);
        self.streak.set(0);
        self.best_streak.set(0);
        self.run_at.set(Some((now(), Instant::now())));
        self.run_hints.set(0);
        self.run_wrong.set(0);
        self.needed_help.borrow_mut().clear();
        self.app.cue(Cue::Tap);
        self.stack.set_visible_child_name("game");
        self.next_question();
    }

    /// Save this run (or lesson) to the score sheet, once.
    fn record(&self, finished: bool, stars: u8, lesson: bool) {
        let Some((at, started)) = self.run_at.take() else { return };
        let (dir, id) = {
            let s = self.app.store.borrow();
            (s.dir().to_path_buf(), s.profiles[self.app.player.get()].id)
        };
        let answered = if lesson { 0 } else { self.answered() as u32 };
        history::append(
            &dir,
            id,
            Run {
                level: self.level.id.to_string(),
                at,
                seconds: started.elapsed().as_secs() as u32,
                lesson,
                total: if lesson { 0 } else { self.total.get() as u32 },
                answered,
                first_try: if lesson { 0 } else { self.clean.get() as u32 },
                hints: self.run_hints.get(),
                wrong: self.run_wrong.get(),
                stars,
                finished,
                missed: if lesson { Vec::new() } else { self.needed_help.take() },
            },
        );
    }

    fn focus_input(&self) {
        let on = self.stack.visible_child_name();
        if on.as_deref() == Some("setup") {
            self.start.grab_focus();
        } else if on.as_deref() == Some("learn") {
            self.learn.focus();
        } else if let Some(pad) = self.colpad.borrow().as_ref() {
            pad.focus();
        } else {
            self.entry.grab_focus();
        }
    }

    /// How many are done, counting a question answered but not yet moved past.
    fn answered(&self) -> usize {
        self.q.get() + self.solved.get() as usize
    }

    /// "Stop here" shows once ten are done in a longer run.
    fn update_stop(&self) {
        self.stop.set_visible(self.total.get() > 10 && self.answered() >= 10 && self.answered() < self.total.get());
    }

    fn next_question(self: &Rc<Self>) {
        let p = (self.level.make)(&mut self.rng.borrow_mut());
        let q = self.q.get();
        let total = self.total.get();
        self.progress.set_fraction(q as f64 / total as f64);
        self.count.set_text(&format!("{} / {total}", q + 1));
        self.streak_label.set_text(&if self.streak.get() >= 2 { format!("🔥 {}", self.streak.get()) } else { String::new() });

        self.visual.set_text(p.visual.as_deref().unwrap_or(""));
        self.visual.set_visible(p.visual.is_some());
        while let Some(c) = self.figure.first_child() {
            self.figure.remove(&c);
        }
        if let Some(f) = &p.figure {
            self.figure.append(&figure::widget(f, 340, 185));
        }
        self.figure.set_visible(p.figure.is_some());
        math::set(&self.prompt, &p.prompt, gtk::Align::Center);
        if p.prompt.chars().count() > 34 || p.column.is_some() {
            self.prompt.add_css_class("long");
        } else {
            self.prompt.remove_css_class("long");
        }
        self.card.remove_css_class("right");
        self.card.remove_css_class("wrong");
        self.tip.set_text(p.answer.format_tip());
        self.scratch.reset(&p.given);
        if std::env::var_os("BILANG_SCRATCH").is_some() && self.scratch.root.is_drawable() {
            let s = self.scratch.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(300), move || s.focus());
        }
        clear(&self.col_holder);
        *self.colpad.borrow_mut() = None;
        if let Some(col) = p.column {
            let pad = ColumnPad::new(col);
            let me = Rc::downgrade(self);
            pad.connect_enter(move || {
                if let Some(me) = me.upgrade() {
                    me.submit();
                }
            });
            let app = self.app.clone();
            pad.connect_key(move || app.cue(Cue::Tap));
            self.col_holder.append(&pad.root);
            self.tip.set_text(if col.op == levels::ColOp::Div {
                "Type the answer on top, left to right. Put what's left over in the r box."
            } else {
                if col.op == levels::ColOp::Sub {
                    "Type the answer from right to left. Click a small box on top to write a digit's new value after borrowing."
                } else {
                    "Type the answer from right to left. Click a small box on top to write a carry."
                }
            });
            *self.colpad.borrow_mut() = Some(pad);
        }
        let in_columns = p.column.is_some();
        self.col_holder.set_visible(in_columns);
        self.entry.set_visible(!in_columns);
        self.row.set_halign(if in_columns { gtk::Align::Center } else { gtk::Align::Fill });

        clear(&self.extra_keys);
        for k in if in_columns { &[][..] } else { p.answer.keys() } {
            let b = key(k, true);
            let me = self.clone();
            b.connect_clicked(move |b| me.press(&b.label().unwrap_or_default()));
            self.extra_keys.append(&b);
        }

        clear(&self.hints);
        self.tries.borrow_mut().clear();
        self.hints_shown.set(0);
        self.missed.set(false);
        self.solved.set(false);
        self.hint_btn.set_sensitive(true);
        self.hint_btn.set_label(&format!("💡 Show a hint (1 of {})", p.hints.len()));
        self.check.set_label("Check ✔");
        self.check.remove_css_class("next");
        self.entry.set_text("");
        self.entry.set_editable(true);
        // BILANG_DEBUG=1 logs each answer, so a script can play a level through.
        if std::env::var_os("BILANG_DEBUG").is_some() {
            eprintln!("answer: {}", p.answer.show().replace('−', "-"));
        }
        *self.problem.borrow_mut() = p;

        self.update_stop();
        let hello = match q {
            0 => format!("Let's do \"{}\"! Need help? Look at the side panel. →", self.level.title),
            _ if q + 1 == total => "Last one! You can do it! 💪".to_string(),
            _ if total >= 20 && q == total / 2 => "Halfway there! Look how far you've come! 🎉".to_string(),
            _ if total >= 50 && q % 10 == 0 => format!("{q} done already! Keep going! 💪"),
            _ if self.streak.get() >= 3 => format!("{} in a row! You're on fire! 🔥", self.streak.get()),
            _ => ["Here's the next one!", "Ready? Let's go!", "Take your time. 🙂", "You've got this!"][self.rng.borrow_mut().random_range(0..4)].to_string(),
        };
        self.say("🦉", &hello);
        self.focus_input();
    }

    fn show_hint(self: &Rc<Self>, asked: bool) {
        let p = self.problem.borrow();
        let i = self.hints_shown.get();
        if i >= p.hints.len() {
            return;
        }
        let last = i + 1 == p.hints.len();
        let h = math::rich(&p.hints[i], gtk::Align::Start);
        h.add_css_class("hint");
        if last {
            h.add_css_class("last");
        }
        self.hints.append(&h);
        self.hints_shown.set(i + 1);
        self.run_hints.set(self.run_hints.get() + 1);
        if last {
            self.hint_btn.set_label("That's every step!");
            self.hint_btn.set_sensitive(false);
        } else {
            self.hint_btn.set_label(&format!("💡 Show the next hint ({} of {})", i + 2, p.hints.len()));
        }
        if asked {
            self.app.cue(Cue::Hint);
            self.say("💡", if last { "Here's the whole way to solve it. Follow the steps!" } else { "Here's a clue! Check the side panel. →" });
        }
    }

    fn submit(self: &Rc<Self>) {
        if self.solved.get() {
            self.advance();
            return;
        }
        let pad = self.colpad.borrow().clone();
        let text = match &pad {
            Some(pad) => match pad.text() {
                Ok(t) => t,
                Err(msg) => {
                    self.say("🤔", &msg);
                    pad.focus();
                    return;
                }
            },
            None => self.entry.text().to_string(),
        };
        let verdict = self.problem.borrow().answer.check(&text);
        if let Some(pad) = &pad
            && verdict != Verdict::Right
        {
            pad.mark(&self.problem.borrow().answer);
        }
        match verdict {
            Verdict::Right => {
                self.solved.set(true);
                self.entry.set_editable(false);
                if let Some(pad) = &pad {
                    pad.mark(&self.problem.borrow().answer);
                    pad.set_locked(true);
                }
                self.card.remove_css_class("wrong");
                self.card.add_css_class("right");
                if !self.missed.get() && self.hints_shown.get() <= 1 {
                    self.clean.set(self.clean.get() + 1);
                } else {
                    let p = self.problem.borrow();
                    self.needed_help.borrow_mut().push(Missed {
                        question: p.describe(),
                        answer: p.answer.show(),
                        tries: self.tries.take(),
                        hints: self.hints_shown.get() as u32,
                    });
                }
                let s = if self.missed.get() { 0 } else { self.streak.get() + 1 };
                self.streak.set(s);
                self.best_streak.set(self.best_streak.get().max(s));
                self.streak_label.set_text(&if s >= 2 { format!("🔥 {s}") } else { String::new() });
                let msg = if self.missed.get() {
                    "Yes! You figured it out! That's how we learn 🌱".to_string()
                } else {
                    PRAISE[self.rng.borrow_mut().random_range(0..PRAISE.len())].to_string()
                };
                self.say("🥳", &msg);
                self.app.cue(Cue::Right);
                self.confetti.burst(if s >= 3 { 90 } else { 45 });
                let total = self.total.get();
                self.progress.set_fraction((self.q.get() + 1) as f64 / total as f64);
                self.check.set_label(if self.q.get() + 1 == total { "Finish 🏁" } else { "Next ➜" });
                self.check.add_css_class("next");
                self.update_stop();
                self.focus_input();
            }
            Verdict::Almost(msg) => {
                self.tries.borrow_mut().push(text.clone());
                self.app.cue(Cue::Almost);
                self.say("🤩", &msg);
                self.focus_input();
            }
            Verdict::Unreadable(msg) => {
                self.say("🤔", &msg);
                self.focus_input();
            }
            Verdict::Wrong => {
                self.tries.borrow_mut().push(text.clone());
                self.run_wrong.set(self.run_wrong.get() + 1);
                self.missed.set(true);
                self.streak.set(0);
                self.streak_label.set_text("");
                self.card.add_css_class("wrong");
                self.app.cue(Cue::Wrong);
                let all_shown = self.hints_shown.get() >= self.problem.borrow().hints.len();
                if all_shown {
                    // Never leave a kid stuck: after every hint, Kiko just tells them.
                    let ans = self.problem.borrow().answer.show();
                    self.say("🦉", &format!("The answer is {ans}. Look at the steps on the side, then type it in!"));
                } else if pad.is_some() {
                    self.say("🤔", "Not quite! The red boxes need another look. The hint shows each column. 💡");
                } else {
                    let msg = OOPS[self.rng.borrow_mut().random_range(0..OOPS.len())];
                    self.say("🤔", msg);
                }
                // A wrong try reveals the next hint by itself.
                self.show_hint(false);
                self.focus_input();
                if pad.is_none() {
                    self.entry.select_region(0, -1);
                }
                let card = self.card.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(900), move || card.remove_css_class("wrong"));
            }
        }
    }

    fn advance(self: &Rc<Self>) {
        let q = self.q.get() + 1;
        self.q.set(q);
        self.solved.set(false);
        if q >= self.total.get() {
            self.finish();
        } else {
            self.next_question();
        }
    }

    fn finish(self: &Rc<Self>) {
        let answered = self.answered().max(1);
        let total = self.total.get();
        let clean = self.clean.get();
        let pct = clean * 100 / answered;
        let stars: u8 = match pct {
            90.. => 3,
            60.. => 2,
            _ => 1,
        };
        // Medals are for finishing a long run with 3 stars.
        let medal = (answered >= total && total >= 50 && stars == 3).then_some(total as u32);
        let has_next = levels::all().nth(self.flat + 1).is_some();
        let new_medal = {
            let mut s = self.app.store.borrow_mut();
            let p = &mut s.profiles[self.app.player.get()];
            let best = p.stars(self.level.id).max(stars);
            p.stars.insert(self.level.id.to_string(), best);
            p.solved += answered as u32;
            p.best_streak = p.best_streak.max(self.best_streak.get());
            let new_medal = medal.filter(|&m| m > p.medal(self.level.id));
            if let Some(m) = new_medal {
                p.medals.insert(self.level.id.to_string(), m);
            }
            s.save();
            new_medal
        };
        self.record(answered >= total, stars, false);
        self.done_stars.set_text(&star_text(stars));
        self.done_msg.set_text(match (new_medal, stars) {
            (Some(100), _) => "MARATHON TROPHY! 🏆 100 questions with 3 stars. Amazing!",
            (Some(_), _) => "CHALLENGE MEDAL! 🥈 50 questions with 3 stars!",
            (_, 3) => "PERFECT! You're a math superstar! 🌟",
            (_, 2) => "Great work! Play again to try for 3 stars!",
            _ => "You finished! Every try makes your brain stronger 💪",
        });
        self.done_stats.set_text(&format!(
            "{clean} of {answered} on the first try{} · best streak 🔥 {}{}",
            if answered < total { format!(" (stopped at {answered} of {total})") } else { String::new() },
            self.best_streak.get(),
            if has_next { "\nThe next level is open!" } else { "\nYou reached the top of Algebra Peak! 🏔️" }
        ));
        self.next_level.set_visible(has_next);
        self.stack.set_visible_child_name("done");
        self.app.cue(Cue::LevelDone);
        self.confetti.burst(160);
        let c = self.confetti.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || c.burst(120));
    }
}

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}
