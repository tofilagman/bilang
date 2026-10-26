//! What the tablet app asks of Bilang's engine (core/): the map, players and
//! their progress, problems with hints, answer checking, lessons, run
//! history and the scratchpad calculator. Plain data in and out, mirrored
//! into Dart by flutter_rust_bridge.

use std::{path::PathBuf, sync::Mutex};

use bilang_core::{
    answer::{Answer, Verdict},
    calc::Pad,
    figure::{self, Figure},
    history::{self, Missed, Run},
    levels::{self, ColOp, Problem},
    store::Store,
    teach::{self, Target},
};
use flutter_rust_bridge::frb;
use rand::{SeedableRng, rngs::StdRng};

static STORE: Mutex<Option<Store>> = Mutex::new(None);

fn with_store<T>(f: impl FnOnce(&mut Store) -> T) -> T {
    let mut guard = STORE.lock().unwrap();
    f(guard.get_or_insert_with(|| Store::load_dir(&PathBuf::from("."))))
}

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// Load players from the app's data folder. Call once at startup.
#[frb(sync)]
pub fn open_store(dir: String) {
    *STORE.lock().unwrap() = Some(Store::load_dir(&PathBuf::from(dir)));
}

// ───────────────────────── The map ─────────────────────────

pub struct LevelInfo {
    pub id: String,
    pub title: String,
    pub lesson: String,
    /// Position in play order, across all worlds.
    pub flat: u32,
    pub world: u32,
}

pub struct WorldInfo {
    pub name: String,
    pub emoji: String,
    pub grades: String,
    pub levels: Vec<LevelInfo>,
}

#[frb(sync)]
pub fn worlds() -> Vec<WorldInfo> {
    let mut flat = 0;
    levels::WORLDS
        .iter()
        .enumerate()
        .map(|(w, world)| WorldInfo {
            name: world.name.into(),
            emoji: world.emoji.into(),
            grades: world.grades.into(),
            levels: world
                .levels
                .iter()
                .map(|l| {
                    flat += 1;
                    LevelInfo { id: l.id.into(), title: l.title.into(), lesson: l.lesson.into(), flat: flat - 1, world: w as u32 }
                })
                .collect(),
        })
        .collect()
}

// ───────────────────────── Players ─────────────────────────

pub struct LevelState {
    pub id: String,
    pub stars: u8,
    pub medal: u32,
    pub unlocked: bool,
    pub learned: bool,
}

pub struct Player {
    pub index: u32,
    pub id: u64,
    pub name: String,
    pub avatar: String,
    pub total_stars: u32,
    pub solved: u32,
    pub best_streak: u32,
    pub unlock_all: bool,
    /// "auto", "always" or "never"
    pub scratchpad: String,
    pub levels: Vec<LevelState>,
}

#[frb(sync)]
pub fn players() -> Vec<Player> {
    with_store(|s| {
        s.profiles
            .iter()
            .enumerate()
            .map(|(i, p)| Player {
                index: i as u32,
                id: p.id,
                name: p.name.clone(),
                avatar: p.avatar.clone(),
                total_stars: p.total_stars(),
                solved: p.solved,
                best_streak: p.best_streak,
                unlock_all: p.unlock_all,
                scratchpad: p.scratchpad.clone(),
                levels: levels::all()
                    .enumerate()
                    .map(|(f, (_, l))| LevelState { id: l.id.into(), stars: p.stars(l.id), medal: p.medal(l.id), unlocked: p.unlocked(f), learned: p.learned.contains(l.id) })
                    .collect(),
            })
            .collect()
    })
}

#[frb(sync)]
pub fn add_player(name: String, avatar: String) -> u32 {
    with_store(|s| s.add(name, avatar) as u32)
}

#[frb(sync)]
pub fn delete_player(index: u32) {
    with_store(|s| {
        if (index as usize) < s.profiles.len() {
            let gone = s.profiles.remove(index as usize);
            history::forget(s.dir(), gone.id);
            s.save();
        }
    })
}

fn edit_player(index: u32, f: impl FnOnce(&mut bilang_core::store::Profile)) {
    with_store(|s| {
        if let Some(p) = s.profiles.get_mut(index as usize) {
            f(p);
            s.save();
        }
    })
}

#[frb(sync)]
pub fn set_unlock_all(index: u32, on: bool) {
    edit_player(index, |p| p.unlock_all = on);
}

#[frb(sync)]
pub fn set_scratchpad(index: u32, mode: String) {
    edit_player(index, |p| p.scratchpad = mode);
}

#[frb(sync)]
pub fn mark_learned(index: u32, level_id: String) {
    edit_player(index, |p| {
        p.learned.insert(level_id);
    });
}

#[frb(sync)]
pub fn sound_on() -> bool {
    with_store(|s| s.sound)
}

#[frb(sync)]
pub fn set_sound(on: bool) {
    with_store(|s| {
        s.sound = on;
        s.save();
    })
}

pub struct Suggestion {
    pub count: u32,
    pub why: String,
}

/// Kiko's pick for how many questions, from how this child did on the level.
#[frb(sync)]
pub fn suggested_count(index: u32, level_id: String) -> Suggestion {
    with_store(|s| {
        let (count, why) = s.profiles.get(index as usize).map(|p| p.suggested_count(&level_id)).unwrap_or((10, ""));
        Suggestion { count: count as u32, why: why.into() }
    })
}

#[frb(sync)]
pub fn scratchpad_in(index: u32, world: u32) -> bool {
    with_store(|s| s.profiles.get(index as usize).is_some_and(|p| p.scratchpad_in(world as usize)))
}

// ───────────────────────── Problems ─────────────────────────

pub enum AnswerDto {
    Int { n: i64 },
    Frac { n: i64, d: i64 },
    Dec { hundredths: i64 },
    Remainder { q: i64, r: i64 },
    Roots { a: i64, b: i64 },
    Pair { x: i64, y: i64 },
    Ineq { op: String, n: i64 },
}

fn to_dto(a: &Answer) -> AnswerDto {
    match *a {
        Answer::Int(n) => AnswerDto::Int { n },
        Answer::Frac(n, d) => AnswerDto::Frac { n, d },
        Answer::Dec(h) => AnswerDto::Dec { hundredths: h },
        Answer::Remainder(q, r) => AnswerDto::Remainder { q, r },
        Answer::Roots(a, b) => AnswerDto::Roots { a, b },
        Answer::Pair(x, y) => AnswerDto::Pair { x, y },
        Answer::Ineq(op, n) => AnswerDto::Ineq { op: op.to_string(), n },
    }
}

fn from_dto(a: &AnswerDto) -> Answer {
    match a {
        AnswerDto::Int { n } => Answer::Int(*n),
        AnswerDto::Frac { n, d } => Answer::Frac(*n, *d),
        AnswerDto::Dec { hundredths } => Answer::Dec(*hundredths),
        AnswerDto::Remainder { q, r } => Answer::Remainder(*q, *r),
        AnswerDto::Roots { a, b } => Answer::Roots(*a, *b),
        AnswerDto::Pair { x, y } => Answer::Pair(*x, *y),
        AnswerDto::Ineq { op, n } => Answer::Ineq(op.chars().next().unwrap_or('<'), *n),
    }
}

/// A stacked ("two-layer") problem: + − × with top over bottom, or ÷ with
/// `top` inside the long-division bracket and `bottom` outside.
pub struct ColumnDto {
    pub op: String,
    pub top: i64,
    pub bottom: i64,
}

pub enum FigureDto {
    Right { adj: f64, opp: f64, angle: String, adj_label: String, opp_label: String, hyp_label: String },
    Sas { a: f64, b: f64, deg: f64, a_label: String, b_label: String, angle: String, third: String },
    Unit { deg: f64, label: String },
}

fn fig_dto(f: &Figure) -> FigureDto {
    match f.clone() {
        Figure::Right { adj, opp, angle, adj_label, opp_label, hyp_label } => FigureDto::Right { adj, opp, angle, adj_label, opp_label, hyp_label },
        Figure::Sas { a, b, deg, a_label, b_label, angle, third } => FigureDto::Sas { a, b, deg, a_label, b_label, angle, third },
        Figure::Unit { deg, label } => FigureDto::Unit { deg, label },
    }
}

pub struct GivenValue {
    pub name: String,
    pub value: String,
}

pub struct ProblemDto {
    /// May hold `$…$` formulas (LaTeX-style).
    pub prompt: String,
    pub visual: Option<String>,
    pub answer: AnswerDto,
    /// The answer as text ("3/4", "x < −4").
    pub answer_shown: String,
    pub format_tip: String,
    /// Extra keypad keys this answer needs ( − / . , r < > = ).
    pub keys: Vec<String>,
    pub hints: Vec<String>,
    pub column: Option<ColumnDto>,
    pub figure: Option<FigureDto>,
    pub given: Vec<GivenValue>,
    /// The question on one plain line, for the score sheet.
    pub describe: String,
}

fn problem_dto(p: &Problem) -> ProblemDto {
    ProblemDto {
        prompt: p.prompt.clone(),
        visual: p.visual.clone(),
        answer: to_dto(&p.answer),
        answer_shown: p.answer.show(),
        format_tip: p.answer.format_tip().into(),
        keys: p.answer.keys().iter().map(|k| k.to_string()).collect(),
        hints: p.hints.clone(),
        column: p.column.map(|c| ColumnDto {
            op: match c.op {
                ColOp::Add => "+",
                ColOp::Sub => "−",
                ColOp::Mul => "×",
                ColOp::Div => "÷",
            }
            .into(),
            top: c.top,
            bottom: c.bottom,
        }),
        figure: p.figure.as_ref().map(fig_dto),
        given: p.given.iter().map(|(n, v)| GivenValue { name: n.clone(), value: v.clone() }).collect(),
        describe: p.describe(),
    }
}

fn generate(level_id: &str, seed: u64) -> Option<Problem> {
    let (_, level) = levels::all().find(|(_, l)| l.id == level_id)?;
    Some((level.make)(&mut StdRng::seed_from_u64(seed)))
}

/// A fresh problem for a level; the same seed always gives the same problem.
#[frb(sync)]
pub fn make_problem(level_id: String, seed: u64) -> Option<ProblemDto> {
    generate(&level_id, seed).as_ref().map(problem_dto)
}

/// The diagram beside a level's "How it works" text, if it has one.
#[frb(sync)]
pub fn lesson_figure(level_id: String) -> Option<FigureDto> {
    figure::for_lesson(&level_id).as_ref().map(fig_dto)
}

pub enum VerdictDto {
    Right,
    Almost { message: String },
    Wrong,
    Unreadable { message: String },
}

#[frb(sync)]
pub fn check(answer: AnswerDto, input: String) -> VerdictDto {
    match from_dto(&answer).check(&input) {
        Verdict::Right => VerdictDto::Right,
        Verdict::Almost(m) => VerdictDto::Almost { message: m },
        Verdict::Wrong => VerdictDto::Wrong,
        Verdict::Unreadable(m) => VerdictDto::Unreadable { message: m },
    }
}

// ───────────────────────── Lessons ("Show me how") ─────────────────────────

pub struct WriteDto {
    /// "answer" (a place, 0 = ones), "carry" (a place), "quotient" (a dividend digit from the left) or "remainder"
    pub kind: String,
    pub index: u32,
    pub digit: String,
}

pub struct StepDto {
    pub say: String,
    pub ask: Option<AnswerDto>,
    pub write: Option<WriteDto>,
    pub point: Option<u32>,
    pub board: Option<String>,
}

pub struct LessonDto {
    pub problem: ProblemDto,
    pub steps: Vec<StepDto>,
}

#[frb(sync)]
pub fn lesson(level_id: String, seed: u64) -> Option<LessonDto> {
    let p = generate(&level_id, seed)?;
    let steps = teach::lesson(&level_id, &p)
        .into_iter()
        .map(|s| StepDto {
            say: s.say,
            ask: s.ask.as_ref().map(to_dto),
            write: s.write.map(|(t, d)| {
                let (kind, index) = match t {
                    Target::Answer(i) => ("answer", i),
                    Target::Carry(i) => ("carry", i),
                    Target::Quotient(i) => ("quotient", i),
                    Target::Remainder => ("remainder", 0),
                };
                WriteDto { kind: kind.into(), index: index as u32, digit: d.to_string() }
            }),
            point: s.point.map(|p| p as u32),
            board: s.board,
        })
        .collect();
    Some(LessonDto { problem: problem_dto(&p), steps })
}

// ───────────────────────── Finishing a run, and history ─────────────────────────

pub struct Outcome {
    pub stars: u8,
    pub new_medal: Option<u32>,
}

/// Score a finished or stopped practice run and save the child's best.
#[frb(sync)]
pub fn finish_run(index: u32, level_id: String, answered: u32, total: u32, first_try: u32, best_streak: u32) -> Outcome {
    with_store(|s| {
        let out = s.profiles.get_mut(index as usize).map(|p| p.apply_run(&level_id, answered, total, first_try, best_streak)).unwrap_or((1, None));
        s.save();
        Outcome { stars: out.0, new_medal: out.1 }
    })
}

pub struct MissedDto {
    pub question: String,
    pub answer: String,
    pub tries: Vec<String>,
    pub hints: u32,
}

pub struct RunDto {
    pub level: String,
    pub at: i64,
    pub seconds: u32,
    pub lesson: bool,
    pub total: u32,
    pub answered: u32,
    pub first_try: u32,
    pub hints: u32,
    pub wrong: u32,
    pub stars: u8,
    pub finished: bool,
    pub missed: Vec<MissedDto>,
}

/// Add a session (practice or lesson) to the child's score sheet.
#[frb(sync)]
pub fn record_run(index: u32, run: RunDto) {
    with_store(|s| {
        let Some(p) = s.profiles.get(index as usize) else { return };
        let r = Run {
            level: run.level,
            at: run.at,
            seconds: run.seconds,
            lesson: run.lesson,
            total: run.total,
            answered: run.answered,
            first_try: run.first_try,
            hints: run.hints,
            wrong: run.wrong,
            stars: run.stars,
            finished: run.finished,
            missed: run.missed.into_iter().map(|m| Missed { question: m.question, answer: m.answer, tries: m.tries, hints: m.hints }).collect(),
        };
        history::append(s.dir(), p.id, r);
    })
}

fn runs_of(index: u32) -> Vec<Run> {
    with_store(|s| s.profiles.get(index as usize).map(|p| history::load(s.dir(), p.id)).unwrap_or_default())
}

/// The child's sessions, oldest first.
#[frb(sync)]
pub fn history_of(index: u32) -> Vec<RunDto> {
    runs_of(index)
        .into_iter()
        .map(|r| RunDto {
            level: r.level,
            at: r.at,
            seconds: r.seconds,
            lesson: r.lesson,
            total: r.total,
            answered: r.answered,
            first_try: r.first_try,
            hints: r.hints,
            wrong: r.wrong,
            stars: r.stars,
            finished: r.finished,
            missed: r.missed.into_iter().map(|m| MissedDto { question: m.question, answer: m.answer, tries: m.tries, hints: m.hints }).collect(),
        })
        .collect()
}

pub struct LevelStatDto {
    pub id: String,
    pub runs: u32,
    pub answered: u32,
    pub first_try: u32,
    pub seconds: u32,
    pub last_at: i64,
    /// First-try percent over the last three practice runs of 5+ questions.
    pub recent: Option<u32>,
}

#[frb(sync)]
pub fn level_stats(index: u32) -> Vec<LevelStatDto> {
    history::per_level(&runs_of(index))
        .into_iter()
        .map(|(id, s)| LevelStatDto { id, runs: s.runs, answered: s.answered, first_try: s.first_try, seconds: s.seconds, last_at: s.last_at, recent: s.recent })
        .collect()
}

pub struct DayDto {
    pub start: i64,
    pub questions: u32,
    pub seconds: u32,
}

#[frb(sync)]
pub fn week(index: u32, now: i64) -> Vec<DayDto> {
    history::by_day(&runs_of(index), now, 7).into_iter().map(|(start, questions, seconds)| DayDto { start, questions, seconds }).collect()
}

/// A local date/time in strftime style.
#[frb(sync)]
pub fn when(at: i64, fmt: String) -> String {
    history::when(at, &fmt)
}

#[frb(sync)]
pub fn duration(seconds: u32) -> String {
    history::duration(seconds)
}

/// Every session as a spreadsheet.
#[frb(sync)]
pub fn history_csv(index: u32) -> String {
    history::to_csv(&runs_of(index), |id| {
        levels::all()
            .find(|(_, l)| l.id == id)
            .map(|(w, l)| (levels::WORLDS[w].name.to_string(), l.title.to_string()))
            .unwrap_or_else(|| (String::new(), id.to_string()))
    })
}

// ───────────────────────── Scratchpad ─────────────────────────

pub struct LineDto {
    /// What was typed, as a `$…$` formula.
    pub shown: String,
    /// The answer as a `$…$` formula, and its decimal when it's a fraction.
    pub result: String,
    pub approx: Option<String>,
    /// The answer as plain text, for the answer box.
    pub plain: String,
}

/// The formula pad's memory: given values, the child's own, and `ans`.
#[frb(opaque)]
pub struct ScratchPad {
    pad: Pad,
}

impl ScratchPad {
    #[frb(sync)]
    pub fn new() -> ScratchPad {
        ScratchPad { pad: Pad::default() }
    }

    #[frb(sync)]
    pub fn set(&mut self, name: String, value: String) {
        self.pad.set(&name, &value);
    }

    /// Forget everything but the given values.
    #[frb(sync)]
    pub fn clear(&mut self, keep: Vec<String>) {
        self.pad.vars.retain(|k, _| keep.contains(k));
    }

    #[frb(sync)]
    pub fn run(&mut self, src: String) -> Result<LineDto, String> {
        self.pad.run(&src).map(|l| LineDto { shown: l.shown, result: l.result, approx: l.approx, plain: l.plain })
    }
}
