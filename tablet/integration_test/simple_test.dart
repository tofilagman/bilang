// Starts the app on the device, on top of the Rust engine, and checks the
// first screen comes up.

import 'package:bilang/main.dart';
import 'package:bilang/src/rust/api/bilang.dart' as api;
import 'package:bilang/src/rust/frb_generated.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await RustLib.init();
    api.openStore(dir: (await getTemporaryDirectory()).path);
  });
  testWidgets('the players screen greets you', (tester) async {
    await tester.pumpWidget(const BilangApp());
    expect(find.textContaining("I'm Kiko"), findsOneWidget);
    expect(api.worlds().length, 10);
  });
}
