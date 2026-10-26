#!/usr/bin/env bash
# Package Bilang for Windows (x64) from the MSYS2 UCRT64 shell: bilang.exe with
# the GTK 4 / libadwaita DLLs it needs, share/ (schemas, Adwaita + hicolor
# icons, licenses) and the SVG image loader, as a portable zip and an NSIS
# installer. Needs these MSYS2 packages (prefix mingw-w64-ucrt-x86_64-):
#   gtk4 libadwaita librsvg adwaita-icon-theme hicolor-icon-theme rust pkgconf gcc nsis
# plus zip.
#   packaging/windows/package.sh   → target/Bilang-<ver>-windows-x64.zip, target/Bilang-<ver>-windows-x64-setup.exe
set -euo pipefail
cd "$(dirname "$0")/../.."

cargo build --release --locked
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
NAME=Bilang-$VERSION-windows-x64
M=${MINGW_PREFIX:-/ucrt64}
OUT=$PWD/target
STAGE=$OUT/package/Bilang
rm -rf "$OUT/package"
mkdir -p "$STAGE"
cp target/release/bilang.exe "$STAGE/"

# the SVG image loader (icons) and a loaders.cache template main.rs fills in
L=lib/gdk-pixbuf-2.0/2.10.0
mkdir -p "$STAGE/$L/loaders"
SVG=$(find "$M/$L/loaders" -iname '*svg*.dll' -print -quit)
cp "$SVG" "$STAGE/$L/loaders/"
gdk-pixbuf-query-loaders "$STAGE/$L/loaders/$(basename "$SVG")" \
  | grep -v '^# ' | sed -E 's|"[^"]*[/\\]+loaders[/\\]+|"@LOADERS_DIR@/|' > "$STAGE/$L/loaders.cache.in"
grep -q '@LOADERS_DIR@' "$STAGE/$L/loaders.cache.in" || { echo "loaders.cache.in has no loader" >&2; cat "$STAGE/$L/loaders.cache.in" >&2; exit 1; }

# every DLL the exe and the loader pull in from MSYS2 (the rest are Windows')
for f in "$STAGE/bilang.exe" "$STAGE/$L/loaders/"*.dll; do
  ldd "$f" | awk -v m="$M/bin/" 'index(tolower($3), tolower(m)) == 1 { print $3 }'
done | sort -u | while read -r dll; do cp "$dll" "$STAGE/"; done

# GSettings schemas, icon themes (no cursors: Windows draws its own), our icon
mkdir -p "$STAGE/share/glib-2.0/schemas" "$STAGE/share/icons/Adwaita" "$STAGE/share/licenses"
cp "$M"/share/glib-2.0/schemas/org.gtk.gtk4.*.gschema.xml "$STAGE/share/glib-2.0/schemas/"
glib-compile-schemas "$STAGE/share/glib-2.0/schemas"
cp -r "$M/share/icons/hicolor" "$STAGE/share/icons/"
(cd "$M/share/icons/Adwaita" && tar --exclude=cursors -cf - .) | tar -xf - -C "$STAGE/share/icons/Adwaita"
install -Dm644 packaging/icon.svg "$STAGE/share/icons/hicolor/scalable/apps/io.r3al.bilang.svg"
for t in Adwaita hicolor; do gtk4-update-icon-cache -q -f -t "$STAGE/share/icons/$t" || true; done

# licenses: Bilang's, and those of the libraries shipped with it
cp LICENSE "$STAGE/LICENSE.txt"
for p in gtk4 libadwaita glib2 gdk-pixbuf2 librsvg pango cairo harfbuzz fribidi graphene libepoxy; do
  [[ -d $M/share/licenses/$p ]] && cp -r "$M/share/licenses/$p" "$STAGE/share/licenses/"
done

(cd "$OUT/package" && rm -f "$OUT/$NAME.zip" && zip -qr9 "$OUT/$NAME.zip" Bilang)
echo "Built $OUT/$NAME.zip ($(du -sh "$OUT/$NAME.zip" | cut -f1), $(ls "$STAGE"/*.dll | wc -l) DLLs)"

makensis -V2 -DVERSION="$VERSION" -DSTAGE="$(cygpath -w "$STAGE")" -DOUTFILE="$(cygpath -w "$OUT/$NAME-setup.exe")" \
  -DICON="$(cygpath -w "$PWD/packaging/windows/bilang.ico")" packaging/windows/installer.nsi
echo "Built $OUT/$NAME-setup.exe ($(du -sh "$OUT/$NAME-setup.exe" | cut -f1))"
