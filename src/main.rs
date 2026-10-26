//! Bilang: a math adventure for kids. Pick your player, walk the map from
//! counting to quadratics, and Kiko the owl cheers you on with hints that
//! teach the method, not just the answer.

mod answer;
mod column;
mod confetti;
mod expr;
mod history;
mod learn;
mod levels;
mod play;
mod scores;
mod sound;
mod store;
mod teach;

use std::{cell::Cell, cell::RefCell, rc::Rc};

use adw::prelude::*;
use gtk::{gdk, glib};

use sound::{Cue, Sound};
use store::Store;

const APP_ID: &str = "io.r3al.bilang";

pub const AVATARS: [&str; 16] = ["🦊", "🐼", "🐯", "🐸", "🦄", "🐵", "🐧", "🐨", "🦁", "🐰", "🐶", "🐱", "🐢", "🐙", "🦖", "🐝"];

const CSS: &str = r#"
window, dialog, popover { font-family: "Chalkboard SE", "Comic Neue", "Comic Sans MS", "Nunito", sans-serif; }
.bg { background: linear-gradient(180deg, #fff8e1 0%, #e3f2fd 100%); color: #2d2d5a; }
.hello { font-size: 34px; font-weight: 800; color: #3a3a7a; }
.sub { font-size: 18px; color: #5a5a8a; }
.mascot { font-size: 60px; }
.bubble { background: white; border-radius: 22px; padding: 14px 20px; font-size: 20px; color: #333;
          box-shadow: 0 3px 10px alpha(black, 0.12); }
.card { background: white; border-radius: 28px; padding: 18px 26px; box-shadow: 0 4px 16px alpha(black, 0.10);
        transition: background 250ms ease-in-out; }
.card.right { background: #c9f5c4; }
.card.wrong { background: #ffd9d9; }
.prompt { font-size: 42px; font-weight: 800; color: #3a3a7a; }
.prompt.long { font-size: 26px; font-weight: 700; }
.visual { font-size: 34px; }
entry.answer { font-size: 32px; min-height: 54px; border-radius: 18px; font-weight: 700; }
button.key { font-size: 26px; min-width: 66px; min-height: 48px; border-radius: 18px; font-weight: 800;
             background: white; color: #3a3a7a; box-shadow: 0 2px 5px alpha(black, 0.10); }
button.key:hover { background: #f0f4ff; }
button.key.extra { background: #fff0c2; }
button.go { font-size: 24px; font-weight: 800; border-radius: 18px; padding: 10px 30px; min-height: 52px;
            background: #43b047; color: white; }
button.go:hover { background: #3a9a3e; }
button.go.next { background: #3f7fe0; }
button.go.next:hover { background: #336dc4; }
button.soft { font-size: 18px; border-radius: 16px; padding: 8px 18px; font-weight: 700; }
.bubble.teach { font-size: 22px; }
.tip { font-size: 15px; color: #6a6a8a; }
.count { font-size: 18px; font-weight: 700; }
.streak { font-size: 20px; font-weight: 800; color: #e8590c; }
progressbar > trough { min-height: 16px; border-radius: 10px; background: alpha(#3a3a7a, 0.12); }
progressbar > trough > progress { min-height: 16px; border-radius: 10px; background: linear-gradient(90deg, #ffb703, #fb8500); }
.side { background: #fffdf4; border-left: 1px solid alpha(black, 0.08); }
.side-title { font-size: 20px; font-weight: 800; color: #3a3a7a; }
.lesson { font-size: 16px; color: #333; }
.hint { background: #fff3c4; border-radius: 16px; padding: 12px 14px; color: #4d3b00; font-size: 16px; }
.hint.last { background: #dff3ff; color: #0b3a5c; }
.world { border-radius: 28px; padding: 18px; }
.world-title { font-size: 26px; font-weight: 800; color: #2d2d5a; }
.world-0 { background: #d8f5d2; } .world-1 { background: #ffe3bd; } .world-2 { background: #e6dbfa; }
.world-3 { background: #d3ebfd; } .world-4 { background: #ffd8c9; } .world-5 { background: #e1e8f0; }
button.tile { border-radius: 22px; padding: 10px; min-width: 150px; min-height: 112px; background: white;
              color: #2d2d5a; box-shadow: 0 3px 8px alpha(black, 0.10); }
button.tile:hover { background: #fffbe6; }
button.tile:disabled { background: alpha(white, 0.55); color: alpha(#2d2d5a, 0.45); box-shadow: none; }
.tile-num { font-size: 28px; font-weight: 800; }
.tile-title { font-size: 15px; font-weight: 700; }
.stars { font-size: 20px; color: #f2a900; }
button.player { border-radius: 28px; padding: 16px; min-width: 160px; min-height: 170px; background: white;
                color: #2d2d5a; box-shadow: 0 4px 12px alpha(black, 0.10); }
button.player:hover { background: #fffbe6; }
.avatar { font-size: 68px; }
.player-name { font-size: 22px; font-weight: 800; }
.big-stars { font-size: 84px; color: #f2a900; }
button.count-pick { border-radius: 22px; padding: 12px; min-width: 130px; min-height: 100px; background: white;
                          color: #2d2d5a; box-shadow: 0 3px 8px alpha(black, 0.10); }
button.count-pick:checked { background: #ffe08a; box-shadow: 0 0 0 4px #fb8500; }
.colgrid .digit { font-size: 40px; font-weight: 800; min-width: 46px; color: #3a3a7a; }
.colgrid .bracket { font-size: 52px; font-weight: 300; color: #3a3a7a; }
.colgrid .slot { font-size: 36px; font-weight: 800; min-width: 44px; min-height: 52px; border: 3px dashed #b8c4e8;
                 border-radius: 12px; color: #2d2d5a; background: #fbfcff; }
.colgrid .slot.active { border: 3px solid #fb8500; background: #fff7e0; }
.colgrid .slot.good { border: 3px solid #43b047; background: #c9f5c4; }
.colgrid .slot.bad { border: 3px solid #e5484d; background: #ffd9d9; }
.colgrid .carry { font-size: 17px; font-weight: 800; min-width: 26px; min-height: 26px; border: 2px dashed #f0c060;
                  border-radius: 8px; color: #d9480f; }
.colgrid .carry.active { border: 2px solid #fb8500; background: #fff7e0; }
.colgrid .digit.pointed { background: #ffe58a; border-radius: 12px; }
.colgrid .slot.pointed, .colgrid .carry.pointed { background: #fff3c4; }
.colgrid separator.rule { min-height: 4px; background: #3a3a7a; }
.stat { background: white; border-radius: 20px; padding: 14px 10px; box-shadow: 0 3px 8px alpha(black, 0.08); }
.stat-num { font-size: 26px; font-weight: 800; color: #2d2d5a; }
.stat-cap { font-size: 14px; color: #5a5a8a; }
.stat-cap.today { font-weight: 800; color: #2d2d5a; }
.week { background: white; border-radius: 20px; padding: 18px 22px 12px 22px; box-shadow: 0 3px 8px alpha(black, 0.08); }
.week .bar { background: #3f7fe0; border-radius: 4px 4px 0 0; }
.week .bar-empty { background: alpha(#3a3a7a, 0.15); border-radius: 2px; }
.sheet list.boxed-list row { color: #2d2d5a; font-size: 16px; }
button.pick { font-size: 34px; border-radius: 16px; min-width: 56px; min-height: 56px; }
button.pick:checked { background: #ffe08a; }
"#;

pub struct App {
    pub win: adw::ApplicationWindow,
    pub nav: adw::NavigationView,
    pub store: RefCell<Store>,
    /// Index of the current player in store.profiles.
    pub player: Cell<usize>,
    sound: Sound,
}

impl App {
    pub fn cue(&self, c: Cue) {
        if self.store.borrow().sound {
            self.sound.play(c);
        }
    }

    /// The 🔊 button that sits in every header bar.
    pub fn sound_button(self: &Rc<Self>) -> gtk::ToggleButton {
        let on = self.store.borrow().sound;
        let b = gtk::ToggleButton::builder().label(if on { "🔊" } else { "🔇" }).active(on).tooltip_text("Sounds").build();
        let app = self.clone();
        b.connect_toggled(move |b| {
            b.set_label(if b.is_active() { "🔊" } else { "🔇" });
            let mut s = app.store.borrow_mut();
            s.sound = b.is_active();
            s.save();
        });
        b
    }
}

pub fn clear(b: &gtk::Box) {
    while let Some(c) = b.first_child() {
        b.remove(&c);
    }
}

pub fn label(text: &str, class: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    if !class.is_empty() {
        l.add_css_class(class);
    }
    l
}

pub fn star_text(n: u8) -> String {
    (0..3).map(|i| if i < n { '★' } else { '☆' }).collect()
}

// ───────────────────────── Who's playing? ─────────────────────────

fn players_page(app: &Rc<App>) -> adw::NavigationPage {
    let flow = gtk::FlowBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .homogeneous(true)
        .max_children_per_line(5)
        .min_children_per_line(2)
        .column_spacing(18)
        .row_spacing(18)
        .halign(gtk::Align::Center)
        .build();
    let body = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(24).margin_top(30).margin_bottom(40).margin_start(24).margin_end(24).build();
    let greet = gtk::Box::builder().spacing(16).halign(gtk::Align::Center).build();
    greet.append(&label("🦉", "mascot"));
    let words = gtk::Box::builder().orientation(gtk::Orientation::Vertical).valign(gtk::Align::Center).build();
    let hello = label("Hi! I'm Kiko.", "hello");
    hello.set_xalign(0.0);
    let sub = label("Let's go on a math adventure! Who's playing today?", "sub");
    sub.set_xalign(0.0);
    words.append(&hello);
    words.append(&sub);
    greet.append(&words);
    body.append(&greet);
    body.append(&flow);

    let scroll = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).vexpand(true).child(&body).build();
    let tv = adw::ToolbarView::new();
    let header = adw::HeaderBar::new();
    header.pack_end(&app.sound_button());
    let sheets = gtk::Button::builder().label("📊 Score sheets").tooltip_text("For grown-ups: see each child's progress").build();
    {
        let app = app.clone();
        sheets.connect_clicked(move |_| {
            if !app.store.borrow().profiles.is_empty() {
                app.nav.push(&scores::page(&app, 0));
            }
        });
    }
    header.pack_start(&sheets);
    tv.add_top_bar(&header);
    tv.set_content(Some(&scroll));
    tv.add_css_class("bg");
    let page = adw::NavigationPage::new(&tv, "Bilang");

    let refresh: Rc<dyn Fn()> = {
        let app = app.clone();
        let flow = flow.clone();
        Rc::new(move || {
            flow.remove_all();
            let profiles = app.store.borrow().profiles.clone();
            for (i, p) in profiles.iter().enumerate() {
                let b = gtk::Button::builder().css_classes(["player"]).build();
                let v = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(4).valign(gtk::Align::Center).build();
                v.append(&label(&p.avatar, "avatar"));
                v.append(&label(&p.name, "player-name"));
                v.append(&label(&format!("⭐ {}", p.total_stars()), "sub"));
                b.set_child(Some(&v));
                let app = app.clone();
                b.connect_clicked(move |_| {
                    app.cue(Cue::Tap);
                    app.player.set(i);
                    app.nav.push(&map_page(&app));
                });
                flow.append(&b);
            }
            let add = gtk::Button::builder().css_classes(["player"]).build();
            let v = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(4).valign(gtk::Align::Center).build();
            v.append(&label("➕", "avatar"));
            v.append(&label("New player", "player-name"));
            add.set_child(Some(&v));
            let app = app.clone();
            add.connect_clicked(move |_| new_player(&app));
            flow.append(&add);
        })
    };
    refresh();
    page.connect_showing(move |_| refresh());
    page
}

fn new_player(app: &Rc<App>) {
    let dialog = adw::AlertDialog::new(Some("New player"), Some("What's your name? Pick an animal buddy too!"));
    let v = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(14).build();
    let name = gtk::Entry::builder().placeholder_text("Your name").max_length(20).build();
    name.add_css_class("answer");
    v.append(&name);
    let grid = gtk::FlowBox::builder().selection_mode(gtk::SelectionMode::None).max_children_per_line(8).min_children_per_line(4).homogeneous(true).build();
    let chosen = Rc::new(RefCell::new(AVATARS[0].to_string()));
    let mut first: Option<gtk::ToggleButton> = None;
    for a in AVATARS {
        let t = gtk::ToggleButton::builder().label(a).css_classes(["pick"]).build();
        match &first {
            Some(f) => t.set_group(Some(f)),
            None => {
                t.set_active(true);
                first = Some(t.clone());
            }
        }
        let chosen = chosen.clone();
        t.connect_toggled(move |t| {
            if t.is_active() {
                *chosen.borrow_mut() = a.to_string();
            }
        });
        grid.append(&t);
    }
    v.append(&grid);
    dialog.set_extra_child(Some(&v));
    dialog.add_responses(&[("cancel", "Cancel"), ("create", "Let's go!")]);
    dialog.set_response_appearance("create", adw::ResponseAppearance::Suggested);
    dialog.set_default_response(Some("create"));
    dialog.set_close_response("cancel");
    dialog.set_response_enabled("create", false);
    {
        let dialog = dialog.clone();
        name.connect_changed(move |e| dialog.set_response_enabled("create", !e.text().trim().is_empty()));
    }
    name.set_activates_default(true);
    let app2 = app.clone();
    dialog.connect_response(None, move |_, r| {
        if r != "create" {
            return;
        }
        let n = name.text().trim().to_string();
        if n.is_empty() {
            return;
        }
        let i = app2.store.borrow_mut().add(n, chosen.borrow().clone());
        app2.cue(Cue::Right);
        app2.player.set(i);
        app2.nav.push(&map_page(&app2));
    });
    dialog.present(Some(&app.win));
}

// ───────────────────────── The adventure map ─────────────────────────

fn map_page(app: &Rc<App>) -> adw::NavigationPage {
    let (name, avatar) = {
        let s = app.store.borrow();
        let p = &s.profiles[app.player.get()];
        (p.name.clone(), p.avatar.clone())
    };
    let body = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(22).margin_top(20).margin_bottom(40).margin_start(20).margin_end(20).build();
    let clamp = adw::Clamp::builder().maximum_size(1100).child(&body).build();
    let scroll = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).vexpand(true).child(&clamp).build();

    let tv = adw::ToolbarView::new();
    let header = adw::HeaderBar::new();
    header.pack_end(&grown_up_menu(app));
    header.pack_end(&app.sound_button());
    tv.add_top_bar(&header);
    tv.set_content(Some(&scroll));
    tv.add_css_class("bg");
    let page = adw::NavigationPage::builder().child(&tv).title(format!("{avatar} {name}'s Adventure")).tag("map").build();

    let refresh = {
        let app = app.clone();
        move || {
            clear(&body);
            let p = app.store.borrow().profiles[app.player.get()].clone();
            let top = gtk::Box::builder().spacing(14).build();
            top.append(&label("🦉", "mascot"));
            let words = gtk::Box::builder().orientation(gtk::Orientation::Vertical).valign(gtk::Align::Center).build();
            let next = levels::all().enumerate().find(|(i, (_, l))| p.unlocked(*i) && p.stars(l.id) == 0);
            let hello = label(&format!("Hi {}! {}", p.name, p.avatar), "hello");
            hello.set_xalign(0.0);
            let sub = label(
                &match next {
                    Some((_, (_, l))) if p.total_stars() == 0 => format!("Let's start with \"{}\"! Tap it below.", l.title),
                    Some((_, (_, l))) => format!("You have ⭐ {} stars! Next up: \"{}\".", p.total_stars(), l.title),
                    None => format!("You finished every level! ⭐ {} stars. Can you get 3 stars everywhere?", p.total_stars()),
                },
                "sub",
            );
            sub.set_xalign(0.0);
            sub.set_wrap(true);
            words.append(&hello);
            words.append(&sub);
            top.append(&words);
            body.append(&top);

            let mut flat = 0;
            for (w, world) in levels::WORLDS.iter().enumerate() {
                let card = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(12).css_classes(["world", &format!("world-{w}")]).build();
                let head = gtk::Box::builder().spacing(10).build();
                head.append(&label(&format!("{} {}", world.emoji, world.name), "world-title"));
                let g = label(world.grades, "sub");
                g.set_valign(gtk::Align::Center);
                head.append(&g);
                card.append(&head);
                let tiles = gtk::FlowBox::builder()
                    .selection_mode(gtk::SelectionMode::None)
                    .homogeneous(true)
                    .max_children_per_line(5)
                    .min_children_per_line(2)
                    .column_spacing(12)
                    .row_spacing(12)
                    .build();
                for level in world.levels {
                    let open = p.unlocked(flat);
                    let b = gtk::Button::builder().css_classes(["tile"]).sensitive(open).build();
                    let v = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).valign(gtk::Align::Center).build();
                    v.append(&label(&if open { (flat + 1).to_string() } else { "🔒".into() }, "tile-num"));
                    let t = label(level.title, "tile-title");
                    t.set_wrap(true);
                    t.set_justify(gtk::Justification::Center);
                    v.append(&t);
                    if open {
                        let medal = match p.medal(level.id) {
                            100.. => " 🏆",
                            50.. => " 🥈",
                            _ => "",
                        };
                        v.append(&label(&format!("{}{medal}", star_text(p.stars(level.id))), "stars"));
                    }
                    b.set_child(Some(&v));
                    let app = app.clone();
                    let i = flat;
                    b.connect_clicked(move |_| {
                        app.cue(Cue::Tap);
                        app.nav.push(&play::page(&app, i));
                    });
                    tiles.append(&b);
                    flat += 1;
                }
                card.append(&tiles);
                body.append(&card);
            }
        }
    };
    refresh();
    page.connect_showing(move |_| refresh());
    page
}

/// Settings a grown-up might change, tucked in a menu.
fn grown_up_menu(app: &Rc<App>) -> gtk::MenuButton {
    let v = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).margin_top(10).margin_bottom(10).margin_start(10).margin_end(10).build();
    v.append(&label("For grown-ups", "side-title"));
    let unlock = gtk::CheckButton::builder().label("Unlock all levels").active(app.store.borrow().profiles[app.player.get()].unlock_all).build();
    {
        let app = app.clone();
        unlock.connect_toggled(move |c| {
            {
                let mut s = app.store.borrow_mut();
                let i = app.player.get();
                s.profiles[i].unlock_all = c.is_active();
                s.save();
            }
            // Re-show the map so the tiles update.
            if let Some(page) = app.nav.find_page("map") {
                page.emit_by_name::<()>("showing", &[]);
            }
        });
    }
    v.append(&unlock);
    let sheet = gtk::Button::builder().label("📊 Score sheet").build();
    v.append(&sheet);
    let delete = gtk::Button::builder().label("Delete this player…").css_classes(["destructive-action"]).build();
    v.append(&delete);
    let pop = gtk::Popover::builder().child(&v).build();
    {
        let app = app.clone();
        let pop = pop.clone();
        sheet.connect_clicked(move |_| {
            pop.popdown();
            app.nav.push(&scores::page(&app, app.player.get()));
        });
    }
    {
        let app = app.clone();
        let pop = pop.clone();
        delete.connect_clicked(move |_| {
            pop.popdown();
            let name = app.store.borrow().profiles[app.player.get()].name.clone();
            let d = adw::AlertDialog::new(Some(&format!("Delete {name}?")), Some("All of their stars will be gone. This can't be undone."));
            d.add_responses(&[("cancel", "Keep"), ("delete", "Delete")]);
            d.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
            d.set_close_response("cancel");
            let app2 = app.clone();
            d.connect_response(None, move |_, r| {
                if r == "delete" {
                    {
                        let mut s = app2.store.borrow_mut();
                        let gone = s.profiles.remove(app2.player.get());
                        history::forget(s.dir(), gone.id);
                        s.save();
                    }
                    app2.player.set(0);
                    app2.nav.pop();
                }
            });
            d.present(Some(&app.win));
        });
    }
    gtk::MenuButton::builder().icon_name("open-menu-symbolic").popover(&pop).tooltip_text("For grown-ups").build()
}

fn main() -> glib::ExitCode {
    let application = adw::Application::builder().application_id(APP_ID).build();
    application.connect_startup(|_| {
        adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceLight);
        let css = gtk::CssProvider::new();
        css.load_from_string(CSS);
        gtk::style_context_add_provider_for_display(&gdk::Display::default().expect("display"), &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    });
    application.connect_activate(|application| {
        if let Some(w) = application.active_window() {
            w.present();
            return;
        }
        let nav = adw::NavigationView::new();
        let win = adw::ApplicationWindow::builder().application(application).title("Bilang").default_width(1180).default_height(800).content(&nav).build();
        let app = Rc::new(App { win: win.clone(), nav: nav.clone(), store: RefCell::new(Store::load()), player: Cell::new(0), sound: Sound::new() });
        nav.push(&players_page(&app));
        // BILANG_START=<player index>:<level index> jumps straight into a level, for testing.
        if let Some((p, l)) = std::env::var("BILANG_START").ok().and_then(|s| {
            let (p, l) = s.split_once(':')?;
            Some((p.parse::<usize>().ok()?, l.parse::<usize>().ok()?))
        }) && p < app.store.borrow().profiles.len()
        {
            app.player.set(p);
            nav.push(&map_page(&app));
            nav.push(&play::page(&app, l));
        }
        // BILANG_SHEET=<player index> opens that child's score sheet, for testing.
        if let Some(p) = std::env::var("BILANG_SHEET").ok().and_then(|s| s.parse::<usize>().ok())
            && p < app.store.borrow().profiles.len()
        {
            nav.push(&scores::page(&app, p));
        }
        win.present();
    });
    application.run()
}
