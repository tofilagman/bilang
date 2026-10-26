// The big on-screen number pad, with the extra keys each answer needs.

import 'package:flutter/material.dart';

import '../theme.dart';

class Keypad extends StatelessWidget {
  final List<String> extras;
  final void Function(String key) onKey;
  final double size;
  const Keypad({super.key, required this.extras, required this.onKey, this.size = 66});

  Widget _key(String k, {bool extra = false, double? width}) => Padding(
        padding: const EdgeInsets.all(4),
        child: SizedBox(
          width: width ?? size,
          height: size * 0.82,
          child: Material(
            color: extra ? keyExtra : Colors.white,
            borderRadius: BorderRadius.circular(16),
            elevation: 1.5,
            child: InkWell(
              borderRadius: BorderRadius.circular(16),
              onTap: () => onKey(k),
              child: Center(child: Text(k, style: TextStyle(fontSize: size * 0.4, fontWeight: FontWeight.bold, color: prompt))),
            ),
          ),
        ),
      );

  @override
  Widget build(BuildContext context) => Row(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (final row in const [
                ['7', '8', '9'],
                ['4', '5', '6'],
                ['1', '2', '3'],
              ])
                Row(mainAxisSize: MainAxisSize.min, children: [for (final k in row) _key(k)]),
              Row(mainAxisSize: MainAxisSize.min, children: [_key('0', width: size * 2 + 8), _key('⌫', extra: true)]),
            ],
          ),
          if (extras.isNotEmpty) Column(mainAxisSize: MainAxisSize.min, children: [for (final k in extras) _key(k, extra: true)]),
        ],
      );
}
