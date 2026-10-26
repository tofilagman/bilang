// A confetti burst over the play area when a kid gets it right.

import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class _Bit {
  Offset pos, vel;
  double angle, spin;
  final Color color;
  _Bit(this.pos, this.vel, this.angle, this.spin, this.color);
}

class ConfettiController {
  _ConfettiState? _state;
  void burst(int count) => _state?._burst(count);
}

class Confetti extends StatefulWidget {
  final ConfettiController controller;
  const Confetti({super.key, required this.controller});

  @override
  State<Confetti> createState() => _ConfettiState();
}

class _ConfettiState extends State<Confetti> with SingleTickerProviderStateMixin {
  static const colors = [Color(0xFFFFB703), Color(0xFFFB8500), Color(0xFF45B047), Color(0xFF3F80E0), Color(0xFFE84D8C), Color(0xFF9966E6)];
  final bits = <_Bit>[];
  final rng = math.Random();
  late final Ticker ticker = createTicker(_tick);
  Duration last = Duration.zero;
  Size size = Size.zero;

  @override
  void initState() {
    super.initState();
    widget.controller._state = this;
  }

  void _burst(int count) {
    for (var i = 0; i < count; i++) {
      bits.add(_Bit(
        Offset(size.width * (0.3 + rng.nextDouble() * 0.4), size.height * 0.45),
        Offset((rng.nextDouble() * 2 - 1) * 420, -300 - rng.nextDouble() * 600),
        rng.nextDouble() * 6.3,
        (rng.nextDouble() * 2 - 1) * 8,
        colors[rng.nextInt(colors.length)],
      ));
    }
    if (!ticker.isActive) {
      last = Duration.zero;
      ticker.start();
    }
  }

  void _tick(Duration now) {
    final dt = last == Duration.zero ? 0.016 : math.min((now - last).inMicroseconds / 1e6, 0.05);
    last = now;
    for (final b in bits) {
      b.vel = Offset(b.vel.dx * 0.99, b.vel.dy + 1400 * dt);
      b.pos += b.vel * dt;
      b.angle += b.spin * dt;
    }
    bits.removeWhere((b) => b.pos.dy > size.height + 20);
    if (bits.isEmpty) ticker.stop();
    setState(() {});
  }

  @override
  void dispose() {
    ticker.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => IgnorePointer(
        child: LayoutBuilder(builder: (context, c) {
          size = c.biggest;
          return CustomPaint(size: c.biggest, painter: _Painter(bits));
        }),
      );
}

class _Painter extends CustomPainter {
  final List<_Bit> bits;
  _Painter(this.bits);

  @override
  void paint(Canvas canvas, Size size) {
    for (final b in bits) {
      canvas.save();
      canvas.translate(b.pos.dx, b.pos.dy);
      canvas.rotate(b.angle);
      canvas.drawRect(const Rect.fromLTWH(-5, -3, 10, 6), Paint()..color = b.color);
      canvas.restore();
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter old) => true;
}
