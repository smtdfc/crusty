import subprocess
import argparse
from pathlib import Path


PLUGINS = [
    "crusty_plugin_example"
]


def run_manager():
    parser = argparse.ArgumentParser(description="Hardcore Plugin Manager")
    parser.add_argument("--out", type=str, required=True,
                        help="Output directory")
    parser.add_argument("--cross", action="store_true",
                        help="Enable cross-compilation")
    args = parser.parse_args()

    # Determine paths
    script_dir = Path(__file__).parent.absolute()
    workspace_root = script_dir.parent
    crates_dir = workspace_root / "crates"

    build_script_rel_path = "../../scripts/build.py"

    print(
        f"Hardcore Manager: Starting sequence for {len(PLUGINS)} plugins...\n")

    for plugin in PLUGINS:
        plugin_path = crates_dir / plugin

        if not plugin_path.exists():
            print(f"Skipping: {plugin} (Folder not found)")
            continue

        print(f"Entering: {plugin}")

        # Build the command exactly as you've been running it
        cmd = ["python", build_script_rel_path, "--out", args.out]
        if args.cross:
            cmd.append("--cross")

        try:
            # The 'cwd' argument does the 'cd' for you
            subprocess.run(cmd, cwd=plugin_path, check=True)
            print(f"Successfully processed: {plugin}\n")
        except subprocess.CalledProcessError:
            print(f"Error: Build script failed for {plugin}\n")


if __name__ == "__main__":
    run_manager()
