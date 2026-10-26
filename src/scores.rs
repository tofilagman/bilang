//! The grown-ups' score sheet: a child's progress at a glance, the week's
//! practice, what needs more work, every level, and each session with the
//! questions the child needed help on and what they typed.

use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use crate::{
    App, clear,
    history::{self, Run},
    label, levels,
    store::Profile,
    star_text,
};

/// Below this first-try percent (over recent sessions) a level is flagged.
const WEAK: u32 = 70;
const RECENT_SESSIONS: usize = 30;

pub fn page(app: &Rc<App>, player: usize) -> adw::NavigationPage {
    let names: Vec<String> = app.store.borrow().profiles.iter().map(|p| format!("{} {}", p.avatar, p.name)).collect();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    let pick = gtk::DropDown::from_strings(&names);
    pick.set_selected(player as u32);
    pick.set_tooltip_text(Some("Whose score sheet"));
    let export = gtk::Button::builder().label("⬇️ Export CSV").tooltip_text("Save every session as a spreadsheet").build();

    let body = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(22).margin_top(20).margin_bottom(40).margin_start(20).margin_end(20).build();
    let clamp = adw::Clamp::builder().maximum_size(980).child(&body).build();
    let scroll = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).vexpand(true).child(&clamp).build();
    let header = adw::HeaderBar::new();
    header.pack_end(&export);
    header.pack_end(&pick);
    let tv = adw::ToolbarView::new();
    tv.add_top_bar(&header);
    tv.set_content(Some(&scroll));
    tv.add_css_class("bg");
    tv.add_css_class("sheet");
    let page = adw::NavigationPage::builder().child(&tv).title("📊 Score sheet").build();

    let show = {
        let app = app.clone();
        let body = body.clone();
        move |i: usize| {
            clear(&body);
            let (profile, dir) = {
                let s = app.store.borrow();
                let Some(p) = s.profiles.get(i) else { return };
                (p.clone(), s.dir().to_path_buf())
            };
            let runs = history::load(&dir, profile.id);
            render(&body, &profile, &runs);
        }
    };
    show(player);
    {
        let show = show.clone();
        pick.connect_selected_notify(move |d| show(d.selected() as usize));
    }
    {
        let app = app.clone();
        let pick = pick.clone();
        export.connect_clicked(move |_| export_csv(&app, pick.selected() as usize));
    }
    // Back from a level, the numbers may have changed.
    page.connect_showing(move |_| show(pick.selected() as usize));
    page
}

fn title_of(id: &str) -> (String, String) {
    levels::all()
        .find(|(_, l)| l.id == id)
        .map(|(w, l)| (levels::WORLDS[w].name.to_string(), l.title.to_string()))
        .unwrap_or_else(|| (String::new(), id.to_string()))
}

fn section(text: &str) -> gtk::Label {
    let l = label(text, "world-title");
    l.set_xalign(0.0);
    l
}

fn boxed() -> gtk::ListBox {
    gtk::ListBox::builder().selection_mode(gtk::SelectionMode::None).css_classes(["boxed-list"]).build()
}

/// adw rows treat titles as markup; ours are plain text.
fn row(title: &str, subtitle: &str) -> adw::ActionRow {
    let r = adw::ActionRow::builder().title(title).subtitle(subtitle).use_markup(false).build();
    r.set_subtitle_lines(3);
    r
}

fn stat(value: &str, caption: &str) -> gtk::Box {
    let b = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).css_classes(["stat"]).build();
    b.append(&label(value, "stat-num"));
    b.append(&label(caption, "stat-cap"));
    b
}

fn render(body: &gtk::Box, p: &Profile, runs: &[Run]) {
    let practice: Vec<&Run> = runs.iter().filter(|r| !r.lesson).collect();
    let now = glib::DateTime::now_local().map(|d| d.to_unix()).unwrap_or(0);

    // ── Who, and when they last played
    let top = gtk::Box::builder().spacing(14).build();
    top.append(&label(&p.avatar, "avatar"));
    let words = gtk::Box::builder().orientation(gtk::Orientation::Vertical).valign(gtk::Align::Center).build();
    let name = label(&format!("{}'s score sheet", p.name), "hello");
    name.set_xalign(0.0);
    let last = runs.iter().map(|r| r.at).max();
    let sub = label(
        &match last {
            Some(t) => format!("Last played {}", history::when(t, "%A, %B %e at %l:%M %p")),
            None if p.solved > 0 => "Sessions are recorded from now on. Earlier play shows up as stars only.".to_string(),
            None => "No practice yet. Sessions will show up here as they play.".to_string(),
        },
        "sub",
    );
    sub.set_xalign(0.0);
    sub.set_wrap(true);
    words.append(&name);
    words.append(&sub);
    top.append(&words);
    body.append(&top);

    // ── At a glance
    let total_levels = levels::all().count();
    let done = levels::all().filter(|(_, l)| p.stars(l.id) > 0).count();
    let (answered, first) = practice.iter().fold((0, 0), |(a, f), r| (a + r.answered, f + r.first_try));
    let seconds: u32 = runs.iter().map(|r| r.seconds).sum();
    let tiles = gtk::FlowBox::builder().selection_mode(gtk::SelectionMode::None).homogeneous(true).min_children_per_line(2).max_children_per_line(6).column_spacing(12).row_spacing(12).build();
    tiles.append(&stat(&format!("{} / {}", p.total_stars(), total_levels * 3), "⭐ stars"));
    tiles.append(&stat(&format!("{done} / {total_levels}"), "🏁 levels finished"));
    tiles.append(&stat(&p.solved.to_string(), "✏️ questions answered"));
    tiles.append(&stat(&if answered > 0 { format!("{}%", first * 100 / answered) } else { "—".into() }, "🎯 right on first try"));
    tiles.append(&stat(&history::duration(seconds), "⏱️ time practicing"));
    tiles.append(&stat(&p.best_streak.to_string(), "🔥 best streak"));
    body.append(&tiles);

    // ── The last 7 days
    let days = history::by_day(runs, now, 7);
    let week_q: u32 = days.iter().map(|d| d.1).sum();
    let week_s: u32 = days.iter().map(|d| d.2).sum();
    body.append(&section(&format!("📅 This week: {week_q} questions · {}", history::duration(week_s))));
    body.append(&week_chart(&days));

    // ── What to go over together
    let stats = history::per_level(runs);
    let mut weak: Vec<(String, u32)> = stats.iter().filter_map(|(id, s)| s.recent.filter(|&r| r < WEAK).map(|r| (id.clone(), r))).collect();
    weak.sort_by_key(|w| w.1);
    body.append(&section("💡 Worth practicing together"));
    if weak.is_empty() {
        let l = label(
            if practice.is_empty() { "Once they've played a few sessions, levels that need more work will show up here." } else { "Nothing worrying right now. Every recent session went well! 🌟" },
            "sub",
        );
        l.set_xalign(0.0);
        l.set_wrap(true);
        body.append(&l);
    } else {
        let list = boxed();
        for (id, pct) in weak {
            let (world, title) = title_of(&id);
            let r = row(&format!("{title} ({world})"), &format!("{pct}% right on the first try lately. Try \"📖 Show me how\" together, then a short round of 10."));
            r.add_suffix(&label("⚠️", ""));
            list.append(&r);
        }
        body.append(&list);
    }

    // ── Every level, by world
    body.append(&section("🗺️ Levels"));
    for world in levels::WORLDS {
        let played: Vec<_> = world.levels.iter().filter(|l| p.stars(l.id) > 0 || stats.contains_key(l.id)).collect();
        let done = world.levels.iter().filter(|l| p.stars(l.id) > 0).count();
        if played.is_empty() {
            continue;
        }
        let head = label(&format!("{} {} · {done} of {} done", world.emoji, world.name, world.levels.len()), "side-title");
        head.set_xalign(0.0);
        body.append(&head);
        let list = boxed();
        for l in played {
            let subtitle = match stats.get(l.id) {
                Some(s) if s.runs > 0 => format!(
                    "Played {}× · {} right on the first try · {} · last {}",
                    s.runs,
                    s.percent().map(|x| format!("{x}%")).unwrap_or("—".into()),
                    history::duration(s.seconds),
                    history::when(s.last_at, "%b %e")
                ),
                Some(s) => format!("Did the lesson · last {}", history::when(s.last_at, "%b %e")),
                None => "Finished before the score sheet started recording".to_string(),
            };
            let r = row(l.title, &subtitle);
            let medal = match p.medal(l.id) {
                100.. => " 🏆",
                50.. => " 🥈",
                _ => "",
            };
            if p.stars(l.id) > 0 {
                r.add_suffix(&label(&format!("{}{medal}", star_text(p.stars(l.id))), "stars"));
            } else {
                r.add_suffix(&label("In progress", "stat-cap"));
            }
            list.append(&r);
        }
        body.append(&list);
    }
    if p.total_stars() == 0 && stats.is_empty() {
        let l = label("No levels started yet.", "sub");
        l.set_xalign(0.0);
        body.append(&l);
    }

    // ── Session by session
    body.append(&section("🕑 Recent sessions"));
    if runs.is_empty() {
        let l = label("Nothing recorded yet.", "sub");
        l.set_xalign(0.0);
        body.append(&l);
        return;
    }
    let mut newest: Vec<&Run> = runs.iter().collect();
    newest.sort_by_key(|r| std::cmp::Reverse(r.at));
    let list = boxed();
    for r in newest.into_iter().take(RECENT_SESSIONS) {
        let (_, title) = title_of(&r.level);
        let when = history::when(r.at, "%b %e, %l:%M %p");
        if r.lesson {
            list.append(&row(&format!("{when} · {title}"), &format!("📖 Learned it with Kiko (Show me how) · {}", history::duration(r.seconds))));
            continue;
        }
        let mut sub = format!("{} of {} right on the first try", r.first_try, r.answered);
        if r.finished {
            sub.push_str(&format!(" · {}", star_text(r.stars)));
        } else {
            sub.push_str(&format!(" · stopped at {} of {}", r.answered, r.total));
        }
        sub.push_str(&format!(" · {} · {} hint{}", history::duration(r.seconds), r.hints, if r.hints == 1 { "" } else { "s" }));
        let e = adw::ExpanderRow::builder().title(format!("{when} · {title}")).subtitle(sub).use_markup(false).build();
        if r.missed.is_empty() {
            e.add_row(&row("Every question right on the first try! 🎉", ""));
        } else {
            for m in &r.missed {
                let tried = if m.tries.is_empty() { "used hints".to_string() } else { format!("first answered {}", m.tries.join(", ")) };
                let hints = match m.hints {
                    0 => String::new(),
                    1 => " · 1 hint".into(),
                    n => format!(" · {n} hints"),
                };
                e.add_row(&row(&m.question, &format!("Right answer {} · {tried}{hints}", m.answer)));
            }
        }
        list.append(&e);
    }
    body.append(&list);
    if runs.len() > RECENT_SESSIONS {
        let l = label(&format!("Showing the latest {RECENT_SESSIONS} of {} sessions. Export CSV has them all.", runs.len()), "tip");
        l.set_xalign(0.0);
        body.append(&l);
    }
}

/// Questions per day as bars on one baseline; hover a bar for its numbers.
fn week_chart(days: &[(i64, u32, u32)]) -> gtk::Box {
    const H: f64 = 120.0;
    let max = days.iter().map(|d| d.1).max().unwrap_or(0).max(1) as f64;
    let chart = gtk::Box::builder().spacing(14).halign(gtk::Align::Start).css_classes(["week"]).build();
    let last = days.len().saturating_sub(1);
    for (i, &(start, q, s)) in days.iter().enumerate() {
        let col = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(6).valign(gtk::Align::End).build();
        // A spacer on top pushes the bar down onto the shared baseline.
        let area = gtk::Box::builder().orientation(gtk::Orientation::Vertical).height_request(H as i32).build();
        area.append(&gtk::Box::builder().vexpand(true).build());
        let bar = gtk::Box::builder().width_request(44).height_request(((q as f64 / max) * H).round().max(if q > 0 { 6.0 } else { 2.0 }) as i32).build();
        bar.add_css_class(if q > 0 { "bar" } else { "bar-empty" });
        let tip = format!("{}: {q} questions, {}", history::when(start, "%A %b %e"), history::duration(s));
        bar.set_tooltip_text(Some(&tip));
        col.set_tooltip_text(Some(&tip));
        area.append(&bar);
        col.append(&area);
        let day = label(&if i == last { "Today".to_string() } else { history::when(start, "%a") }, "stat-cap");
        if i == last {
            day.add_css_class("today");
        }
        col.append(&day);
        chart.append(&col);
    }
    chart
}

fn export_csv(app: &Rc<App>, player: usize) {
    let (runs, name) = {
        let s = app.store.borrow();
        let Some(p) = s.profiles.get(player) else { return };
        (history::load(s.dir(), p.id), p.name.clone())
    };
    let csv = history::to_csv(&runs, title_of);
    let dialog = gtk::FileDialog::builder().title("Save score sheet").initial_name(format!("{name} - Bilang score sheet.csv")).build();
    let win = app.win.clone();
    dialog.save(Some(&win), gio::Cancellable::NONE, move |res| {
        let Ok(file) = res else { return };
        let Some(path) = file.path() else { return };
        if let Err(e) = std::fs::write(&path, csv.as_bytes()) {
            eprintln!("bilang: export failed: {e}");
        }
    });
}
