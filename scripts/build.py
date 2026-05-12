import os
import shutil
import json
import argparse
import subprocess
from pathlib import Path

TARGET_MAP = [
    ("x86_64-pc-windows-msvc", "windows", "x86_64"),
    ("aarch64-pc-windows-msvc", "windows", "aarch64"),
    ("x86_64-unknown-linux-gnu", "linux", "x86_64"),
    ("aarch64-unknown-linux-gnu", "linux", "aarch64"),
]

PLUGIN_NAME = Path.cwd().name
PLUGIN_ID = f"@crusty/{PLUGIN_NAME}"


def get_real_filename(plugin_name, target_triple):
    """
    Rust tấu hài: Windows ra .dll, Linux ra lib*.so
    """
    if "windows" in target_triple:
        return f"{plugin_name}.dll"
    else:
        return f"lib{plugin_name}.so"


def build_targets(use_cross=False, target=None):
    compiler = "cross" if use_cross else "cargo"
    print(f"--- [{PLUGIN_NAME}] BUILDING WITH {compiler.upper()} ---")

    targets_to_build = [
        t for t in TARGET_MAP if target is None or t[0] == target]

    for target_triple, _, _ in targets_to_build:
        if use_cross and "windows-msvc" in target_triple:
            print(
                f"Skipping {target_triple}: Cross-rs doesn't support MSVC images easily. 🤡")
            continue

        print(f"Compiling {target_triple}...")

        if not use_cross:
            subprocess.run(["rustup", "target", "add",
                           target_triple], check=False)

        cmd = [compiler, "build", "--release", "--target", target_triple]
        try:
            subprocess.run(cmd, check=True)
        except subprocess.CalledProcessError:
            print(f"❌ Error building {target_triple}. Skipping...")


def ship_it(output_dir, target=None):
    workspace_root = Path.cwd().parent.parent

    plugin_dest_folder = Path(output_dir).joinpath(*PLUGIN_ID.split('/'))
    platforms_metadata = []

    print(f"--- PACKAGING {PLUGIN_ID} ---")
    print(f"Dest: {plugin_dest_folder}")

    targets_to_ship = [
        t for t in TARGET_MAP if target is None or t[0] == target]

    for target_triple, os_name, arch_name in targets_to_ship:
        real_file_name = get_real_filename(PLUGIN_NAME, target_triple)

        source_file = workspace_root / "target" / \
            target_triple / "release" / real_file_name

        if not source_file.exists():
            source_file = workspace_root / "target" / "release" / real_file_name

        if source_file.exists():
            print(f"✅ Found binary: {source_file.name} for {target_triple}")

            arch_dir = f"{os_name}-{arch_name}"
            rel_bin_path = Path("bin") / arch_dir
            full_bin_path = plugin_dest_folder / rel_bin_path

            full_bin_path.mkdir(parents=True, exist_ok=True)

            dest_file_name = f"{PLUGIN_NAME}{Path(real_file_name).suffix}"
            shutil.copy(source_file, full_bin_path / dest_file_name)

            platforms_metadata.append({
                "os": os_name,
                "arch": arch_name,
                "file": (rel_bin_path / dest_file_name).as_posix()
            })
        else:
            print(
                f"❓ Binary not found for {target_triple} (Expected: {real_file_name})")

    if not platforms_metadata:
        print(
            f"‼️ No binaries found for {PLUGIN_NAME}! Check your build logs.")
        return

    metadata = {
        "id": PLUGIN_ID,
        "name": PLUGIN_NAME.replace('_', ' ').title(),
        "version": "1.0.0",
        "platforms": platforms_metadata,
        "features": ["chat"]
    }

    with open(plugin_dest_folder / "metadata.json", "w", encoding="utf-8") as f:
        json.dump(metadata, f, indent=4, ensure_ascii=False)

    print(f"✨ Done packaging {PLUGIN_NAME}.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=str, required=True,
                        help="Output directory for packaged plugins")
    parser.add_argument("--cross", action="store_true",
                        help="Use 'cross' instead of 'cargo'")
    parser.add_argument("--no-build", action="store_true",
                        help="Skip the build step, only package")
    parser.add_argument("--target", type=str, default=None,
                        help="Specific target triple to process")

    args = parser.parse_args()

    if not args.no_build:
        build_targets(use_cross=args.cross, target=args.target)

    ship_it(args.out, target=args.target)
