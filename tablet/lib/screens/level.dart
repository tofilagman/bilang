// One level, like the desktop's play.rs and learn.rs: a "Show me how"
// lesson the first time, then "how many questions?", the questions with
// Kiko, hints and the scratchpad beside them, and the results.

import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../sound.dart';
import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';
import '../widgets/column_pad.dart';
import '../widgets/confetti.dart';
import '../widgets/figure.dart';
import '../widgets/keypad.dart';
import '../widgets/math_text.dart';
import '../widgets/scratchpad.dart';

enum Stage { setup, learn, game, done }

const praise = ['Great job! ⭐', 'You got it! 🎉', 'Awesome! 🙌', 'Math wizard! 🪄', 'Galing! 👏', 'Correct! Keep going! 🚀', "Yes! That's it! 💯", 'Super! 😄'];
const oops = [
  "Not quite, but you're close. Try again! 💪",
  'Oops! Mistakes help our brains grow. Peek at the hint 💡',
  "Hmm, let's think again. The hint on the side can help!",
  'Almost! Read the hint and give it another go.',
  "That's okay! Let's figure it out together 🦉",
];
const nice = ['Yes! ', 'Great! ', 'Right! ', 'Perfect! ', 'Good job! ', "That's it! "];
const counts = [(10, 'Quick'), (20, 'Practice'), (50, 'Challenge 🥈'), (100, 'Marathon 🏆')];

int _now() => DateTime.now().millisecondsSinceEpoch ~/ 1000;

class LevelScreen extends StatefulWidget {
  final int player;
  final api.LevelInfo level;
  const LevelScreen({super.key, required this.player, required this.level});

  @override
  State<LevelScreen> createState() => _LevelScreenState();
}

class _LevelScreenState extends State<LevelScreen> {
  final rng = math.Random();
  final entry = TextEditingController();
  final confetti = ConfettiController();
  final scratch = ScratchController();
  late final bool scratchOk = api.scratchpadIn(index: widget.player, world: widget.level.world);
  late final api.FigureDto? lessonFigure = api.lessonFigure(levelId: widget.level.id);
  late final api.LevelInfo? nextLevel = api.worlds().expand((w) => w.levels).where((l) => l.flat == widget.level.flat + 1).firstOrNull;

  Stage stage = Stage.setup;
  bool showScratch = false;

  // ── setup
  int chosen = 10;
  String suggestWhy = '';

  // ── game
  int total = 10, q = 0, clean = 0, hintsShown = 0, streak = 0, bestStreak = 0;
  bool missed = false, solved = false;
  late api.ProblemDto problem;
  ColumnPadController? pad;
  String face = '🦉', say = '';
  Color cardColor = Colors.white;

  // ── for the score sheet
  int? runAt;
  final clock = Stopwatch();
  int runHints = 0, runWrong = 0;
  List<String> tries = [];
  final neededHelp = <api.MissedDto>[];

  // ── results
  String doneStars = '', doneMsg = '', doneStats = '';

  // ── lesson
  api.LessonDto? lesson;
  int step = 0, stepTries = 0;
  bool cheer = false;
  ColumnPadController? lessonPad;
  final lessonEntry = TextEditingController();
  String lessonSay = '', lessonFace = '🦉', lessonBoard = '';

  String get id => widget.level.id;

  @override
  void initState() {
    super.initState();
    final st = api.players()[widget.player].levels[widget.level.flat];
    if (st.stars == 0 && !st.learned) {
      openLesson();
    } else {
      openSetup();
    }
  }

  // ───────────────────────── setup ─────────────────────────

  void openSetup() {
    final s = api.suggestedCount(index: widget.player, levelId: id);
    setState(() {
      chosen = s.count;
      suggestWhy = s.why;
      stage = Stage.setup;
    });
  }

  void begin() {
    Sounds.tap();
    total = chosen;
    q = clean = streak = bestStreak = 0;
    runAt = _now();
    clock
      ..reset()
      ..start();
    runHints = runWrong = 0;
    neededHelp.clear();
    stage = Stage.game;
    nextQuestion();
  }

  // ───────────────────────── questions ─────────────────────────

  int get answered => q + (solved ? 1 : 0);

  void nextQuestion() {
    final p = api.makeProblem(levelId: id, seed: BigInt.from(rng.nextInt(1 << 31)))!;
    setState(() {
      problem = p;
      pad = p.column == null ? null : (ColumnPadController(p.column!)..onKey = Sounds.tap);
      entry.clear();
      hintsShown = 0;
      missed = solved = false;
      tries = [];
      cardColor = Colors.white;
      face = '🦉';
      say = switch (q) {
        0 => 'Let\'s do "${widget.level.title}"! Need help? Look at the side panel. →',
        _ when q + 1 == total => 'Last one! You can do it! 💪',
        _ when total >= 20 && q == total ~/ 2 => "Halfway there! Look how far you've come! 🎉",
        _ when total >= 50 && q % 10 == 0 => '$q done already! Keep going! 💪',
        _ when streak >= 3 => '$streak in a row! You\'re on fire! 🔥',
        _ => ['Here\'s the next one!', 'Ready? Let\'s go!', 'Take your time. 🙂', "You've got this!"][rng.nextInt(4)],
      };
    });
    scratch.reset(p.given);
  }

  void press(String k) {
    if (solved) return;
    if (pad != null) {
      pad!.press(k);
      return;
    }
    Sounds.tap();
    var t = entry.text;
    switch (k) {
      case '⌫':
        if (t.isNotEmpty) t = t.substring(0, t.length - 1);
      case 'r':
        t += ' r ';
      case ',':
        t += ', ';
      default:
        t += k;
    }
    setState(() => entry.text = t);
  }

  void showHint({bool asked = true}) {
    if (hintsShown >= problem.hints.length) return;
    setState(() {
      hintsShown++;
      runHints++;
      if (asked) {
        Sounds.hint();
        face = '💡';
        say = hintsShown == problem.hints.length ? "Here's the whole way to solve it. Follow the steps!" : "Here's a clue! Check the side panel. →";
      }
    });
  }

  void submit() {
    if (solved) {
      q++;
      solved = false;
      if (q >= total) {
        finish();
      } else {
        nextQuestion();
      }
      return;
    }
    String text;
    if (pad != null) {
      final (t, why) = pad!.text();
      if (t == null) {
        setState(() {
          face = '🤔';
          say = why!;
        });
        return;
      }
      text = t;
    } else {
      text = entry.text;
    }
    final verdict = api.check(answer: problem.answer, input: text);
    if (pad != null && verdict is! api.VerdictDto_Right) pad!.mark(problem.answer);
    setState(() {
      switch (verdict) {
        case api.VerdictDto_Right():
          solved = true;
          pad?.mark(problem.answer);
          pad?.setLocked(true);
          cardColor = const Color(0xFFC9F5C4);
          if (!missed && hintsShown <= 1) {
            clean++;
          } else {
            neededHelp.add(api.MissedDto(question: problem.describe, answer: problem.answerShown, tries: tries, hints: hintsShown));
          }
          streak = missed ? 0 : streak + 1;
          bestStreak = math.max(bestStreak, streak);
          face = '🥳';
          say = missed ? "Yes! You figured it out! That's how we learn 🌱" : praise[rng.nextInt(praise.length)];
          Sounds.right();
          confetti.burst(streak >= 3 ? 90 : 45);
        case api.VerdictDto_Almost(:final message):
          tries = [...tries, text];
          Sounds.almost();
          face = '🤩';
          say = message;
        case api.VerdictDto_Unreadable(:final message):
          face = '🤔';
          say = message;
        case api.VerdictDto_Wrong():
          tries = [...tries, text];
          runWrong++;
          missed = true;
          streak = 0;
          cardColor = const Color(0xFFFFD9D9);
          Sounds.wrong();
          if (hintsShown >= problem.hints.length) {
            face = '🦉';
            say = 'The answer is ${problem.answerShown}. Look at the steps on the side, then type it in!';
          } else {
            face = '🤔';
            say = pad != null ? 'Not quite! The red boxes need another look. The hint shows each column. 💡' : oops[rng.nextInt(oops.length)];
            showHint(asked: false);
          }
          Future.delayed(const Duration(milliseconds: 900), () {
            if (mounted && !solved) setState(() => cardColor = Colors.white);
          });
      }
    });
  }

  void record({required bool finished, required int stars, bool lessonRun = false}) {
    final at = runAt;
    if (at == null) return;
    runAt = null;
    api.recordRun(
      index: widget.player,
      run: api.RunDto(
        level: id,
        at: at,
        seconds: clock.elapsed.inSeconds,
        lesson: lessonRun,
        total: lessonRun ? 0 : total,
        answered: lessonRun ? 0 : answered,
        firstTry: lessonRun ? 0 : clean,
        hints: runHints,
        wrong: runWrong,
        stars: stars,
        finished: finished,
        missed: lessonRun ? [] : List.of(neededHelp),
      ),
    );
  }

  void finish() {
    final n = math.max(answered, 1);
    final out = api.finishRun(index: widget.player, levelId: id, answered: n, total: total, firstTry: clean, bestStreak: bestStreak);
    record(finished: n >= total, stars: out.stars);
    Sounds.levelDone();
    confetti.burst(160);
    Future.delayed(const Duration(milliseconds: 500), () => confetti.burst(120));
    setState(() {
      stage = Stage.done;
      doneStars = starText(out.stars);
      doneMsg = switch ((out.newMedal, out.stars)) {
        (100, _) => 'MARATHON TROPHY! 🏆 100 questions with 3 stars. Amazing!',
        (int _, _) => 'CHALLENGE MEDAL! 🥈 50 questions with 3 stars!',
        (_, 3) => "PERFECT! You're a math superstar! 🌟",
        (_, 2) => 'Great work! Play again to try for 3 stars!',
        _ => 'You finished! Every try makes your brain stronger 💪',
      };
      doneStats = '$clean of $n on the first try${n < total ? ' (stopped at $n of $total)' : ''} · best streak 🔥 $bestStreak'
          '\n${nextLevel != null ? 'The next level is open!' : 'You reached the top of the map! 🏔️'}';
    });
  }

  // ───────────────────────── lesson ─────────────────────────

  void openLesson() {
    final l = api.lesson(levelId: id, seed: BigInt.from(rng.nextInt(1 << 31)))!;
    runAt = _now();
    clock
      ..reset()
      ..start();
    runHints = runWrong = 0;
    scratch.reset([]);
    setState(() {
      lesson = l;
      step = 0;
      cheer = false;
      lessonBoard = l.problem.prompt;
      lessonPad = l.problem.column == null
          ? null
          : (ColumnPadController(l.problem.column!)
            ..locked = true
            ..onKey = Sounds.tap
            ..onGuided = guided);
      stage = Stage.learn;
    });
    showStep();
  }

  void showStep() {
    final st = lesson!.steps[step];
    setState(() {
      stepTries = 0;
      lessonSay = (cheer ? nice[rng.nextInt(nice.length)] : '') + st.say;
      cheer = false;
      lessonFace = step + 1 == lesson!.steps.length ? '🥳' : '🦉';
      if (st.board != null) lessonBoard = st.board!;
      lessonEntry.clear();
    });
    lessonPad?.point(st.point);
    lessonPad?.setGuide(st.write);
  }

  void advanceLesson() {
    if (step + 1 >= lesson!.steps.length) {
      api.markLearned(index: widget.player, levelId: id);
      record(finished: true, stars: 0, lessonRun: true);
      chosen = 10;
      begin();
      return;
    }
    step++;
    showStep();
  }

  void lessonPress(String k) {
    final st = lesson!.steps[step];
    if (st.write != null) {
      lessonPad?.press(k);
      return;
    }
    Sounds.tap();
    var t = lessonEntry.text;
    t = k == '⌫' ? (t.isEmpty ? t : t.substring(0, t.length - 1)) : t + (k == 'r' ? ' r ' : (k == ',' ? ', ' : k));
    setState(() => lessonEntry.text = t);
  }

  void lessonSubmit() {
    final ask = lesson!.steps[step].ask;
    if (ask == null) return;
    final v = api.check(answer: ask, input: lessonEntry.text);
    switch (v) {
      case api.VerdictDto_Right():
        Sounds.right();
        cheer = true;
        advanceLesson();
      case api.VerdictDto_Almost(:final message):
        Sounds.almost();
        setState(() {
          lessonFace = '🤩';
          lessonSay = message;
        });
      case api.VerdictDto_Unreadable(:final message):
        setState(() {
          lessonFace = '🤔';
          lessonSay = message;
        });
      case api.VerdictDto_Wrong():
        Sounds.wrong();
        stepTries++;
        setState(() {
          if (stepTries >= 2) {
            final shown = _answerText(ask);
            lessonFace = '🦉';
            lessonSay = "That's okay! It's $shown. I put it in for you, press Check and let's keep going.";
            lessonEntry.text = shown.replaceAll('−', '-');
          } else {
            lessonFace = '🤔';
            lessonSay = 'Hmm, not quite. Take your time and try again! 💪';
          }
        });
    }
  }

  void guided(bool ok) {
    if (ok) {
      Sounds.right();
      cheer = true;
      Future.delayed(const Duration(milliseconds: 450), () {
        if (mounted) advanceLesson();
      });
      return;
    }
    Sounds.wrong();
    stepTries++;
    final want = lesson!.steps[step].write?.digit ?? '?';
    setState(() {
      lessonFace = stepTries >= 2 ? '🦉' : '🤔';
      lessonSay = stepTries >= 2 ? 'Almost! The digit that goes there is $want. Type $want in the glowing box.' : 'Not that one. Look at the number we just found, and try again!';
    });
  }

  String _answerText(api.AnswerDto a) => switch (a) {
        api.AnswerDto_Int(:final n) => '$n',
        api.AnswerDto_Frac(:final n, :final d) => d == 1 ? '$n' : '$n/$d',
        api.AnswerDto_Dec(:final hundredths) => (hundredths / 100).toString(),
        api.AnswerDto_Remainder(:final q, :final r) => r == 0 ? '$q' : '$q r $r',
        api.AnswerDto_Roots(:final a, :final b) => '$a, $b',
        api.AnswerDto_Pair(:final x, :final y) => '$x, $y',
        api.AnswerDto_Ineq(:final op, :final n) => 'x $op $n',
      };

  // ───────────────────────── layout ─────────────────────────

  @override
  Widget build(BuildContext context) {
    final wide = MediaQuery.sizeOf(context).width >= 1000;
    final body = Stack(
      children: [
        Positioned.fill(child: switch (stage) {
          Stage.setup => setupView(),
          Stage.learn => learnView(),
          Stage.game => gameView(),
          Stage.done => doneView(),
        }),
        Positioned.fill(child: Confetti(controller: confetti)),
      ],
    );
    return PopScope(
      onPopInvokedWithResult: (didPop, _) {
        // Leaving in the middle still counts on the score sheet.
        if (didPop && stage == Stage.game && answered > 0) record(finished: false, stars: 0);
      },
      child: Sky(
        child: Scaffold(
          appBar: AppBar(
            title: Text('${api.worlds()[widget.level.world].emoji} ${widget.level.title}', style: const TextStyle(fontWeight: FontWeight.bold)),
            actions: [
              IconButton(
                icon: Text(Sounds.on ? '🔊' : '🔇', style: const TextStyle(fontSize: 22)),
                onPressed: () => setState(Sounds.toggle),
              ),
              if (!wide) Builder(builder: (c) => TextButton(onPressed: () => Scaffold.of(c).openEndDrawer(), child: const Text('💡 Help', style: TextStyle(fontSize: 18)))),
            ],
          ),
          endDrawer: wide ? null : Drawer(width: 420, backgroundColor: const Color(0xFFFFFDF4), child: SafeArea(child: sidePanel())),
          body: SafeArea(
            child: wide
                ? Row(children: [
                    Expanded(child: body),
                    Container(
                      width: showScratch ? 460 : 380,
                      decoration: const BoxDecoration(color: Color(0xFFFFFDF4), border: Border(left: BorderSide(color: Color(0x14000000)))),
                      child: sidePanel(),
                    ),
                  ])
                : body,
          ),
        ),
      ),
    );
  }

  Widget sidePanel() => StatefulBuilder(
        builder: (context, set) => Column(
          children: [
            if (scratchOk)
              Padding(
                padding: const EdgeInsets.fromLTRB(14, 12, 14, 0),
                child: SegmentedButton<bool>(
                  segments: const [ButtonSegment(value: false, label: Text('💡 Help')), ButtonSegment(value: true, label: Text('🧮 Scratchpad'))],
                  selected: {showScratch},
                  onSelectionChanged: (s) => setState(() => showScratch = s.first),
                ),
              ),
            Expanded(
              child: showScratch && scratchOk
                  ? Scratchpad(
                      controller: scratch,
                      onUse: (v) {
                        if (stage == Stage.game && !solved && pad == null) setState(() => entry.text = v);
                      })
                  : helpView(),
            ),
          ],
        ),
      );

  Widget helpView() => ListView(
        padding: const EdgeInsets.all(18),
        children: [
          const Text('📘 How it works', style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold, color: prompt)),
          const SizedBox(height: 8),
          MathText(widget.level.lesson, style: const TextStyle(fontSize: 17)),
          if (lessonFigure != null) Padding(padding: const EdgeInsets.only(top: 12), child: Center(child: FigureView(figure: lessonFigure!, width: 300, height: 180))),
          const Divider(height: 32),
          const Text('💡 Hints for this one', style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold, color: prompt)),
          const SizedBox(height: 8),
          if (stage == Stage.game) ...[
            for (var i = 0; i < hintsShown; i++)
              Padding(
                padding: const EdgeInsets.only(bottom: 10),
                child: CardBox(
                  radius: 16,
                  padding: const EdgeInsets.all(12),
                  color: i + 1 == problem.hints.length ? hintBlue : hintYellow,
                  child: MathText(problem.hints[i], style: const TextStyle(fontSize: 17)),
                ),
              ),
            FilledButton.tonal(
              onPressed: hintsShown >= problem.hints.length ? null : showHint,
              child: Text(
                hintsShown >= problem.hints.length ? "That's every step!" : '💡 ${hintsShown == 0 ? 'Show a hint' : 'Show the next hint'} (${hintsShown + 1} of ${problem.hints.length})',
                style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
              ),
            ),
          ] else
            const Text('💡 Hints come during practice', style: TextStyle(fontSize: 17, color: inkSoft)),
        ],
      );

  Widget setupView() => Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(28),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text('${api.worlds()[widget.level.world].emoji} ${widget.level.title}', textAlign: TextAlign.center, style: const TextStyle(fontSize: 34, fontWeight: FontWeight.bold, color: prompt)),
              const SizedBox(height: 18),
              Kiko(bubble: Text('How many questions today? $suggestWhy I suggest ${api.suggestedCount(index: widget.player, levelId: id).count}!', style: const TextStyle(fontSize: 20))),
              const SizedBox(height: 24),
              Wrap(
                spacing: 14,
                runSpacing: 14,
                alignment: WrapAlignment.center,
                children: [
                  for (final (n, name) in counts)
                    GestureDetector(
                      onTap: () => setState(() => chosen = n),
                      child: Container(
                        width: 140,
                        height: 104,
                        decoration: BoxDecoration(
                          color: chosen == n ? const Color(0xFFFFE08A) : Colors.white,
                          borderRadius: BorderRadius.circular(22),
                          border: chosen == n ? Border.all(color: accent, width: 4) : null,
                          boxShadow: const [BoxShadow(color: Color(0x1A000000), blurRadius: 8, offset: Offset(0, 3))],
                        ),
                        child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
                          Text('$n', style: const TextStyle(fontSize: 30, fontWeight: FontWeight.bold)),
                          Text(name, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                        ]),
                      ),
                    ),
                ],
              ),
              const SizedBox(height: 26),
              Wrap(spacing: 14, children: [
                OutlinedButton(onPressed: openLesson, style: OutlinedButton.styleFrom(padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 18)), child: const Text('📖 Show me how', style: TextStyle(fontSize: 20))),
                bigButton('Start ➜', go, begin),
              ]),
            ],
          ),
        ),
      );

  Widget bigButton(String label, Color color, VoidCallback onTap) => FilledButton(
        onPressed: onTap,
        style: FilledButton.styleFrom(backgroundColor: color, padding: const EdgeInsets.symmetric(horizontal: 34, vertical: 18), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18))),
        child: Text(label, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
      );

  Widget questionCard(api.ProblemDto p, String board, ColumnPadController? padC, Color color) {
    final long = board.length > 34 || p.column != null;
    return CardBox(
      color: color,
      padding: const EdgeInsets.symmetric(horizontal: 26, vertical: 18),
      child: Column(
        children: [
          if (p.visual != null) Text(p.visual!, textAlign: TextAlign.center, style: const TextStyle(fontSize: 34)),
          if (p.figure != null) FigureView(figure: p.figure!, width: 340, height: 185),
          MathText(board, align: TextAlign.center, style: TextStyle(fontSize: long ? 26 : 44, fontWeight: FontWeight.bold, color: prompt)),
          if (padC != null) Padding(padding: const EdgeInsets.only(top: 10), child: ColumnPad(c: padC)),
        ],
      ),
    );
  }

  // A tall card (column pad or diagram) gets the answer controls beside it, so
  // the keypad stays on screen; otherwise they sit underneath.
  // A column pad can't wrap, so it shrinks to fit instead.
  Widget workArea(Widget card, bool tall, List<Widget> controls, {bool shrink = false}) => LayoutBuilder(
        builder: (context, box) => tall && box.maxWidth >= 720
            ? Row(crossAxisAlignment: CrossAxisAlignment.start, mainAxisAlignment: MainAxisAlignment.center, children: [
                Flexible(child: shrink ? FittedBox(fit: BoxFit.scaleDown, alignment: Alignment.topCenter, child: card) : card),
                const SizedBox(width: 20),
                SizedBox(width: 316, child: Column(children: controls)),
              ])
            : Column(children: [card, const SizedBox(height: 12), ...controls]),
      );

  Widget answerField(TextEditingController c) => Expanded(
        child: TextField(
          controller: c,
          readOnly: true,
          showCursor: true,
          textAlign: TextAlign.center,
          style: const TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: ink),
          decoration: InputDecoration(
            hintText: 'Your answer',
            hintStyle: const TextStyle(fontSize: 20, fontWeight: FontWeight.normal, color: inkSoft),
            filled: true,
            fillColor: const Color(0xFFE8ECF4),
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(18)),
          ),
        ),
      );

  Widget gameView() => SingleChildScrollView(
        padding: const EdgeInsets.fromLTRB(28, 14, 28, 24),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 820),
            child: Column(
              children: [
                Row(children: [
                  Expanded(child: ClipRRect(borderRadius: BorderRadius.circular(10), child: LinearProgressIndicator(value: (q + (solved ? 1 : 0)) / total, minHeight: 16, color: accent, backgroundColor: const Color(0x1F3A3A7A)))),
                  const SizedBox(width: 14),
                  Text('${q + 1} / $total', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                  if (streak >= 2) Padding(padding: const EdgeInsets.only(left: 12), child: Text('🔥 $streak', style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold, color: Color(0xFFE8590C)))),
                  if (total > 10 && answered >= 10 && answered < total)
                    Padding(padding: const EdgeInsets.only(left: 12), child: OutlinedButton(onPressed: finish, child: const Text('🏁 Stop here'))),
                ]),
                const SizedBox(height: 12),
                Kiko(face: face, bubble: Text(say, style: const TextStyle(fontSize: 20))),
                const SizedBox(height: 12),
                workArea(questionCard(problem, problem.prompt, pad, cardColor), pad != null || problem.figure != null, [
                  Row(children: [
                    if (pad == null) ...[answerField(entry), const SizedBox(width: 12)] else const Spacer(),
                    bigButton(solved ? (q + 1 == total ? 'Finish 🏁' : 'Next ➜') : 'Check ✔', solved ? next : go, submit),
                    if (pad != null) const Spacer(),
                  ]),
                  const SizedBox(height: 6),
                  Text(pad != null ? (pad!.division ? 'Type the answer on top, left to right. Put what\'s left over in the r box.' : 'Type the answer from right to left. Tap a small box on top to write a carry.') : problem.formatTip,
                      textAlign: TextAlign.center, style: const TextStyle(fontSize: 16, color: inkSoft)),
                  const SizedBox(height: 10),
                  Keypad(extras: pad != null ? const [] : (problem.keys.where((k) => k != '−').toList()..insert(0, '−')), onKey: press),
                ], shrink: pad != null),
              ],
            ),
          ),
        ),
      );

  Widget learnView() {
    final l = lesson!;
    final st = l.steps[step];
    final last = step + 1 == l.steps.length;
    return SingleChildScrollView(
      padding: const EdgeInsets.fromLTRB(28, 14, 28, 24),
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 820),
          child: Column(
            children: [
              Row(children: [
                const Expanded(child: Text("📖 Let's learn it together!", style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold, color: prompt))),
                Text('Step ${step + 1} of ${l.steps.length}', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(width: 12),
                OutlinedButton(
                  onPressed: () {
                    api.markLearned(index: widget.player, levelId: id);
                    runAt = null;
                    openSetup();
                  },
                  child: const Text('Skip ⏭'),
                ),
              ]),
              const SizedBox(height: 12),
              Kiko(face: lessonFace, bubble: MathText(lessonSay, style: const TextStyle(fontSize: 21))),
              const SizedBox(height: 12),
              workArea(questionCard(l.problem, lessonBoard, lessonPad, Colors.white), lessonPad != null || l.problem.figure != null, [
                if (st.ask != null)
                  FittedBox(
                    fit: BoxFit.scaleDown,
                    child: Row(mainAxisSize: MainAxisSize.min, children: [
                      SizedBox(width: 200, child: Row(children: [answerField(lessonEntry)])),
                      const SizedBox(width: 12),
                      bigButton('Check ✔', go, lessonSubmit),
                    ]),
                  )
                else if (st.write == null)
                  bigButton(last ? "Let's practice! ➜" : 'Next ➜', next, () {
                    Sounds.tap();
                    advanceLesson();
                  }),
                const SizedBox(height: 6),
                Text(st.ask != null ? 'Type your answer, then press Check.' : (st.write != null ? 'Type the digit in the glowing box ✨' : ''),
                    textAlign: TextAlign.center, style: const TextStyle(fontSize: 16, color: inkSoft)),
                const SizedBox(height: 10),
                if (st.ask != null || st.write != null)
                  Keypad(extras: st.ask == null ? const [] : _keysFor(st.ask!), onKey: lessonPress),
              ], shrink: lessonPad != null),
            ],
          ),
        ),
      ),
    );
  }

  List<String> _keysFor(api.AnswerDto a) => switch (a) {
        api.AnswerDto_Frac() => ['−', '/'],
        api.AnswerDto_Dec() => ['−', '.'],
        api.AnswerDto_Remainder() => ['r'],
        api.AnswerDto_Roots() || api.AnswerDto_Pair() => ['−', ','],
        api.AnswerDto_Ineq() => ['−', '<', '>', '='],
        _ => ['−'],
      };

  Widget doneView() => Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(28),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text('🦉', style: TextStyle(fontSize: 72)),
              Text(doneStars, style: const TextStyle(fontSize: 90, color: star)),
              Text(doneMsg, textAlign: TextAlign.center, style: const TextStyle(fontSize: 34, fontWeight: FontWeight.bold, color: prompt)),
              const SizedBox(height: 10),
              Text(doneStats, textAlign: TextAlign.center, style: const TextStyle(fontSize: 20, color: inkSoft)),
              const SizedBox(height: 22),
              Wrap(spacing: 14, runSpacing: 14, alignment: WrapAlignment.center, children: [
                OutlinedButton(onPressed: () => Navigator.of(context).pop(), style: OutlinedButton.styleFrom(padding: const EdgeInsets.all(18)), child: const Text('🗺️ Map', style: TextStyle(fontSize: 20))),
                OutlinedButton(onPressed: openSetup, style: OutlinedButton.styleFrom(padding: const EdgeInsets.all(18)), child: const Text('🔁 Play again', style: TextStyle(fontSize: 20))),
                if (nextLevel != null)
                  bigButton('Next level ➜', next, () {
                    Navigator.of(context).pushReplacement(MaterialPageRoute(builder: (_) => LevelScreen(player: widget.player, level: nextLevel!)));
                  }),
              ]),
            ],
          ),
        ),
      );
}
