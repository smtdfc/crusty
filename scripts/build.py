import json
import argparse
import subprocess
import shutil
from pathlib import Path

# Updated comprehensive map
TARGET_MAP = {
    "x86_64-pc-windows-msvc": {"os": "windows", "arch": "x86_64", "ext": ".dll", "lib_prefix": ""},
    "aarch64-pc-windows-msvc": {"os": "windows", "arch": "aarch64", "ext": ".dll", "lib_prefix": ""},
    "x86_64-unknown-linux-gnu": {"os": "linux", "arch": "x86_64", "ext": ".so", "lib_prefix": "lib"},
    "aarch64-unknown-linux-gnu": {"os": "linux", "arch": "aarch64", "ext": ".so", "lib_prefix": "lib"},
    "aarch64-apple-darwin": {"os": "macos", "arch": "aarch64", "ext": ".dylib", "lib_prefix": "lib"},
    "x86_64-apple-darwin": {"os": "macos", "arch": "x86_64", "ext": ".dylib", "lib_prefix": "lib"},
}


def get_workspace_target_dir() -> Path:
    try:
        cmd = ["cargo", "metadata", "--format-version", "1", "--no-deps"]
        # Use shell=False for better argument handling in CI
        result = subprocess.check_output(cmd, text=True)
        return Path(json.loads(result)["target_directory"])
    except:
        return Path.cwd().parents[1] / "target"


def create_dist(data, target_triple):
    info = TARGET_MAP.get(target_triple)
    if not info:
        print(f"Target {target_triple} not supported in TARGET_MAP")
        return

    dist_dir = Path.cwd() / "dist"
    dist_dir.mkdir(exist_ok=True)

    package_name = data.get("package")
    file_name = f"{info['lib_prefix']}{package_name}{info['ext']}"

    # 1. Prepare metadata to match Rust Structs
    metadata = {
        "name": data.get("name"),
        "id": data.get("id"),
        "version": data.get("version"),
        "platforms": [{
            "os": info["os"],
            "arch": info["arch"],
            "file": file_name
        }],
        "features": data.get("features", [])
    }

    with open(dist_dir / "metadata.json", "w", encoding="utf-8") as f:
        json.dump(metadata, f, indent=4)

    # 2. Locate and copy binary
    workspace_target = get_workspace_target_dir()
    source_path = workspace_target / target_triple / "release" / file_name
    dest_path = dist_dir / file_name

    # FIX: Initialize success variable properly
    success = False

    # Check both exact name and lib_prefix variant
    possible_sources = [source_path, source_path.parent / f"lib{file_name}"]

    for src in possible_sources:
        if src.exists():
            shutil.copy2(src, dest_path)
            print(f"Successfully copied: {src.name}")
            success = True
            break

    if not success:
        print(f"Error: Could not find build output at {source_path}")
        exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--target",
                        default="x86_64-pc-windows-msvc")
    parser.add_argument("--cross", action="store_true")
    args = parser.parse_args()

    # Load local plugin config
    config_path = Path("plugin.crusty.json")
    if not config_path.exists():
        print("plugin.crusty.json not found. Skip")
        exit(0)

    with open(config_path, "r") as f:
        data = json.load(f)

    # Execute Build
    tool = "cross" if args.cross else "cargo"
    # Ensure command is a list for robust subprocess execution
    build_cmd = [tool, "build", "--target", args.target, "--release"]

    print(f"Running: {' '.join(build_cmd)}")
    result = subprocess.run(build_cmd)

    if result.returncode == 0:
        create_dist(data, args.target)
    else:
        exit(result.returncode)
