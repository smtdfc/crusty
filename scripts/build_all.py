import os
import subprocess
import argparse
from pathlib import Path


def find_plugin_modules(root_dir: Path):
    """Scan all folders containing plugin.crusty.json"""
    return [p.parent for p in root_dir.rglob("plugin.crusty.json")
            if "target" not in p.parts and "dist" not in p.parts]


def main():
    parser = argparse.ArgumentParser(
        description="Crusty Workspace Orchestrator")
    parser.add_argument("-t", "--target", required=True, help="Target triple")
    parser.add_argument("--cross", action="store_true",
                        help="Use cross instead of cargo")
    args = parser.parse_args()

    # Assuming script is in scripts/ directory
    workspace_root = Path(__file__).parent.parent
    build_script = workspace_root / "scripts" / "build.py"

    modules = find_plugin_modules(workspace_root)

    if not modules:
        print("No plugin modules found. Please check your project structure.")
        return

    print(f"Found {len(modules)} plugin(s). starting build process...")

    for module in modules:
        print(f"\n--- Building: {module.name} ---")

        # Execute build.py for each module
        # Pass target and cross arguments to the build script
        cmd = [
            "python", str(build_script),
            "--target", args.target
        ]
        if args.cross:
            cmd.append("--cross")

        # Run the build script within the module's directory
        # This allows build.py to locate the local plugin.crusty.json
        result = subprocess.run(cmd, cwd=module)

        if result.returncode != 0:
            print(f"Error: Module {module.name} failed to build.")
        else:
            print(f"Success: Module {module.name} is ready in dist/")


if __name__ == "__main__":
    main()
