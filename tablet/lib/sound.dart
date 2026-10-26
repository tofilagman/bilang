// Little synthesized chimes, like the desktop's src/sound.rs, made in code
// so there are no audio files to ship.

import 'dart:io';
import 'dart:math' as math;
import 'dart:typed_data';

import 'package:audioplayers/audioplayers.dart';

import 'src/rust/api/bilang.dart' as api;

class Sounds {
  static bool on = api.soundOn();
  static final _files = <String, String>{};
  static final _players = List.generate(4, (_) => AudioPlayer()..setPlayerMode(PlayerMode.lowLatency));
  static int _next = 0;

  static void toggle() {
    on = !on;
    api.setSound(on_: on);
  }

  // (semitones from C5, start in seconds, length in seconds)
  static const _cues = {
    'right': [(4.0, 0.0, 0.25), (7.0, 0.08, 0.25), (12.0, 0.16, 0.45)],
    'almost': [(7.0, 0.0, 0.2), (9.0, 0.1, 0.3)],
    'wrong': [(-5.0, 0.0, 0.25), (-8.0, 0.14, 0.35)],
    'tap': [(12.0, 0.0, 0.06)],
    'hint': [(9.0, 0.0, 0.15), (14.0, 0.07, 0.3)],
    'done': [(0.0, 0.0, 0.3), (4.0, 0.12, 0.3), (7.0, 0.24, 0.3), (12.0, 0.36, 0.7), (16.0, 0.36, 0.7)],
  };

  /// A short WAV of the cue's notes: sine plus an octave, quick attack, soft decay.
  static Uint8List _wav(String cue) {
    const rate = 22050;
    final notes = _cues[cue]!;
    final gain = cue == 'tap' ? 0.08 : 0.18;
    final len = notes.map((n) => n.$2 + n.$3).reduce(math.max);
    final samples = Float64List((len * rate).ceil());
    for (final (semi, start, dur) in notes) {
      final f = 523.25 * math.pow(2, semi / 12);
      final s0 = (start * rate).round(), n = (dur * rate).round();
      for (var i = 0; i < n && s0 + i < samples.length; i++) {
        final t = i / rate, p = i / n;
        final env = math.min(p * 40, 1.0) * (1 - p) * (1 - p);
        samples[s0 + i] += (math.sin(2 * math.pi * f * t) + 0.3 * math.sin(4 * math.pi * f * t)) * env * gain;
      }
    }
    final bytes = ByteData(44 + samples.length * 2);
    void str(int at, String s) {
      for (var i = 0; i < s.length; i++) {
        bytes.setUint8(at + i, s.codeUnitAt(i));
      }
    }

    str(0, 'RIFF');
    bytes.setUint32(4, 36 + samples.length * 2, Endian.little);
    str(8, 'WAVEfmt ');
    bytes
      ..setUint32(16, 16, Endian.little)
      ..setUint16(20, 1, Endian.little)
      ..setUint16(22, 1, Endian.little)
      ..setUint32(24, rate, Endian.little)
      ..setUint32(28, rate * 2, Endian.little)
      ..setUint16(32, 2, Endian.little)
      ..setUint16(34, 16, Endian.little);
    str(36, 'data');
    bytes.setUint32(40, samples.length * 2, Endian.little);
    for (var i = 0; i < samples.length; i++) {
      bytes.setInt16(44 + i * 2, (samples[i].clamp(-0.9, 0.9) * 32767).round(), Endian.little);
    }
    return bytes.buffer.asUint8List();
  }

  /// Writes each cue to a WAV in [dir] at startup: low-latency playback on
  /// Android only takes files, not bytes.
  static Future<void> init(String dir) async {
    await Directory('$dir/sounds').create(recursive: true);
    for (final cue in _cues.keys) {
      final f = File('$dir/sounds/$cue.wav');
      await f.writeAsBytes(_wav(cue));
      _files[cue] = f.path;
    }
  }

  static void _play(String cue) {
    final path = _files[cue];
    if (!on || path == null) return;
    final p = _players[_next++ % _players.length];
    p.play(DeviceFileSource(path)).catchError((_) {});
  }

  static void tap() => _play('tap');
  static void right() => _play('right');
  static void almost() => _play('almost');
  static void wrong() => _play('wrong');
  static void hint() => _play('hint');
  static void levelDone() => _play('done');
}
