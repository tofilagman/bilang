// The grown-ups' score sheet, like the desktop's src/scores.rs: totals, this
// week, what to practice together, every level, and each session with the
// questions the child needed help on.

import 'dart:convert';
import 'dart:typed_data';

import 'package:file_saver/file_saver.dart';
import 'package:flutter/material.dart';

import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';
import '../widgets/math_text.dart';

const _weak = 70;

class ScoresScreen extends StatefulWidget {
  final int player;
  const ScoresScreen({super.key, required this.player});

  @override
  State<ScoresScreen> createState() => _ScoresScreenState();
}

class _ScoresScreenState extends State<ScoresScreen> {
  late int player = widget.player;
  final worlds = api.worlds();

  (String, String) titleOf(String id) {
    for (final w in worlds) {
      for (final l in w.levels) {
        if (l.id == id) return (w.name, l.title);
      }
    }
    return ('', id);
  }

  Future<void> export(api.Player p) async {
    final csv = api.historyCsv(index: player);
    await FileSaver.instance.saveAs(name: '${p.name} - Bilang score sheet', bytes: Uint8List.fromList(utf8.encode(csv)), fileExtension: 'csv', mimeType: MimeType.csv);
  }

  Widget stat(String value, String caption) => SizedBox(
        width: 200,
        child: CardBox(
          radius: 20,
          padding: const EdgeInsets.symmetric(vertical: 14, horizontal: 10),
          child: Column(children: [
            Text(value, style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold)),
            Text(caption, style: const TextStyle(fontSize: 15, color: inkSoft)),
          ]),
        ),
      );

  Widget section(String t) => Padding(padding: const EdgeInsets.only(top: 26, bottom: 10), child: Text(t, style: const TextStyle(fontSize: 26, fontWeight: FontWeight.bold)));

  @override
  Widget build(BuildContext context) {
    final players = api.players();
    final p = players[player];
    final runs = api.historyOf(index: player);
    final practice = runs.where((r) => !r.lesson).toList();
    final stats = {for (final s in api.levelStats(index: player)) s.id: s};
    final now = DateTime.now().millisecondsSinceEpoch ~/ 1000;
    final days = api.week(index: player, now: now);
    final answered = practice.fold<int>(0, (a, r) => a + r.answered), first = practice.fold<int>(0, (a, r) => a + r.firstTry);
    final seconds = runs.fold<int>(0, (a, r) => a + r.seconds);
    final total = worlds.fold<int>(0, (a, w) => a + w.levels.length);
    final done = p.levels.where((l) => l.stars > 0).length;
    final weak = stats.values.where((s) => s.recent != null && s.recent! < _weak).toList()..sort((a, b) => a.recent!.compareTo(b.recent!));
    final maxQ = days.fold<int>(1, (a, d) => d.questions > a ? d.questions : a);
    final lastAt = runs.isEmpty ? null : runs.map((r) => r.at).reduce((a, b) => a > b ? a : b);
    final newest = [...runs]..sort((a, b) => b.at.compareTo(a.at));

    return Sky(
      child: Scaffold(
        appBar: AppBar(
          title: const Text('📊 Score sheet', style: TextStyle(fontWeight: FontWeight.bold)),
          actions: [
            DropdownButton<int>(
              value: player,
              underline: const SizedBox(),
              items: [for (final q in players) DropdownMenuItem(value: q.index, child: Text('${q.avatar} ${q.name}', style: const TextStyle(fontSize: 18)))],
              onChanged: (v) => setState(() => player = v!),
            ),
            TextButton(onPressed: () => export(p), child: const Text('⬇️ Export CSV', style: TextStyle(fontSize: 17))),
          ],
        ),
        body: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 980),
            child: ListView(
              padding: const EdgeInsets.all(24),
              children: [
                Row(children: [
                  Text(p.avatar, style: const TextStyle(fontSize: 64)),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                      Text("${p.name}'s score sheet", style: const TextStyle(fontSize: 34, fontWeight: FontWeight.bold, color: prompt)),
                      Text(
                        lastAt != null
                            ? 'Last played ${api.when(at: lastAt, fmt: '%A, %B %e at %l:%M %p')}'
                            : (p.solved > 0 ? 'Sessions are recorded from now on. Earlier play shows up as stars only.' : 'No practice yet. Sessions will show up here as they play.'),
                        style: const TextStyle(fontSize: 18, color: inkSoft),
                      ),
                    ]),
                  ),
                ]),
                const SizedBox(height: 16),
                Wrap(spacing: 12, runSpacing: 12, children: [
                  stat('${p.totalStars} / ${total * 3}', '⭐ stars'),
                  stat('$done / $total', '🏁 levels finished'),
                  stat('${p.solved}', '✏️ questions answered'),
                  stat(answered > 0 ? '${first * 100 ~/ answered}%' : '—', '🎯 right on first try'),
                  stat(api.duration(seconds: seconds), '⏱️ time practicing'),
                  stat('${p.bestStreak}', '🔥 best streak'),
                ]),
                section('📅 This week: ${days.fold<int>(0, (a, d) => a + d.questions)} questions · ${api.duration(seconds: days.fold<int>(0, (a, d) => a + d.seconds))}'),
                CardBox(
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.end,
                    children: [
                      for (final (i, d) in days.indexed)
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 8),
                          child: Tooltip(
                            message: '${api.when(at: d.start, fmt: '%A %b %e')}: ${d.questions} questions, ${api.duration(seconds: d.seconds)}',
                            child: Column(children: [
                              Container(
                                width: 44,
                                height: d.questions == 0 ? 2 : 6 + 114 * d.questions / maxQ,
                                decoration: BoxDecoration(color: d.questions == 0 ? const Color(0x263A3A7A) : next, borderRadius: const BorderRadius.vertical(top: Radius.circular(4))),
                              ),
                              const SizedBox(height: 6),
                              Text(i == days.length - 1 ? 'Today' : api.when(at: d.start, fmt: '%a'), style: TextStyle(fontSize: 15, fontWeight: i == days.length - 1 ? FontWeight.bold : FontWeight.normal)),
                            ]),
                          ),
                        ),
                    ],
                  ),
                ),
                section('💡 Worth practicing together'),
                if (weak.isEmpty)
                  Text(practice.isEmpty ? "Once they've played a few sessions, levels that need more work will show up here." : 'Nothing worrying right now. Every recent session went well! 🌟', style: const TextStyle(fontSize: 18, color: inkSoft))
                else
                  for (final s in weak)
                    Card(
                      child: ListTile(
                        title: Text('${titleOf(s.id).$2} (${titleOf(s.id).$1})', style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                        subtitle: Text('${s.recent}% right on the first try lately. Try "📖 Show me how" together, then a short round of 10.'),
                        trailing: const Text('⚠️', style: TextStyle(fontSize: 22)),
                      ),
                    ),
                section('🗺️ Levels'),
                for (final w in worlds)
                  if (w.levels.any((l) => p.levels[l.flat].stars > 0 || stats.containsKey(l.id))) ...[
                    Padding(
                      padding: const EdgeInsets.only(top: 8, bottom: 4),
                      child: Text('${w.emoji} ${w.name} · ${w.levels.where((l) => p.levels[l.flat].stars > 0).length} of ${w.levels.length} done', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: prompt)),
                    ),
                    for (final l in w.levels)
                      if (p.levels[l.flat].stars > 0 || stats.containsKey(l.id))
                        Card(
                          child: ListTile(
                            title: Text(l.title, style: const TextStyle(fontSize: 18)),
                            subtitle: Text(switch (stats[l.id]) {
                              final s? when s.runs > 0 =>
                                'Played ${s.runs}× · ${s.answered > 0 ? '${s.firstTry * 100 ~/ s.answered}%' : '—'} right on the first try · ${api.duration(seconds: s.seconds)} · last ${api.when(at: s.lastAt, fmt: '%b %e')}',
                              final s? => 'Did the lesson · last ${api.when(at: s.lastAt, fmt: '%b %e')}',
                              null => 'Finished before the score sheet started recording',
                            }),
                            trailing: Text(p.levels[l.flat].stars > 0 ? '${starText(p.levels[l.flat].stars)}${medalText(p.levels[l.flat].medal)}' : 'In progress',
                                style: TextStyle(fontSize: 20, color: p.levels[l.flat].stars > 0 ? star : inkSoft)),
                          ),
                        ),
                  ],
                section('🕑 Recent sessions'),
                if (runs.isEmpty) const Text('Nothing recorded yet.', style: TextStyle(fontSize: 18, color: inkSoft)),
                for (final r in newest.take(30))
                  Card(
                    child: r.lesson
                        ? ListTile(
                            title: Text('${api.when(at: r.at, fmt: '%b %e, %l:%M %p')} · ${titleOf(r.level).$2}', style: const TextStyle(fontSize: 18)),
                            subtitle: Text('📖 Learned it with Kiko (Show me how) · ${api.duration(seconds: r.seconds)}'),
                          )
                        : ExpansionTile(
                            title: Text('${api.when(at: r.at, fmt: '%b %e, %l:%M %p')} · ${titleOf(r.level).$2}', style: const TextStyle(fontSize: 18)),
                            subtitle: Text('${r.firstTry} of ${r.answered} right on the first try · '
                                '${r.finished ? starText(r.stars) : 'stopped at ${r.answered} of ${r.total}'} · ${api.duration(seconds: r.seconds)} · ${r.hints} hint${r.hints == 1 ? '' : 's'}'),
                            children: [
                              if (r.missed.isEmpty) const ListTile(title: Text('Every question right on the first try! 🎉')),
                              for (final m in r.missed)
                                ListTile(
                                  title: MathText(m.question, style: const TextStyle(fontSize: 18)),
                                  subtitle: Text('Right answer ${m.answer} · ${m.tries.isEmpty ? 'used hints' : 'first answered ${m.tries.join(', ')}'}${m.hints == 0 ? '' : ' · ${m.hints} hint${m.hints == 1 ? '' : 's'}'}'),
                                ),
                            ],
                          ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
