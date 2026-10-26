// Text with real-looking formulas, like the desktop app (src/math.rs).
// Inside text, anything between $…$ is a formula: \frac{a}{b} stacks,
// \sqrt{x} gets a bar, ^{…} and _{…} go up and down. Formulas sit inside
// the line and wrap with it.

import 'package:flutter/material.dart';

sealed class _Node {}

class _Text extends _Node {
  final String t;
  _Text(this.t);
}

class _Frac extends _Node {
  final List<_Node> top, bottom;
  _Frac(this.top, this.bottom);
}

class _Sqrt extends _Node {
  final List<_Node> inner;
  _Sqrt(this.inner);
}

class _Script extends _Node {
  final List<_Node> inner;
  final bool up;
  _Script(this.inner, this.up);
}

class _Parser {
  final List<String> c;
  int i = 0;
  _Parser(String s) : c = s.characters.toList();

  List<_Node> nodes() {
    final out = <_Node>[];
    final text = StringBuffer();
    void flush() {
      if (text.isNotEmpty) {
        out.add(_Text(text.toString()));
        text.clear();
      }
    }

    while (i < c.length) {
      final ch = c[i];
      if (ch == '}') {
        i++;
        break;
      } else if (ch == r'\') {
        i++;
        final name = StringBuffer();
        while (i < c.length && RegExp(r'[A-Za-z]').hasMatch(c[i])) {
          name.write(c[i++]);
        }
        switch (name.toString()) {
          case 'frac':
            flush();
            final a = group(), b = group();
            out.add(_Frac(a, b));
          case 'sqrt':
            flush();
            out.add(_Sqrt(group()));
          case '':
            if (i < c.length) text.write(c[i++]);
          default:
            text.write(name);
        }
      } else if (ch == '^' || ch == '_') {
        i++;
        flush();
        out.add(_Script(group(), ch == '^'));
      } else {
        text.write(ch);
        i++;
      }
    }
    flush();
    return out;
  }

  List<_Node> group() {
    if (i >= c.length) return [];
    if (c[i] == '{') {
      i++;
      return nodes();
    }
    return [_Text(c[i++])];
  }
}

const _supDigits = {'0': '⁰', '1': '¹', '2': '²', '3': '³', '4': '⁴', '5': '⁵', '6': '⁶', '7': '⁷', '8': '⁸', '9': '⁹', '-': '⁻', '−': '⁻', '+': '⁺', 'n': 'ⁿ', 'x': 'ˣ'};

/// "2" → "²", if every character has a superscript form.
String? _superscript(List<_Node> n) {
  if (n.length != 1 || n.first is! _Text) return null;
  final t = (n.first as _Text).t;
  final out = StringBuffer();
  for (final ch in t.characters) {
    final s = _supDigits[ch];
    if (s == null) return null;
    out.write(s);
  }
  return out.toString();
}

List<InlineSpan> _spans(List<_Node> nodes, TextStyle style) {
  final out = <InlineSpan>[];
  for (final n in nodes) {
    switch (n) {
      case _Text(:final t):
        out.add(TextSpan(text: t, style: style));
      case _Script(:final inner, :final up):
        final sup = up ? _superscript(inner) : null;
        if (sup != null) {
          out.add(TextSpan(text: sup, style: style));
        } else {
          final small = style.copyWith(fontSize: (style.fontSize ?? 16) * 0.65);
          out.add(WidgetSpan(
            alignment: up ? PlaceholderAlignment.top : PlaceholderAlignment.bottom,
            child: Text.rich(TextSpan(children: _spans(inner, small))),
          ));
        }
      case _Frac(:final top, :final bottom):
        out.add(WidgetSpan(
          alignment: PlaceholderAlignment.middle,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 3),
            child: IntrinsicWidth(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Padding(padding: const EdgeInsets.symmetric(horizontal: 4), child: Text.rich(TextSpan(children: _spans(top, style)), textAlign: TextAlign.center)),
                  Container(height: 2, margin: const EdgeInsets.symmetric(vertical: 2), color: style.color),
                  Padding(padding: const EdgeInsets.symmetric(horizontal: 4), child: Text.rich(TextSpan(children: _spans(bottom, style)), textAlign: TextAlign.center)),
                ],
              ),
            ),
          ),
        ));
      case _Sqrt(:final inner):
        out.add(WidgetSpan(
          alignment: PlaceholderAlignment.middle,
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text('√', style: style.copyWith(fontSize: (style.fontSize ?? 16) * 1.15)),
              Container(
                padding: const EdgeInsets.fromLTRB(2, 2, 2, 0),
                decoration: BoxDecoration(border: Border(top: BorderSide(color: style.color ?? Colors.black, width: 2))),
                child: Text.rich(TextSpan(children: _spans(inner, style))),
              ),
            ],
          ),
        ));
    }
  }
  return out;
}

/// The spans for a whole text: plain parts as text, `$…$` parts as math.
List<InlineSpan> mathSpans(String text, TextStyle style) {
  final out = <InlineSpan>[];
  final parts = text.split(r'$');
  for (var i = 0; i < parts.length; i++) {
    if (parts[i].isEmpty) continue;
    if (i.isOdd) {
      out.addAll(_spans(_Parser(parts[i]).nodes(), style));
    } else {
      out.add(TextSpan(text: parts[i], style: style));
    }
  }
  return out;
}

/// Text that may hold `$…$` formulas.
class MathText extends StatelessWidget {
  final String text;
  final TextStyle? style;
  final TextAlign align;
  const MathText(this.text, {super.key, this.style, this.align = TextAlign.start});

  @override
  Widget build(BuildContext context) {
    final base = DefaultTextStyle.of(context).style.merge(style);
    return Text.rich(TextSpan(children: mathSpans(text, base)), textAlign: align);
  }
}

/// The same text on plain lines: `$\frac{3}{4}$` → `3/4`.
String plainMath(String text) {
  String flat(List<_Node> nodes) => nodes.map((n) {
        switch (n) {
          case _Text(:final t):
            return t;
          case _Frac(:final top, :final bottom):
            return '${flat(top)}/${flat(bottom)}';
          case _Sqrt(:final inner):
            return '√${flat(inner)}';
          case _Script(:final inner, :final up):
            return up ? (_superscript(inner) ?? '^${flat(inner)}') : flat(inner);
        }
      }).join();
  final parts = text.split(r'$');
  return [for (var i = 0; i < parts.length; i++) i.isOdd ? flat(_Parser(parts[i]).nodes()) : parts[i]].join();
}
