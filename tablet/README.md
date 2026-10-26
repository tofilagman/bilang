# Bilang for Android tablets

The touch version of Bilang. It's a Flutter app on top of the same Rust engine as the
desktop app (`../core`), bridged with flutter_rust_bridge.

```bash
flutter run                 # on a connected tablet or emulator
flutter build apk --release
```

- `rust/src/api/bilang.rs` is the bridge. After changing it, regenerate the Dart side
  with `flutter_rust_bridge_codegen generate`.
- `lib/screens/` has the players, map, level and score sheet screens.
- `lib/widgets/` has the number pad, stacked pad, formulas, diagrams and scratchpad.

See the main [README](../README.md) for downloads and the rest of the project.
