// The scratchpad beside a problem, like the desktop's src/scratch.rs: a
// formula pad that works out one line at a time (the problem's values are
// ready to use), and a scribble page on graph paper for a finger or stylus.

import 'package:flutter/material.dart';

import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';
import 'math_text.dart';

class ScratchController extends ChangeNotifier {
  api.ScratchPad pad = api.ScratchPad();
  List<api.GivenValue> given = [];
  final lines = <api.LineDto>[];
  String message = '';
  final strokes = <(bool, List<Offset>)>[];

  /// A fresh pad and page for a new question.
  void reset(List<api.GivenValue> values) {
    pad = api.ScratchPad();
    given = values;
    for (final g in values) {
      pad.set_(name: g.name, value: g.value);
    }
    lines.clear();
    strokes.clear();
    message = '';
    notifyListeners();
  }

  bool run(String src) {
    try {
      lines.add(pad.run(src: src));
      message = '';
      notifyListeners();
      return true;
    } catch (e) {
      message = '🤔 ${e is String ? e : e.toString()}';
      notifyListeners();
      return false;
    }
  }

  void clearLines() {
    pad.clear(keep: [for (final g in given) g.name]);
    lines.clear();
    message = '';
    notifyListeners();
  }
}

class Scratchpad extends StatefulWidget {
  final ScratchController controller;

  /// "Use" on a line: put that result in the answer box.
  final void Function(String value) onUse;
  const Scratchpad({super.key, required this.controller, required this.onUse});

  @override
  State<Scratchpad> createState() => _ScratchpadState();
}

class _ScratchpadState extends State<Scratchpad> {
  final entry = TextEditingController();
  final scroll = ScrollController();
  bool paper = false, erasing = false;

  ScratchController get c => widget.controller;

  void insert(String s) {
    final sel = entry.selection;
    final at = sel.isValid ? sel.start : entry.text.length;
    entry.text = entry.text.replaceRange(at, sel.isValid ? sel.end : at, s);
    entry.selection = TextSelection.collapsed(offset: at + s.length);
  }

  void submit() {
    if (c.run(entry.text)) {
      entry.clear();
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (scroll.hasClients) scroll.jumpTo(scroll.position.maxScrollExtent);
      });
    }
  }

  Widget chip(String text, VoidCallback onTap, {Color color = Colors.white}) => Material(
        color: color,
        borderRadius: BorderRadius.circular(10),
        child: InkWell(
          borderRadius: BorderRadius.circular(10),
          onTap: onTap,
          child: Padding(padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6), child: Text(text, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: prompt))),
        ),
      );

  @override
  Widget build(BuildContext context) => ListenableBuilder(
        listenable: c,
        builder: (context, _) => Padding(
          padding: const EdgeInsets.all(14),
          child: Column(
            children: [
              SegmentedButton<bool>(
                segments: const [ButtonSegment(value: false, label: Text('🧮 Formula pad')), ButtonSegment(value: true, label: Text('✏️ Scribble'))],
                selected: {paper},
                onSelectionChanged: (s) => setState(() => paper = s.first),
              ),
              const SizedBox(height: 10),
              Expanded(child: paper ? scribble() : formulaPad()),
            ],
          ),
        ),
      );

  Widget formulaPad() => Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (c.given.isNotEmpty) ...[
            const Text('Given (tap to use):', style: TextStyle(fontSize: 14, color: inkSoft)),
            const SizedBox(height: 4),
            Wrap(spacing: 6, runSpacing: 6, children: [for (final g in c.given) chip('${g.name} = ${g.value.replaceAll('-', '−')}', () => insert(g.name), color: hintBlue)]),
            const SizedBox(height: 8),
          ],
          Expanded(
            child: c.lines.isEmpty
                ? const Text('Work it out here, one line at a time.\nTry: b^2 - 4ac,  sqrt(ans),  3/4 + 1/6,  sin 30\nName a value with d = …', style: TextStyle(fontSize: 15, color: inkSoft))
                : ListView.separated(
                    controller: scroll,
                    itemCount: c.lines.length,
                    separatorBuilder: (_, _) => const SizedBox(height: 8),
                    itemBuilder: (_, i) {
                      final l = c.lines[i];
                      return CardBox(
                        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                        radius: 12,
                        child: Row(
                          children: [
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  MathText(l.shown, style: const TextStyle(fontSize: 18)),
                                  MathText('= ${l.result}${l.approx == null ? '' : '   ≈ ${l.approx}'}', style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                                ],
                              ),
                            ),
                            chip('↪ Use', () => widget.onUse(l.plain)),
                          ],
                        ),
                      );
                    },
                  ),
          ),
          if (c.message.isNotEmpty) Padding(padding: const EdgeInsets.only(top: 6), child: Text(c.message, style: const TextStyle(color: Color(0xFFB54708), fontSize: 15))),
          const SizedBox(height: 8),
          Row(
            children: [
              Expanded(
                child: TextField(
                  controller: entry,
                  style: const TextStyle(fontSize: 18),
                  decoration: const InputDecoration(hintText: 'Type, then Enter', border: OutlineInputBorder(), isDense: true),
                  // Handling Enter here keeps the keyboard up for the next line.
                  onEditingComplete: submit,
                  textInputAction: TextInputAction.next,
                  autocorrect: false,
                  enableSuggestions: false,
                ),
              ),
              const SizedBox(width: 6),
              chip('Clear', c.clearLines),
            ],
          ),
          const SizedBox(height: 8),
          Wrap(spacing: 6, runSpacing: 6, children: [
            for (final k in ['√', '^', '(', ')', 'π', 'sin', 'cos', 'tan', '!', 'ans']) chip(k, () => insert(['√', 'sin', 'cos', 'tan'].contains(k) ? '$k(' : k)),
            chip('= ⏎', submit, color: const Color(0xFFC9F5C4)),
          ]),
        ],
      );

  Widget scribble() => Column(
        children: [
          Expanded(
            child: ClipRRect(
              borderRadius: BorderRadius.circular(12),
              child: Listener(
                onPointerDown: (e) => setState(() => c.strokes.add((erasing, [e.localPosition]))),
                onPointerMove: (e) => setState(() => c.strokes.last.$2.add(e.localPosition)),
                child: CustomPaint(painter: _Paper(c.strokes), size: Size.infinite),
              ),
            ),
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              ChoiceChip(label: const Text('✏️ Pen'), selected: !erasing, onSelected: (_) => setState(() => erasing = false)),
              const SizedBox(width: 6),
              ChoiceChip(label: const Text('🧽 Eraser'), selected: erasing, onSelected: (_) => setState(() => erasing = true)),
              const SizedBox(width: 6),
              ActionChip(label: const Text('🗑️ Clear'), onPressed: () => setState(c.strokes.clear)),
            ],
          ),
        ],
      );
}

/// Graph paper with ink; the eraser removes ink but not the grid.
class _Paper extends CustomPainter {
  final List<(bool, List<Offset>)> strokes;
  _Paper(this.strokes);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawRect(Offset.zero & size, Paint()..color = Colors.white);
    final grid = Paint()
      ..color = const Color(0xFFDBE5F7)
      ..strokeWidth = 1;
    for (var x = 0.5; x < size.width; x += 20) {
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), grid);
    }
    for (var y = 0.5; y < size.height; y += 20) {
      canvas.drawLine(Offset(0, y), Offset(size.width, y), grid);
    }
    canvas.saveLayer(Offset.zero & size, Paint());
    for (final (erase, pts) in strokes) {
      final p = Paint()
        ..style = PaintingStyle.stroke
        ..strokeCap = StrokeCap.round
        ..strokeJoin = StrokeJoin.round
        ..strokeWidth = erase ? 22 : 3
        ..color = erase ? Colors.transparent : ink
        ..blendMode = erase ? BlendMode.clear : BlendMode.srcOver;
      final path = Path()..moveTo(pts.first.dx, pts.first.dy);
      for (final q in pts.skip(1)) {
        path.lineTo(q.dx, q.dy);
      }
      if (pts.length == 1) path.lineTo(pts.first.dx + 0.1, pts.first.dy);
      canvas.drawPath(path, p);
    }
    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant CustomPainter old) => true;
}
