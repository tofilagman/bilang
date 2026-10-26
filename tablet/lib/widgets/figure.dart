// Labeled diagrams for the triangle and trig levels, like the desktop's
// src/figure.rs: drawn to scale, with the side to find marked "?" in orange.

import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';

class FigureView extends StatelessWidget {
  final api.FigureDto figure;
  final double width, height;
  const FigureView({super.key, required this.figure, this.width = 360, this.height = 200});

  @override
  Widget build(BuildContext context) => SizedBox(width: width, height: height, child: CustomPaint(painter: _FigurePainter(figure)));
}

class _FigurePainter extends CustomPainter {
  final api.FigureDto fig;
  _FigurePainter(this.fig);

  static const fill = Color(0xFFF0F5FF);
  static const soft = Color(0xFF3F80E0);

  Paint stroke(Color c, double w, {bool dashed = false}) => Paint()
    ..color = c
    ..strokeWidth = w
    ..style = PaintingStyle.stroke
    ..strokeCap = StrokeCap.round
    ..strokeJoin = StrokeJoin.round;

  void label(Canvas canvas, String text, Offset at, Offset dir, double gap) {
    if (text.isEmpty) return;
    final tp = TextPainter(
      text: TextSpan(text: text, style: TextStyle(fontFamily: 'ComicNeue', fontSize: 18, fontWeight: FontWeight.bold, color: text.contains('?') ? accent : ink)),
      textDirection: TextDirection.ltr,
    )..layout();
    final push = gap + 0.5 * (dir.dx.abs() * tp.width + dir.dy.abs() * tp.height);
    final c = at + dir * push;
    tp.paint(canvas, c - Offset(tp.width / 2, tp.height / 2));
  }

  Offset norm(Offset v) => v.distance == 0 ? v : v / v.distance;

  Offset outward(Offset p, Offset q, Offset inside) {
    var n = norm(Offset(-(q.dy - p.dy), q.dx - p.dx));
    final mid = (p + q) / 2;
    if (n.dx * (mid.dx - inside.dx) + n.dy * (mid.dy - inside.dy) < 0) n = -n;
    return n;
  }

  List<Offset> fit(List<Offset> pts, Size size, double pad) {
    final minx = pts.map((p) => p.dx).reduce(math.min), maxx = pts.map((p) => p.dx).reduce(math.max);
    final miny = pts.map((p) => p.dy).reduce(math.min), maxy = pts.map((p) => p.dy).reduce(math.max);
    final s = math.min((size.width - 2 * pad) / math.max(maxx - minx, 1e-9), (size.height - 2 * pad) / math.max(maxy - miny, 1e-9));
    final ox = (size.width - s * (maxx - minx)) / 2, oy = (size.height - s * (maxy - miny)) / 2;
    return [for (final p in pts) Offset(ox + s * (p.dx - minx), size.height - oy - s * (p.dy - miny))];
  }

  void dashed(Canvas canvas, Offset a, Offset b, Paint paint) {
    final len = (b - a).distance, dir = norm(b - a);
    for (var t = 0.0; t < len; t += 16) {
      canvas.drawLine(a + dir * t, a + dir * math.min(t + 9, len), paint);
    }
  }

  void arc(Canvas canvas, Offset c, double r, double angle, String text) {
    canvas.drawArc(Rect.fromCircle(center: c, radius: r), 0, -angle, false, stroke(accent, 2.5));
    label(canvas, text, c + Offset(r * math.cos(angle / 2), -r * math.sin(angle / 2)), Offset(math.cos(angle / 2), -math.sin(angle / 2)), 6);
  }

  @override
  void paint(Canvas canvas, Size size) {
    switch (fig) {
      case api.FigureDto_Right(:final adj, :final opp, :final angle, :final adjLabel, :final oppLabel, :final hypLabel):
        final p = fit([Offset.zero, Offset(adj, 0), Offset(adj, opp)], size, 46);
        final (a, c, b) = (p[0], p[1], p[2]);
        final inside = (a + b + c) / 3;
        final path = Path()
          ..moveTo(a.dx, a.dy)
          ..lineTo(c.dx, c.dy)
          ..lineTo(b.dx, b.dy)
          ..close();
        canvas.drawPath(path, Paint()..color = fill);
        canvas.drawPath(path, stroke(ink, 3));
        canvas.drawPath(
            Path()
              ..moveTo(c.dx - 14, c.dy)
              ..lineTo(c.dx - 14, c.dy - 14)
              ..lineTo(c.dx, c.dy - 14),
            stroke(ink, 2));
        if (angle.isNotEmpty) arc(canvas, a, 34, math.atan(opp / adj), angle);
        label(canvas, adjLabel, (a + c) / 2, const Offset(0, 1), 6);
        label(canvas, oppLabel, (c + b) / 2, const Offset(1, 0), 8);
        label(canvas, hypLabel, (a + b) / 2, outward(a, b, inside), 8);
      case api.FigureDto_Sas(:final a, :final b, :final deg, :final aLabel, :final bLabel, :final angle, :final third):
        final t = deg * math.pi / 180;
        final p = fit([Offset.zero, Offset(b, 0), Offset(a * math.cos(t), a * math.sin(t))], size, 46);
        final (v, pb, pa) = (p[0], p[1], p[2]);
        final inside = (v + pa + pb) / 3;
        canvas.drawPath(
            Path()
              ..moveTo(v.dx, v.dy)
              ..lineTo(pb.dx, pb.dy)
              ..lineTo(pa.dx, pa.dy)
              ..close(),
            Paint()..color = fill);
        canvas.drawPath(
            Path()
              ..moveTo(pa.dx, pa.dy)
              ..lineTo(v.dx, v.dy)
              ..lineTo(pb.dx, pb.dy),
            stroke(ink, 3));
        dashed(canvas, pb, pa, stroke(accent, 3));
        arc(canvas, v, 30, t, angle);
        label(canvas, bLabel, (v + pb) / 2, outward(v, pb, inside), 6);
        label(canvas, aLabel, (v + pa) / 2, outward(v, pa, inside), 8);
        label(canvas, third, (pb + pa) / 2, outward(pb, pa, inside), 8);
      case api.FigureDto_Unit(:final deg, label: final text):
        final r = math.max(math.min(size.width, size.height) / 2 - 30, 20.0);
        final c = Offset(size.width / 2, size.height / 2);
        final axes = stroke(const Color(0xFFBFC4D9), 1.5);
        canvas.drawLine(c - Offset(r + 18, 0), c + Offset(r + 18, 0), axes);
        canvas.drawLine(c - Offset(0, r + 18), c + Offset(0, r + 18), axes);
        canvas.drawCircle(c, r, stroke(ink, 2.5));
        label(canvas, '1', c + Offset(r, 0), const Offset(0.6, 0.8), 3);
        label(canvas, '−1', c - Offset(r, 0), const Offset(-0.6, 0.8), 3);
        final t = deg * math.pi / 180;
        final pt = c + Offset(r * math.cos(t), -r * math.sin(t));
        dashed(canvas, pt, Offset(pt.dx, c.dy), stroke(soft, 2));
        canvas.drawLine(c, pt, stroke(ink, 2.5));
        arc(canvas, c, 24, t, '');
        label(canvas, text, c + Offset(24 * math.cos(t / 2), -24 * math.sin(t / 2)), Offset(math.cos(t / 2), -math.sin(t / 2)), 8);
        canvas.drawCircle(pt, 6, Paint()..color = accent);
        label(canvas, '(cos, sin)', pt, norm(Offset(math.cos(t), -math.sin(t))), 8);
    }
  }

  @override
  bool shouldRepaint(covariant _FigurePainter old) => old.fig != fig;
}
