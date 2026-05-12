import json
import os
import argparse
from pathlib import Path
import subprocess
import shutil
from typing import Any


TARGET_MAP = {
    "x86_64-pc-windows-msvc": {
        "os": "windows",
        "arch": "x86_64",
        "ext": ".dll",
        "lib_prefix": ""
    },
    "x86_64-unknown-linux-gnu": {
        "os": "linux",
        "arch": "x86_64",
        "ext": ".so",
        "lib_prefix": "lib"
    },
    "aarch64-apple-darwin": {
        "os": "macos",
        "arch": "aarch64",
        "ext": ".dylib",
        "lib_prefix": "lib"
    }
}


cwd = Path.cwd()
plugin_decl_file = cwd / "plugin.crusty.json"
plugin_dist = cwd / "dist"
plugin_metdata_file = plugin_dist / "metadata.json"

data: dict[str, str] | None = None
with open(plugin_decl_file, "r") as f:
    data = json.load(f)


def get_workspace_target_dir() -> Path:
    try:
        cmd = ["cargo", "metadata", "--format-version", "1", "--no-deps"]
        result = subprocess.check_output(cmd, shell=True, text=True)
        data = json.loads(result)
        return Path(data["target_directory"])
    except Exception as e:
        return Path.cwd().parent.parent / "target"


workspace_target = get_workspace_target_dir()


def create_dist(data: Any, target_triple: str) -> None:
    Path.mkdir(plugin_dist, exist_ok=True)
    info = TARGET_MAP.get(target_triple, {
        "ext": ".so",
        "lib_prefix": "lib"
    })

    bin_name = data.get("package")
    file_name = f"{info['lib_prefix']}{bin_name}{info['ext']}"
    workspace_target = get_workspace_target_dir()
    source_path = workspace_target / target_triple / "release" / file_name
    dest_path = plugin_dist / file_name

    if dest_path.exists():
        shutil.copy2(source_path, dest_path)
        print(f"✅ Copied file at: {source_path} -> {dest_path}")
        success = True

    if not success:
        print(f"File not found at: {source_path}")

    data["platforms"] = [
        {
            "os": info.get("os"),
            "arch": info.get("arch"),
            "file": dest_path.name
        }
    ]

    with open(plugin_metdata_file, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4)


parser = argparse.ArgumentParser(
    description="Crusty Plugin build script")
parser.add_argument("-t", "--target", type=str,
                    default="x86_64-pc-windows-msvc", help="Build target")
args = parser.parse_args()


build_tool = "cross" if getattr(args, 'cross', False) else "cargo"

result = subprocess.run([
    build_tool, "build",
    "--target", args.target,
    "--release"
], shell=True)


if result.returncode == 0:
    create_dist(data, args.target)
    print("🚀 Build và package successful !")
else:
    print("💥 Build failed")
