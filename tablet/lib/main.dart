// Bilang for Android tablets: the same math adventure as the desktop app,
// on top of the same Rust engine (core/), with touch-first screens.

import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

import 'screens/players.dart';
import 'sound.dart';
import 'src/rust/api/bilang.dart' as api;
import 'src/rust/frb_generated.dart';
import 'theme.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  final dir = await getApplicationSupportDirectory();
  api.openStore(dir: dir.path);
  await Sounds.init(dir.path);
  runApp(const BilangApp());
}

class BilangApp extends StatelessWidget {
  const BilangApp({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
        title: 'Bilang',
        debugShowCheckedModeBanner: false,
        theme: bilangTheme(),
        home: const PlayersScreen(),
      );
}
