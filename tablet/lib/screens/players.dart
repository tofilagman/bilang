// "Who's playing?": each child picks an animal buddy; no passwords.

import 'package:flutter/material.dart';

import '../src/rust/api/bilang.dart' as api;
import '../theme.dart';
import 'map.dart';
import 'scores.dart';

class PlayersScreen extends StatefulWidget {
  const PlayersScreen({super.key});

  @override
  State<PlayersScreen> createState() => _PlayersScreenState();
}

class _PlayersScreenState extends State<PlayersScreen> {
  List<api.Player> players = api.players();

  void refresh() => setState(() => players = api.players());

  Future<void> open(int index) async {
    await Navigator.of(context).push(MaterialPageRoute(builder: (_) => MapScreen(player: index)));
    refresh();
  }

  Future<void> newPlayer() async {
    final made = await showDialog<int>(context: context, builder: (_) => const NewPlayerDialog());
    if (made != null) open(made);
  }

  @override
  Widget build(BuildContext context) => Sky(
        child: Scaffold(
          appBar: AppBar(
            title: const Text('Bilang', style: TextStyle(fontWeight: FontWeight.bold)),
            leading: players.isEmpty
                ? null
                : TextButton(
                    onPressed: () => Navigator.of(context).push(MaterialPageRoute(builder: (_) => const ScoresScreen(player: 0))),
                    child: const Text('📊', style: TextStyle(fontSize: 24)),
                  ),
            leadingWidth: 64,
          ),
          body: SafeArea(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(32),
              child: Column(
                children: [
                  const Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text('🦉', style: TextStyle(fontSize: 80)),
                      SizedBox(width: 16),
                      Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text("Hi! I'm Kiko.", style: TextStyle(fontSize: 40, fontWeight: FontWeight.bold, color: prompt)),
                          Text("Let's go on a math adventure! Who's playing today?", style: TextStyle(fontSize: 20, color: inkSoft)),
                        ],
                      ),
                    ],
                  ),
                  const SizedBox(height: 32),
                  Wrap(
                    spacing: 20,
                    runSpacing: 20,
                    alignment: WrapAlignment.center,
                    children: [
                      for (final p in players)
                        _PlayerCard(avatar: p.avatar, name: p.name, sub: '⭐ ${p.totalStars}', onTap: () => open(p.index)),
                      _PlayerCard(avatar: '➕', name: 'New player', sub: '', onTap: newPlayer),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      );
}

class _PlayerCard extends StatelessWidget {
  final String avatar, name, sub;
  final VoidCallback onTap;
  const _PlayerCard({required this.avatar, required this.name, required this.sub, required this.onTap});

  @override
  Widget build(BuildContext context) => GestureDetector(
        onTap: onTap,
        child: SizedBox(
          width: 180,
          height: 214,
          child: CardBox(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(avatar, style: const TextStyle(fontSize: 72)),
                Text(name, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
                if (sub.isNotEmpty) Text(sub, style: const TextStyle(fontSize: 18, color: inkSoft)),
              ],
            ),
          ),
        ),
      );
}

class NewPlayerDialog extends StatefulWidget {
  const NewPlayerDialog({super.key});

  @override
  State<NewPlayerDialog> createState() => _NewPlayerDialogState();
}

class _NewPlayerDialogState extends State<NewPlayerDialog> {
  final name = TextEditingController();
  String avatar = avatars.first;

  void create() {
    final n = name.text.trim();
    if (n.isEmpty) return;
    Navigator.of(context).pop(api.addPlayer(name: n, avatar: avatar));
  }

  @override
  Widget build(BuildContext context) => AlertDialog(
        title: const Text('New player', style: TextStyle(fontWeight: FontWeight.bold)),
        scrollable: true,
        content: SizedBox(
          width: 520,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text("What's your name? Pick an animal buddy too!", style: TextStyle(fontSize: 18)),
              const SizedBox(height: 12),
              TextField(
                controller: name,
                maxLength: 20,
                style: const TextStyle(fontSize: 26, fontWeight: FontWeight.bold),
                decoration: const InputDecoration(hintText: 'Your name', border: OutlineInputBorder()),
                onSubmitted: (_) => create(),
                onChanged: (_) => setState(() {}),
              ),
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: [
                  for (final a in avatars)
                    ChoiceChip(
                      label: Text(a, style: const TextStyle(fontSize: 32)),
                      selected: a == avatar,
                      onSelected: (_) => setState(() => avatar = a),
                    ),
                ],
              ),
            ],
          ),
        ),
        actions: [
          TextButton(onPressed: () => Navigator.of(context).pop(), child: const Text('Cancel')),
          FilledButton(onPressed: name.text.trim().isEmpty ? null : create, child: const Text("Let's go!")),
        ],
      );
}
