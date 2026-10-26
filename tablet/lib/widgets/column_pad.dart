// The "two-layer" answer pad, like the desktop's src/column.rs: the problem
// written in columns as on paper, with a box for every answer digit. Kids
// fill it from the ones place (or left to right above a long-division
// bracket) and can jot carries in the small boxes. Checking colors each
// digit; in a lesson only the glowing box takes a digit.

import 'package:flutter/material.dart';

import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';

enum SlotKind { answer, carry, remainder }

class Slot {
  final SlotKind kind;

  /// Place from the right (0 = ones); for long division, the dividend digit from the left.
  final int place;
  final int col, row;
  String? value;
  bool? good;
  Slot(this.kind, this.place, this.col, this.row);
}

class ColumnPadController extends ChangeNotifier {
  final api.ColumnDto col;
  final slots = <Slot>[];

  /// Answer boxes left to right, then the remainder box.
  final visual = <int>[];

  /// The order digits are typed in.
  final order = <int>[];

  /// Fixed text in the grid: (col, row, text).
  final fixed = <(int, int, String)>[];
  late final int cols;
  late final (int, int, int) bar; // (row, from col, span)
  int cursor = 0, resume = 0;
  bool locked = false;
  (int, String)? guide; // slot, digit
  int? pointed;
  void Function(bool ok)? onGuided;
  void Function()? onKey;

  bool get division => col.op == '÷';

  ColumnPadController(this.col) {
    if (division) {
      final ds = col.top.toString().split('');
      final n = ds.length;
      for (var k = 0; k < n; k++) {
        visual.add(_add(Slot(SlotKind.answer, k, 2 + k, 0)));
      }
      fixed.add((2 + n, 0, 'r'));
      visual.add(_add(Slot(SlotKind.remainder, -1, 3 + n, 0)));
      bar = (1, 1, n + 1);
      fixed.add((0, 2, col.bottom.toString()));
      fixed.add((1, 2, ')'));
      for (var k = 0; k < n; k++) {
        fixed.add((2 + k, 2, ds[k]));
      }
      cols = n + 4;
      order.addAll(visual);
    } else {
      final top = col.top.toString().split(''), bottom = col.bottom.toString().split('');
      final w = col.op == '−' ? top.length : (top.length > bottom.length ? top.length : bottom.length) + 1;
      String? digitAt(List<String> ds, int place) {
        final i = ds.length - 1 - place;
        return i >= 0 ? ds[i] : null;
      }

      for (var c = 1; c <= w; c++) {
        final place = w - c;
        final carry = col.op == '−' ? place >= 1 && place < top.length : place >= 1 && place <= top.length;
        if (carry) _add(Slot(SlotKind.carry, place, c, 0));
        final t = digitAt(top, place), b = digitAt(bottom, place);
        if (t != null) fixed.add((c, 1, t));
        if (b != null) fixed.add((c, 2, b));
        visual.add(_add(Slot(SlotKind.answer, place, c, 4)));
      }
      fixed.add((0, 2, col.op));
      bar = (3, 0, w + 1);
      cols = w + 1;
      order.addAll(visual.reversed);
    }
    cursor = order.first;
  }

  int _add(Slot s) {
    slots.add(s);
    return slots.length - 1;
  }

  void select(int i) {
    if (locked || guide != null) return;
    cursor = i;
    final pos = order.indexOf(i);
    if (pos >= 0) resume = pos;
    notifyListeners();
  }

  void setLocked(bool v) {
    locked = v;
    notifyListeners();
  }

  void point(int? place) {
    pointed = place;
    notifyListeners();
  }

  int? _slotFor(api.WriteDto w) {
    final i = slots.indexWhere((s) => switch (w.kind) {
          'answer' || 'quotient' => s.kind == SlotKind.answer && s.place == w.index,
          'carry' => s.kind == SlotKind.carry && s.place == w.index,
          _ => s.kind == SlotKind.remainder,
        });
    return i < 0 ? null : i;
  }

  /// Lesson mode: only this box takes a digit, and it glows. Null ends it.
  void setGuide(api.WriteDto? w) {
    final slot = w == null ? null : _slotFor(w);
    guide = slot == null ? null : (slot, w!.digit);
    if (slot != null) {
      cursor = slot;
      slots[slot].good = null;
    }
    notifyListeners();
  }

  /// A digit or ⌫ from the keypad.
  void press(String k) {
    if (guide case (final slot, final want)) {
      if (!RegExp(r'^\d$').hasMatch(k)) return;
      onKey?.call();
      slots[slot].value = k;
      final ok = k == want;
      slots[slot].good = ok;
      if (ok) guide = null;
      notifyListeners();
      onGuided?.call(ok);
      return;
    }
    if (locked) return;
    onKey?.call();
    final s = slots[cursor];
    if (k == '⌫') {
      if (s.value != null) {
        s.value = null;
      } else {
        final pos = order.indexOf(cursor);
        if (pos > 0) {
          cursor = order[pos - 1];
          slots[cursor].value = null;
          resume = pos - 1;
        }
      }
    } else if (RegExp(r'^\d$').hasMatch(k)) {
      s.value = k;
      s.good = null;
      if (s.kind == SlotKind.carry) {
        cursor = order[resume];
      } else {
        final pos = order.indexOf(cursor);
        if (pos >= 0 && pos + 1 < order.length) {
          cursor = order[pos + 1];
          resume = pos + 1;
        }
      }
    }
    notifyListeners();
  }

  /// What the kid wrote ("1508", "110 r 1"), or a friendly reason it can't be read.
  (String?, String?) text() {
    final digits = StringBuffer();
    var gap = false;
    String? rem;
    for (final i in visual) {
      final s = slots[i];
      if (s.kind == SlotKind.remainder) {
        rem = s.value;
      } else if (s.value != null) {
        if (gap) return (null, "There's an empty box in the middle of your answer. Fill it in (it might be 0)!");
        digits.write(s.value);
      } else if (digits.isNotEmpty) {
        gap = true;
      }
    }
    if (digits.isEmpty) return (null, 'Type your answer in the boxes!');
    return (rem == null ? digits.toString() : '$digits r $rem', null);
  }

  /// Color each answer box green or red against the right answer.
  void mark(api.AnswerDto answer) {
    final (main, rem) = switch (answer) {
      api.AnswerDto_Int(:final n) => (n, null),
      api.AnswerDto_Remainder(:final q, :final r) => (q, r),
      _ => (null, null),
    };
    if (main == null) return;
    final want = main.toString().split('');
    final boxes = visual.where((i) => slots[i].kind == SlotKind.answer).toList();
    final pad = boxes.length - want.length;
    for (var k = 0; k < boxes.length; k++) {
      final expected = k - pad >= 0 ? want[k - pad] : null;
      _paint(boxes[k], expected, false);
    }
    if (rem != null) _paint(visual.last, rem.toString(), rem == 0);
    notifyListeners();
  }

  void _paint(int i, String? expected, bool blankIsZero) {
    final s = slots[i];
    final blankOk = s.value == null || s.value == '0';
    if (expected == null) {
      s.good = blankOk ? null : false;
    } else if (s.value == expected) {
      s.good = true;
    } else if (expected == '0' && blankIsZero && s.value == null) {
      s.good = null;
    } else {
      s.good = false;
    }
  }
}

class ColumnPad extends StatelessWidget {
  final ColumnPadController c;
  final double cell;
  const ColumnPad({super.key, required this.c, this.cell = 54});

  @override
  Widget build(BuildContext context) => ListenableBuilder(
        listenable: c,
        builder: (context, _) {
          final rows = <Widget>[];
          final rowCount = c.division ? 3 : 5;
          for (var r = 0; r < rowCount; r++) {
            if (r == c.bar.$1) {
              rows.add(Row(mainAxisSize: MainAxisSize.min, children: [
                SizedBox(width: c.bar.$2 * cell),
                Container(width: c.bar.$3 * cell, height: 4, margin: const EdgeInsets.symmetric(vertical: 4), color: prompt),
                SizedBox(width: (c.cols - c.bar.$2 - c.bar.$3) * cell),
              ]));
              continue;
            }
            final cells = <Widget>[];
            for (var col = 0; col < c.cols; col++) {
              final si = c.slots.indexWhere((s) => s.col == col && s.row == r);
              final fx = c.fixed.where((f) => f.$1 == col && f.$2 == r).firstOrNull;
              Widget w = const SizedBox();
              if (si >= 0) {
                w = _box(si);
              } else if (fx != null) {
                final placeOf = c.division ? col - 2 : c.cols - 1 - col;
                final lit = c.pointed != null && placeOf == c.pointed && (r == 1 || r == 2) && fx.$3.contains(RegExp(r'\d'));
                w = Container(
                  decoration: lit ? BoxDecoration(color: const Color(0xFFFFE58A), borderRadius: BorderRadius.circular(12)) : null,
                  alignment: Alignment.center,
                  child: Text(fx.$3, style: TextStyle(fontSize: fx.$3 == ')' ? 52 : 40, fontWeight: fx.$3 == ')' ? FontWeight.w300 : FontWeight.bold, color: prompt)),
                );
              }
              cells.add(SizedBox(width: cell, height: r == 0 && !c.division ? cell * 0.6 : cell * 1.08, child: Padding(padding: const EdgeInsets.all(3), child: w)));
            }
            rows.add(Row(mainAxisSize: MainAxisSize.min, children: cells));
          }
          return Column(mainAxisSize: MainAxisSize.min, children: rows);
        },
      );

  Widget _box(int i) {
    final s = c.slots[i];
    final carry = s.kind == SlotKind.carry;
    final guided = c.guide?.$1 == i;
    final active = guided || (i == c.cursor && !c.locked && c.guide == null);
    final lit = c.pointed != null && s.kind != SlotKind.remainder && s.place == c.pointed;
    Color bg = lit ? const Color(0xFFFFF3C4) : const Color(0xFFFBFCFF);
    Color border = carry ? const Color(0xFFF0C060) : const Color(0xFFB8C4E8);
    if (active) {
      bg = const Color(0xFFFFF7E0);
      border = accent;
    }
    if (s.good == true) {
      bg = const Color(0xFFC9F5C4);
      border = go;
    } else if (s.good == false) {
      bg = const Color(0xFFFFD9D9);
      border = const Color(0xFFE5484D);
    }
    return GestureDetector(
      onTap: () => c.select(i),
      child: Container(
        decoration: BoxDecoration(color: bg, borderRadius: BorderRadius.circular(carry ? 8 : 12), border: Border.all(color: border, width: active || s.good != null ? 3 : 2)),
        alignment: Alignment.center,
        child: Text(s.value ?? '', style: TextStyle(fontSize: carry ? 17 : 34, fontWeight: FontWeight.bold, color: carry ? const Color(0xFFD9480F) : ink)),
      ),
    );
  }
}
