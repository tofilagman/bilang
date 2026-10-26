# Bilang

A math adventure for kids on the desktop (Rust + GTK4/libadwaita). Kiko the owl
guides players across six worlds, from counting up to algebra:

| World | Grades | Levels |
|---|---|---|
| 🌻 Number Garden | 1–2 | adding to 10, make a ten, taking away, mystery number, 2-digit + and − |
| 🐒 Times Jungle | 3 | times tables, division facts, 2-digit ×, remainders |
| 🏙️ Column City | 2–4 | stacked ("two-layer") +, −, × and long division, in tens, hundreds and thousands |
| 🏰 MDAS Castle | 4–5 | MDAS, parentheses, story problems, area & perimeter |
| 🌊 Fraction Falls | 5–6 | simplify, + − × ÷ fractions, decimals, percent |
| 🌋 Integer Volcano | 6–7 | negative numbers, exponents & roots, PEMDAS |
| 🏔️ Algebra Peak | 7–10 | equations, plug-in, proportions, sales %, Pythagoras, slope, systems, quadratics |

Before each level the kid picks how many questions: 10 (Quick), 20 (Practice),
50 (Challenge) or 100 (Marathon). Kiko suggests one from their stars on that level.
In runs longer than 10, "Stop here" appears after 10 questions. Finishing 50 or 100
with 3 stars earns a 🥈 or 🏆 shown on the map.

**📖 Show me how.** The first time a kid opens a level, Kiko teaches it before any
practice, the way a parent or teacher would. One example is broken into small
questions ("Look at the ones column: 9 and 9. What is 9 + 9?"), each followed by what
to write and where ("18 is 1 ten and 8 ones… write the 8 under the ones… carry the 1").
In stacked problems the column being discussed lights up and only the right box takes
the digit. Order-of-operations lessons shrink the expression on the board one step at
a time. Two misses on a step and Kiko just says the answer and moves on. The lesson
ends in a 10-question practice round, and "📖 Show me how" replays it any time.

Column City problems are written the way they're taught on paper. Kids fill in the
answer box by box, from the ones place (or left to right above the long-division
bracket), and can jot carries in the small boxes. Checking turns each digit green or red.

Questions are freshly generated every time. The **💡 Help** side panel shows the
level's lesson, plus hints that go from a nudge to a full worked solution using that
problem's own numbers (order-of-operations problems are solved step by step). A
wrong answer reveals the next hint by itself. "Almost" answers (6/8 instead of 3/4,
x and y swapped, a missing remainder) get a nudge instead of a miss.

Stars: 3 for 90%+ right on the first try (one hint allowed), 2 for 60%+, 1 for
finishing. Finishing a level unlocks the next one. Grown-ups can unlock everything
from the ☰ menu on the map.

## Run

```bash
brew install gtk4 libadwaita
cargo run --release
```

Players are saved in the app data folder (`~/Library/Application Support/io.r3al.Bilang/players.json`
on a Mac). Test hooks: `BILANG_DATA=<dir>` uses another folder, `BILANG_START=<player>:<level>`
opens a level directly, and `BILANG_DEBUG=1` logs each answer to stderr.

`cargo test` checks every level's generator 300 times and walks every stacked lesson to make sure it writes the right answer. `cargo test samples -- --ignored --nocapture`
prints one problem with its hints from every level, for proofreading; `cargo test lesson_samples -- --ignored --nocapture` does the same for lessons. With `BILANG_DEBUG=1`, lessons log `learn: ask/write/next` for each step.
