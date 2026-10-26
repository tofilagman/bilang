// Bilang's look, matching the desktop app: a warm sky background, white
// rounded cards, Comic Neue, and one color per world.

import 'package:flutter/material.dart';

const ink = Color(0xFF2D2D5A);
const inkSoft = Color(0xFF5A5A8A);
const prompt = Color(0xFF3A3A7A);
const go = Color(0xFF43B047);
const next = Color(0xFF3F7FE0);
const accent = Color(0xFFFB8500);
const hintYellow = Color(0xFFFFF3C4);
const hintBlue = Color(0xFFDFF3FF);
const keyExtra = Color(0xFFFFF0C2);
const star = Color(0xFFF2A900);

/// Each world's card color, in map order.
const worldColors = [
  Color(0xFFD8F5D2), // Number Garden
  Color(0xFFFFE3BD), // Times Jungle
  Color(0xFFDDE7FF), // Column City
  Color(0xFFE6DBFA), // MDAS Castle
  Color(0xFFD3EBFD), // Fraction Falls
  Color(0xFFFFD8C9), // Integer Volcano
  Color(0xFFE1E8F0), // Algebra Peak
  Color(0xFFE2DDFB), // Equation Galaxy
  Color(0xFFFCE7C8), // Triangle Island
  Color(0xFFFBD9E6), // Chance Carnival
];

const avatars = ['🦊', '🐼', '🐯', '🐸', '🦄', '🐵', '🐧', '🐨', '🦁', '🐰', '🐶', '🐱', '🐢', '🐙', '🦖', '🐝'];

ThemeData bilangTheme() {
  final base = ThemeData(
    useMaterial3: true,
    colorScheme: ColorScheme.fromSeed(seedColor: next, brightness: Brightness.light),
    fontFamily: 'ComicNeue',
  );
  return base.copyWith(
    scaffoldBackgroundColor: Colors.transparent,
    textTheme: base.textTheme.apply(bodyColor: ink, displayColor: ink),
    appBarTheme: const AppBarTheme(backgroundColor: Colors.transparent, elevation: 0, foregroundColor: ink, centerTitle: true),
  );
}

/// The sky behind every screen.
class Sky extends StatelessWidget {
  final Widget child;
  const Sky({super.key, required this.child});

  @override
  Widget build(BuildContext context) => DecoratedBox(
        decoration: const BoxDecoration(
          gradient: LinearGradient(begin: Alignment.topCenter, end: Alignment.bottomCenter, colors: [Color(0xFFFFF8E1), Color(0xFFE3F2FD)]),
        ),
        child: child,
      );
}

/// A white rounded card with a soft shadow.
class CardBox extends StatelessWidget {
  final Widget child;
  final EdgeInsets padding;
  final Color color;
  final double radius;
  const CardBox({super.key, required this.child, this.padding = const EdgeInsets.all(20), this.color = Colors.white, this.radius = 28});

  @override
  Widget build(BuildContext context) => AnimatedContainer(
        duration: const Duration(milliseconds: 250),
        padding: padding,
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(radius),
          boxShadow: const [BoxShadow(color: Color(0x1A000000), blurRadius: 14, offset: Offset(0, 4))],
        ),
        child: child,
      );
}

String starText(int n) => List.generate(3, (i) => i < n ? '★' : '☆').join();

String medalText(int m) => m >= 100 ? ' 🏆' : (m >= 50 ? ' 🥈' : '');

/// Kiko and a speech bubble.
class Kiko extends StatelessWidget {
  final String face;
  final Widget bubble;
  const Kiko({super.key, this.face = '🦉', required this.bubble});

  @override
  Widget build(BuildContext context) => Row(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Text(face, style: const TextStyle(fontSize: 56)),
          const SizedBox(width: 12),
          Flexible(
            child: CardBox(padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14), radius: 22, child: bubble),
          ),
        ],
      );
}
