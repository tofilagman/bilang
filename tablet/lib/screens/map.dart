// The adventure map: every world and its levels, with stars and medals.
// Finishing a level opens the next.

import 'package:flutter/material.dart';

import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';
import 'level.dart';
import 'scores.dart';

class MapScreen extends StatefulWidget {
  final int player;
  const MapScreen({super.key, required this.player});

  @override
  State<MapScreen> createState() => _MapScreenState();
}

class _MapScreenState extends State<MapScreen> {
  final worlds = api.worlds();
  late api.Player p = api.players()[widget.player];

  void refresh() => setState(() => p = api.players()[widget.player]);

  Future<void> play(api.LevelInfo level) async {
    await Navigator.of(context).push(MaterialPageRoute(builder: (_) => LevelScreen(player: widget.player, level: level)));
    refresh();
  }

  void grownUps() => showModalBottomSheet(
        context: context,
        builder: (_) => StatefulBuilder(
          builder: (context, set) => Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('For grown-ups', style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold)),
                SwitchListTile(
                  title: const Text('Unlock all levels'),
                  value: p.unlockAll,
                  onChanged: (v) {
                    api.setUnlockAll(index: widget.player, on_: v);
                    refresh();
                    set(() {});
                  },
                ),
                ListTile(
                  title: const Text('Scratchpad'),
                  trailing: DropdownButton<String>(
                    value: p.scratchpad,
                    items: const [
                      DropdownMenuItem(value: 'auto', child: Text('High school worlds')),
                      DropdownMenuItem(value: 'always', child: Text('Every level')),
                      DropdownMenuItem(value: 'never', child: Text('Off')),
                    ],
                    onChanged: (v) {
                      api.setScratchpad(index: widget.player, mode: v!);
                      refresh();
                      set(() {});
                    },
                  ),
                ),
                ListTile(
                  leading: const Text('📊', style: TextStyle(fontSize: 24)),
                  title: const Text('Score sheet'),
                  onTap: () {
                    Navigator.of(context).pop();
                    Navigator.of(context).push(MaterialPageRoute(builder: (_) => ScoresScreen(player: widget.player)));
                  },
                ),
                ListTile(
                  leading: const Icon(Icons.delete_outline, color: Colors.red),
                  title: Text('Delete ${p.name}…', style: const TextStyle(color: Colors.red)),
                  onTap: () async {
                    final yes = await showDialog<bool>(
                      context: context,
                      builder: (c) => AlertDialog(
                        title: Text('Delete ${p.name}?'),
                        content: const Text("All of their stars will be gone. This can't be undone."),
                        actions: [
                          TextButton(onPressed: () => Navigator.of(c).pop(false), child: const Text('Keep')),
                          FilledButton(style: FilledButton.styleFrom(backgroundColor: Colors.red), onPressed: () => Navigator.of(c).pop(true), child: const Text('Delete')),
                        ],
                      ),
                    );
                    if (yes == true && context.mounted) {
                      api.deletePlayer(index: widget.player);
                      Navigator.of(context)
                        ..pop()
                        ..pop();
                    }
                  },
                ),
              ],
            ),
          ),
        ),
      );

  @override
  Widget build(BuildContext context) {
    final next = worlds.expand((w) => w.levels).where((l) => p.levels[l.flat].unlocked && p.levels[l.flat].stars == 0).firstOrNull;
    return Sky(
      child: Scaffold(
        appBar: AppBar(
          title: Text("${p.avatar} ${p.name}'s Adventure", style: const TextStyle(fontWeight: FontWeight.bold)),
          actions: [IconButton(icon: const Icon(Icons.menu), tooltip: 'For grown-ups', onPressed: grownUps)],
        ),
        body: SafeArea(
          child: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 1100),
              child: ListView(
                padding: const EdgeInsets.all(24),
                children: [
                  Row(
                    children: [
                      const Text('🦉', style: TextStyle(fontSize: 64)),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text('Hi ${p.name}! ${p.avatar}', style: const TextStyle(fontSize: 34, fontWeight: FontWeight.bold, color: prompt)),
                            Text(
                              next == null
                                  ? 'You finished every level! ⭐ ${p.totalStars} stars. Can you get 3 stars everywhere?'
                                  : (p.totalStars == 0 ? 'Let\'s start with "${next.title}"! Tap it below.' : 'You have ⭐ ${p.totalStars} stars! Next up: "${next.title}".'),
                              style: const TextStyle(fontSize: 19, color: inkSoft),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 20),
                  for (final (i, w) in worlds.indexed) _World(world: w, color: worldColors[i % worldColors.length], player: p, onPlay: play),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _World extends StatelessWidget {
  final api.WorldInfo world;
  final Color color;
  final api.Player player;
  final void Function(api.LevelInfo) onPlay;
  const _World({required this.world, required this.color, required this.player, required this.onPlay});

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.only(bottom: 22),
        child: CardBox(
          color: color,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                crossAxisAlignment: CrossAxisAlignment.baseline,
                textBaseline: TextBaseline.alphabetic,
                children: [
                  Text('${world.emoji} ${world.name}', style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold)),
                  const SizedBox(width: 12),
                  Text(world.grades, style: const TextStyle(fontSize: 18, color: inkSoft)),
                ],
              ),
              const SizedBox(height: 14),
              LayoutBuilder(
                builder: (context, c) {
                  final across = (c.maxWidth / 190).floor().clamp(2, 6);
                  final w = (c.maxWidth - (across - 1) * 14) / across;
                  return Wrap(
                    spacing: 14,
                    runSpacing: 14,
                    children: [for (final l in world.levels) SizedBox(width: w, height: 140, child: _Tile(level: l, state: player.levels[l.flat], onTap: () => onPlay(l)))],
                  );
                },
              ),
            ],
          ),
        ),
      );
}

class _Tile extends StatelessWidget {
  final api.LevelInfo level;
  final api.LevelState state;
  final VoidCallback onTap;
  const _Tile({required this.level, required this.state, required this.onTap});

  @override
  Widget build(BuildContext context) {
    final open = state.unlocked;
    return Opacity(
      opacity: open ? 1 : 0.5,
      child: GestureDetector(
        onTap: open ? onTap : null,
        child: CardBox(
          radius: 22,
          padding: const EdgeInsets.all(8),
          color: open ? Colors.white : Colors.white70,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(open ? '${level.flat + 1}' : '🔒', style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold)),
              Text(level.title, textAlign: TextAlign.center, maxLines: 2, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
              if (open) Text('${starText(state.stars)}${medalText(state.medal)}', style: const TextStyle(fontSize: 20, color: star)),
            ],
          ),
        ),
      ),
    );
  }
}
