import os
import shutil
import json
import argparse
import subprocess
from pathlib import Path

# Target configuration list
TARGET_MAP = [
    ("x86_64-pc-windows-msvc", "windows", "x86_64"),
    ("aarch64-pc-windows-msvc", "windows", "aarch64"),
    ("x86_64-unknown-linux-gnu", "linux", "x86_64"),
    ("aarch64-unknown-linux-gnu", "linux", "aarch64"),
]


CURRENT_DIR_NAME = Path.cwd().name
PLUGIN_NAME = CURRENT_DIR_NAME
PLUGIN_ID = f"@crusty/{PLUGIN_NAME}"


def get_extension(target):
    if "windows" in target:
        return ".dll"
    return ".so"


def build_targets(use_cross=False):
    compiler = "cross" if use_cross else "cargo"
    print(f"[{PLUGIN_NAME}] Starting build using {compiler.upper()}...")

    for target, _, _ in TARGET_MAP:
        print(f"Compiling {target}...")
        if not use_cross:
            subprocess.run(["rustup", "target", "add", target], check=False)

        cmd = [compiler, "build", "--release", "--target", target]
        try:
            subprocess.run(cmd, check=True)
        except subprocess.CalledProcessError:
            print(f"Error building {target}. Skipping...")


def ship_it(output_dir):
    workspace_root = Path.cwd().parent.parent

    plugin_dest_folder = Path(output_dir).joinpath(*PLUGIN_ID.split('/'))
    platforms_metadata = []

    print(f"Packaging {PLUGIN_ID} to: {plugin_dest_folder}")

    for target, os_name, arch_name in TARGET_MAP:
        ext = get_extension(target)
        source_file = workspace_root / "target" / \
            target / "release" / f"{PLUGIN_NAME}{ext}"

        # Fallback if binary is in root target/release (for native host builds)
        if not source_file.exists():
            source_file = workspace_root / "target" / \
                "release" / f"{PLUGIN_NAME}{ext}"

        if source_file.exists():
            arch_dir = f"{os_name}-{arch_name}"
            rel_bin_path = Path("bin") / arch_dir
            full_bin_path = plugin_dest_folder / rel_bin_path

            full_bin_path.mkdir(parents=True, exist_ok=True)
            shutil.copy(source_file, full_bin_path / f"{PLUGIN_NAME}{ext}")

            platforms_metadata.append({
                "os": os_name,
                "arch": arch_name,
                "file": (rel_bin_path / f"{PLUGIN_NAME}{ext}").as_posix()
            })

    if not platforms_metadata:
        print(f"No binaries found for {PLUGIN_NAME}!")
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

    print(f"Done packaging {PLUGIN_NAME}.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=str, required=True)
    parser.add_argument("--cross", action="store_true")
    parser.add_argument("--no-build", action="store_true")
    args = parser.parse_args()

    if not args.no_build:
        build_targets(use_cross=args.cross)
    ship_it(args.out)
