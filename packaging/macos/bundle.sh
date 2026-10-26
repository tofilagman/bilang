#!/bin/zsh
# Build a self-contained Bilang.app and a .dmg: the Homebrew GTK 4 / libadwaita
# dylibs, schemas, icons and image loaders go inside the app, ad-hoc signed
# (first open on another Mac: right-click → Open).
#   packaging/macos/bundle.sh          → target/Bilang.app, target/Bilang-<ver>-mac-<arch>.dmg
#   packaging/macos/bundle.sh --no-dmg
set -euo pipefail
cd "$(dirname "$0")/../.."

cargo build --release --locked
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
ARCH=$(uname -m)

APP="target/Bilang.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp target/release/bilang "$APP/Contents/MacOS/bilang"
cp packaging/macos/bilang.icns "$APP/Contents/Resources/"
MINOS=$(python3 packaging/macos/bundle_mac.py "$APP" | tail -1)

cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>Bilang</string>
  <key>CFBundleDisplayName</key><string>Bilang</string>
  <key>CFBundleIdentifier</key><string>io.r3al.bilang</string>
  <key>CFBundleExecutable</key><string>bilang</string>
  <key>CFBundleIconFile</key><string>bilang</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>LSMinimumSystemVersion</key><string>$MINOS</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>LSApplicationCategoryType</key><string>public.app-category.education</string>
</dict>
</plist>
PLIST

# install_name_tool broke the Homebrew signatures; re-sign inside out
find "$APP/Contents" \( -name '*.dylib' -o -name '*.so' \) -exec codesign --force --sign - {} + 2>/dev/null
codesign --force --sign - "$APP"
codesign --verify --deep --strict "$APP"
echo "Built $APP ($VERSION, $ARCH, macOS ≥ $MINOS, $(du -sh "$APP" | cut -f1))"

[[ ${1:-} == --no-dmg ]] && exit 0
DMG="target/Bilang-$VERSION-mac-$ARCH.dmg"
STAGE=$(mktemp -d)
trap 'rm -rf "$STAGE"' EXIT
ditto "$APP" "$STAGE/Bilang.app"
ln -s /Applications "$STAGE/Applications"
rm -f "$DMG"
hdiutil create -quiet -volname "Bilang" -srcfolder "$STAGE" -fs HFS+ -format UDZO "$DMG"
echo "Built $DMG ($(du -sh "$DMG" | cut -f1))"
