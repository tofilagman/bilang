#!/usr/bin/env python3
"""Make "Bilang.app" self-contained: copy every Homebrew dylib the
binary pulls in (recursively) into Contents/Frameworks and rewrite the load
paths with install_name_tool, then add what GTK reads at run time — compiled
GSettings schemas, the Adwaita/hicolor icon themes and the gdk-pixbuf loaders.

    bundle_mac.py <App.app>      (Contents/MacOS/bilang already in place)

main.rs points GTK at Contents/Resources when it finds the schemas there.
"""
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

BREW = Path(subprocess.run(["brew", "--prefix"], capture_output=True, text=True).stdout.strip() or "/opt/homebrew")
PIXBUF_VER = "2.10.0"


def otool_deps(path):
    out = subprocess.run(["otool", "-L", str(path)], capture_output=True, text=True, check=True).stdout
    deps = [l.strip().split(" (")[0] for l in out.splitlines()[1:] if l.strip()]
    own = subprocess.run(["otool", "-D", str(path)], capture_output=True, text=True).stdout.splitlines()[1:]
    return [d for d in deps if d not in own]  # a dylib lists its own install name first


def rpaths(path):
    out = subprocess.run(["otool", "-l", str(path)], capture_output=True, text=True, check=True).stdout
    return re.findall(r"cmd LC_RPATH\n\s+cmdsize \d+\n\s+path (\S+)", out)


def is_system(dep):
    return dep.startswith("/usr/lib/") or dep.startswith("/System/")


def resolve(dep, referrer):
    """The real file behind a load command (absolute, @rpath, @loader_path)."""
    if dep.startswith("@rpath/"):
        name = dep[len("@rpath/"):]
        for rp in rpaths(referrer):
            rp = rp.replace("@loader_path", str(Path(referrer).parent))
            cand = Path(rp) / name
            if cand.exists():
                return cand.resolve()
        cand = BREW / "lib" / name
        if cand.exists():
            return cand.resolve()
        raise SystemExit(f"can't resolve {dep} from {referrer}")
    if dep.startswith("@loader_path/"):
        return (Path(referrer).parent / dep[len("@loader_path/"):]).resolve()
    if dep.startswith("@executable_path/"):
        raise SystemExit(f"unexpected {dep} in {referrer}")
    return Path(dep).resolve()


def fix(copy, orig, name_of, in_fw):
    """Point every non-system dependency of `copy` at Contents/Frameworks
    (deps are resolved against `orig`, where @rpath/@loader_path make sense)."""
    args = []
    for dep in otool_deps(copy):
        if is_system(dep) or dep.startswith("@executable_path/"):
            continue
        args += ["-change", dep, f"@executable_path/../Frameworks/{name_of[resolve(dep, orig)]}"]
    for rp in rpaths(copy):
        if not rp.startswith("@executable_path"):
            args += ["-delete_rpath", rp]
    if in_fw:
        args += ["-id", f"@executable_path/../Frameworks/{copy.name}"]
    elif subprocess.run(["otool", "-D", str(copy)], capture_output=True,
                                                 text=True).stdout.count("\n") > 1:
        args += ["-id", copy.name]
    if args:
        subprocess.run(["install_name_tool", *args, str(copy)], check=True, stderr=subprocess.DEVNULL)


def minos(path):
    out = subprocess.run(["otool", "-l", str(path)], capture_output=True, text=True).stdout
    m = re.search(r"cmd LC_BUILD_VERSION\n(?:.*\n){2}\s+minos (\S+)", out)
    return tuple(int(x) for x in m.group(1).split(".")) if m else (0,)


def main():
    app = Path(sys.argv[1]).resolve()
    contents = app / "Contents"
    exe = contents / "MacOS" / "bilang"
    fw = contents / "Frameworks"
    res = contents / "Resources"
    share = res / "share"
    loaders_dst = res / "lib" / "gdk-pixbuf-2.0" / PIXBUF_VER / "loaders"
    fw.mkdir(parents=True, exist_ok=True)

    # gdk-pixbuf loaders (the svg one is librsvg's, linked into the loaders dir)
    loaders_src = BREW / "lib" / "gdk-pixbuf-2.0" / PIXBUF_VER / "loaders"
    loaders_dst.mkdir(parents=True, exist_ok=True)
    loaders = []
    for so in sorted(loaders_src.glob("*.so")):
        dst = loaders_dst / so.name
        shutil.copy2(so.resolve(), dst)
        os.chmod(dst, 0o755)
        loaders.append(dst)

    extras = [(loaders_src / l.name, l) for l in loaders]   # (original, copy)

    # walk the dependency graph from the binary and the extras (original files)
    name_of = {}            # real source path -> file name in Frameworks
    todo = [exe, *(src for src, _ in extras)]
    while todo:
        cur = todo.pop()
        for dep in otool_deps(cur):
            if is_system(dep) or dep.startswith("@executable_path/"):
                continue
            real = resolve(dep, cur)
            if real in name_of:
                continue
            name = Path(dep).name  # the install name's file name (libfoo.1.dylib)
            if name in name_of.values():
                raise SystemExit(f"two libraries named {name}")
            name_of[real] = name
            todo.append(real)

    for real, name in name_of.items():
        dst = fw / name
        shutil.copy2(real, dst)
        os.chmod(dst, 0o755)
        fix(dst, real, name_of, True)
    fix(exe, exe, name_of, False)
    for src, dst in extras:
        fix(dst, src.resolve(), name_of, False)

    # the app's own icon, for the window
    apps = share / "icons" / "hicolor" / "scalable" / "apps"
    apps.mkdir(parents=True, exist_ok=True)
    shutil.copy2(Path(__file__).resolve().parent.parent / "icon.svg", apps / "io.r3al.bilang.svg")

    # loaders.cache with a placeholder dir; main.rs fills in the real path
    out = subprocess.run([str(BREW / "opt/gdk-pixbuf/bin/gdk-pixbuf-query-loaders"),
                          *(str(loaders_src / l.name) for l in loaders)],
                         capture_output=True, text=True, check=True).stdout
    out = out.replace(str(loaders_src), "@LOADERS_DIR@")
    out = "\n".join(l for l in out.splitlines() if not l.startswith("# LoaderDir") and not l.startswith("# Created"))
    (loaders_dst.parent / "loaders.cache.in").write_text(out + "\n")

    # GSettings schemas GTK and libadwaita read
    schemas = share / "glib-2.0" / "schemas"
    schemas.mkdir(parents=True, exist_ok=True)
    for x in (BREW / "share/glib-2.0/schemas").glob("org.gtk.gtk4.*.gschema.xml"):
        shutil.copy2(x, schemas)
    subprocess.run([str(BREW / "opt/glib/bin/glib-compile-schemas"), str(schemas)], check=True)

    # icon themes: Adwaita (no cursors — macOS draws its own) and hicolor
    icons = share / "icons"
    adw = (BREW / "share/icons/Adwaita").resolve()
    shutil.copytree(adw, icons / "Adwaita", symlinks=False,
                    ignore=shutil.ignore_patterns("cursors"), dirs_exist_ok=True)
    shutil.copytree((BREW / "share/icons/hicolor").resolve(), icons / "hicolor", dirs_exist_ok=True)
    gtk_cache = BREW / "opt/gtk4/bin/gtk4-update-icon-cache"
    for theme in ("Adwaita", "hicolor"):
        subprocess.run([str(gtk_cache), "-q", "-f", "-t", str(icons / theme)], check=False)

    # the newest macOS any bundled library was built for
    print(".".join(map(str, max(minos(f) for f in [exe, *fw.glob("*.dylib")]))))


if __name__ == "__main__":
    main()
