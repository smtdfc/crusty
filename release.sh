#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: ./release.sh <package_name> <new_version>"
    exit 1
fi

PKG_NAME=$1
NEW_VERSION=$2
MANIFEST_PATH="crates/$PKG_NAME/Cargo.toml"
CHANGELOG_PATH="crates/$PKG_NAME/CHANGELOG.md"

if [ ! -f "$MANIFEST_PATH" ]; then
    echo "❌ Error: $MANIFEST_PATH not found"
    exit 1
fi

echo "🚀 Starting release for $PKG_NAME v$NEW_VERSION..."

# --- 1. GENERATE CHANGELOG ---
echo "📝 Updating Changelog..."
DATE=$(date +%Y-%m-%d)
LAST_TAG=$(git describe --tags --abbrev=0 --match "$PKG_NAME-*" 2>/dev/null)

if [ -z "$LAST_TAG" ]; then
    LOGS=$(git log --pretty=format:"* %s (%h)" -- "crates/$PKG_NAME/")
else
    LOGS=$(git log "$LAST_TAG..HEAD" --pretty=format:"* %s (%h)" -- "crates/$PKG_NAME/")
fi

if [ -z "$LOGS" ]; then
    LOGS="* No specific changes recorded."
fi

NEW_CHANGES="## [$NEW_VERSION] - $DATE\n$LOGS\n"

if [ ! -f "$CHANGELOG_PATH" ]; then
    echo -e "# Changelog: $PKG_NAME\n\n$NEW_CHANGES" > "$CHANGELOG_PATH"
else

    echo -e "$NEW_CHANGES\n$(cat $CHANGELOG_PATH)" > "$CHANGELOG_PATH"
fi
echo "✅ Changelog updated at $CHANGELOG_PATH"

# --- 2. BUMP VERSION ---
sed -i "0,/^version = .*/s//version = \"$NEW_VERSION\"/" "$MANIFEST_PATH"
echo "✅ Cargo.toml updated"

# --- 3. GIT STUFF ---
git add "$MANIFEST_PATH" "$CHANGELOG_PATH"
git commit -m "release: $PKG_NAME v$NEW_VERSION"

TAG_NAME="${PKG_NAME}-v${NEW_VERSION}"
git tag -a "$TAG_NAME" -m "Release $TAG_NAME"
echo "✅ Created tag: $TAG_NAME"

# --- 4. PUSH ---
echo "📤 Pushing to GitHub..."
git push origin main && git push origin "$TAG_NAME"

if [ $? -eq 0 ]; then
    echo "🎉 Success! Check GitHub Actions."
else
    echo "❌ Error: Push failed."
    exit 1
fi