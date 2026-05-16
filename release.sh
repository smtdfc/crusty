#!/bin/bash

# Check for required arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: ./release.sh <package_name> <new_version>"
    echo "Example: ./release.sh crusty_plugin_telegram 0.1.2"
    exit 1
fi

PKG_NAME=$1
NEW_VERSION=$2
MANIFEST_PATH="crates/$PKG_NAME/Cargo.toml"

# 1. Validate manifest existence
if [ ! -f "$MANIFEST_PATH" ]; then
    echo "❌ Error: $MANIFEST_PATH not found"
    exit 1
fi

echo "🚀 Starting release for $PKG_NAME version $NEW_VERSION..."

# 2. Update version in Cargo.toml (using sed)
# Replaces the first occurrence of version = "..."
sed -i "0,/^version = .*/s//version = \"$NEW_VERSION\"/" "$MANIFEST_PATH"

if [ $? -eq 0 ]; then
    echo "✅ Updated version in Cargo.toml"
else
    echo "❌ Error: Failed to update Cargo.toml"
    exit 1
fi

# 3. Git Commit
git add "$MANIFEST_PATH"

# Add CHANGELOG if it exists in the package folder
if [ -f "crates/$PKG_NAME/CHANGELOG.md" ]; then
    git add "crates/$PKG_NAME/CHANGELOG.md"
fi

git commit -m "release: $PKG_NAME v$NEW_VERSION"

# 4. Create Git Tag (matches your GitHub Action regex)
TAG_NAME="${PKG_NAME}-v${NEW_VERSION}"
git tag -a "$TAG_NAME" -m "Release $TAG_NAME"

echo "✅ Created tag: $TAG_NAME"

# 5. Push changes and tags
echo "📤 Pushing to GitHub..."
git push origin main && git push origin "$TAG_NAME"

if [ $? -eq 0 ]; then
    echo "🎉 Success! Check GitHub Actions to monitor the R2 deployment."
else
    echo "❌ Error: Push failed. Check your connection or permissions."
    exit 1
fi