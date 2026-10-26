//! Every practice run and lesson, kept per player for the grown-ups' score
//! sheet: when, how it went, how long, and the questions that needed help
//! (with what the kid typed), so a parent can see what to go over together.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

use chrono::{Datelike, Local, TimeZone};

/// Oldest runs are dropped past this, so the file stays small.
const KEEP: usize = 1000;
/// Missed questions kept per run.
const KEEP_MISSED: usize = 30;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Missed {
    pub question: String,
    pub answer: String,
    /// What the kid typed before getting it (wrong or "almost" tries).
    #[serde(default)]
    pub tries: Vec<String>,
    #[serde(default)]
    pub hints: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Run {
    pub level: String,
    /// Start time, unix seconds.
    pub at: i64,
    pub seconds: u32,
    /// A "Show me how" lesson rather than practice.
    #[serde(default)]
    pub lesson: bool,
    #[serde(default)]
    pub total: u32,
    #[serde(default)]
    pub answered: u32,
    /// Right on the first try with at most one hint.
    #[serde(default)]
    pub first_try: u32,
    #[serde(default)]
    pub hints: u32,
    #[serde(default)]
    pub wrong: u32,
    #[serde(default)]
    pub stars: u8,
    /// Played all the way through (not stopped early or left).
    #[serde(default)]
    pub finished: bool,
    #[serde(default)]
    pub missed: Vec<Missed>,
}

impl Run {
    pub fn percent(&self) -> Option<u32> {
        (self.answered > 0).then(|| self.first_try * 100 / self.answered)
    }
}

pub fn path(dir: &Path, player_id: u64) -> std::path::PathBuf {
    dir.join("history").join(format!("{player_id}.json"))
}

/// Oldest first.
pub fn load(dir: &Path, player_id: u64) -> Vec<Run> {
    std::fs::read(path(dir, player_id)).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}

pub fn append(dir: &Path, player_id: u64, mut run: Run) {
    run.missed.truncate(KEEP_MISSED);
    let mut runs = load(dir, player_id);
    runs.push(run);
    if runs.len() > KEEP {
        runs.drain(..runs.len() - KEEP);
    }
    let p = path(dir, player_id);
    if let Some(d) = p.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let tmp = p.with_extension("json.tmp");
    if serde_json::to_vec(&runs).ok().is_some_and(|b| std::fs::write(&tmp, b).is_ok()) {
        let _ = std::fs::rename(&tmp, &p);
    } else {
        eprintln!("bilang: could not save {}", p.display());
    }
}

pub fn forget(dir: &Path, player_id: u64) {
    let _ = std::fs::remove_file(path(dir, player_id));
}

#[derive(Default, Debug, PartialEq)]
pub struct LevelStats {
    pub runs: u32,
    pub answered: u32,
    pub first_try: u32,
    pub seconds: u32,
    pub last_at: i64,
    /// First-try percent over the last three practice runs of 5+ questions.
    pub recent: Option<u32>,
}

impl LevelStats {
    pub fn percent(&self) -> Option<u32> {
        (self.answered > 0).then(|| self.first_try * 100 / self.answered)
    }
}

pub fn per_level(runs: &[Run]) -> BTreeMap<String, LevelStats> {
    let mut out: BTreeMap<String, LevelStats> = BTreeMap::new();
    let mut recent: BTreeMap<&str, Vec<&Run>> = BTreeMap::new();
    for r in runs {
        let s = out.entry(r.level.clone()).or_default();
        s.seconds += r.seconds;
        s.last_at = s.last_at.max(r.at);
        if r.lesson {
            continue;
        }
        s.runs += 1;
        s.answered += r.answered;
        s.first_try += r.first_try;
        if r.answered >= 5 {
            recent.entry(&r.level).or_default().push(r);
        }
    }
    for (level, rs) in recent {
        let last: Vec<_> = rs.iter().rev().take(3).collect();
        let (a, f) = last.iter().fold((0, 0), |(a, f), r| (a + r.answered, f + r.first_try));
        if let Some(s) = out.get_mut(level) {
            s.recent = (a > 0).then(|| f * 100 / a);
        }
    }
    out
}

/// The local calendar day of a time, as (year, month, day).
pub fn day_of(at: i64) -> Option<(i32, i32, i32)> {
    let d = Local.timestamp_opt(at, 0).single()?;
    Some((d.year(), d.month() as i32, d.day() as i32))
}

/// Questions answered and seconds practiced on each of the last `days` local
/// days, oldest first, with each day's start time.
pub fn by_day(runs: &[Run], now: i64, days: u32) -> Vec<(i64, u32, u32)> {
    let Some(today) = Local.timestamp_opt(now, 0).single() else { return Vec::new() };
    (0..days as i64)
        .rev()
        .filter_map(|back| {
            let day = today.date_naive() - chrono::Days::new(back as u64);
            let start = Local.from_local_datetime(&day.and_hms_opt(0, 0, 0)?).earliest()?;
            let key = (day.year(), day.month() as i32, day.day() as i32);
            let (q, s) = runs.iter().filter(|r| day_of(r.at) == Some(key)).fold((0, 0), |(q, s), r| (q + r.answered, s + r.seconds));
            Some((start.timestamp(), q, s))
        })
        .collect()
}

/// A local date/time in strftime style. %e and %l pad with spaces, so whitespace is collapsed.
pub fn when(at: i64, fmt: &str) -> String {
    Local.timestamp_opt(at, 0).single().map(|d| d.format(fmt).to_string().split_whitespace().collect::<Vec<_>>().join(" ")).unwrap_or_default()
}

pub fn duration(seconds: u32) -> String {
    match seconds {
        0..60 => format!("{seconds} sec"),
        60..3600 => format!("{} min", seconds / 60),
        _ => format!("{} h {} min", seconds / 3600, seconds % 3600 / 60),
    }
}

fn csv(field: &str) -> String {
    if field.contains([',', '"', '\n']) { format!("\"{}\"", field.replace('"', "\"\"")) } else { field.to_string() }
}

/// One row per session, for a spreadsheet or a teacher.
pub fn to_csv(runs: &[Run], title_of: impl Fn(&str) -> (String, String)) -> String {
    let mut out = String::from("Date,Time,World,Level,Kind,Answered,Questions,First try,First try %,Stars,Minutes,Hints,Wrong tries,Finished,Needed help with\n");
    for r in runs {
        let (world, level) = title_of(&r.level);
        let help: Vec<String> = r.missed.iter().map(|m| format!("{} (answer {}; tried {})", m.question, m.answer, if m.tries.is_empty() { "-".into() } else { m.tries.join(" / ") })).collect();
        let row = [
            when(r.at, "%Y-%m-%d"),
            when(r.at, "%H:%M"),
            world,
            level,
            if r.lesson { "Lesson".into() } else { "Practice".into() },
            r.answered.to_string(),
            r.total.to_string(),
            r.first_try.to_string(),
            r.percent().map(|p| p.to_string()).unwrap_or_default(),
            r.stars.to_string(),
            format!("{:.1}", r.seconds as f64 / 60.0),
            r.hints.to_string(),
            r.wrong.to_string(),
            if r.finished { "yes".into() } else { "no".into() },
            help.join(" | "),
        ];
        out.push_str(&row.iter().map(|f| csv(f)).collect::<Vec<_>>().join(","));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(level: &str, at: i64, answered: u32, first: u32) -> Run {
        Run { level: level.into(), at, seconds: 60, lesson: false, total: answered, answered, first_try: first, hints: 0, wrong: 0, stars: 3, finished: true, missed: vec![] }
    }

    #[test]
    fn level_stats_use_the_last_three_runs() {
        let runs = vec![run("a", 1, 10, 2), run("a", 2, 10, 10), run("a", 3, 10, 9), run("a", 4, 10, 8), run("b", 5, 3, 3)];
        let s = per_level(&runs);
        assert_eq!(s["a"].runs, 4);
        assert_eq!(s["a"].percent(), Some(72));
        assert_eq!(s["a"].recent, Some(90));
        assert_eq!(s["b"].recent, None); // too short to judge
    }

    #[test]
    fn saves_and_loads() {
        let dir = std::env::temp_dir().join(format!("bilang-test-{}", std::process::id()));
        let mut r = run("a", 10, 5, 4);
        r.missed.push(Missed { question: "7 + 8".into(), answer: "15".into(), tries: vec!["14".into()], hints: 1 });
        append(&dir, 1, r.clone());
        append(&dir, 1, run("b", 20, 5, 5));
        let back = load(&dir, 1);
        assert_eq!(back.len(), 2);
        assert_eq!(back[0], r);
        let csv = to_csv(&back, |l| ("W".into(), l.to_uppercase()));
        assert!(csv.lines().nth(1).unwrap().contains("7 + 8 (answer 15; tried 14)"));
        forget(&dir, 1);
        assert!(load(&dir, 1).is_empty());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn days_add_up() {
        let now = Local::now().timestamp();
        let days = by_day(&[run("a", now, 7, 7), run("a", now - 1, 3, 3), run("a", now - 9 * 86400, 50, 50)], now, 7);
        assert_eq!(days.len(), 7);
        assert_eq!(days[6].1, 10);
        assert_eq!(days.iter().map(|d| d.1).sum::<u32>(), 10);
    }
}
