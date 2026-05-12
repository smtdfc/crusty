## Guide: Semantic Versioning & Commit Standards

### 1. The Golden Rule

All commit messages **MUST** follow the **Conventional Commits** specification. This is mandatory for the automated release system to calculate versions and generate changelogs correctly.

**Structure:**
`<type>(scope): <description>`

### 2. Commit Types

Select the appropriate `type` based on the nature of the change:

| Type           | Meaning                               | Release Type       |
| -------------- | ------------------------------------- | ------------------ |
| **`feat`**     | A new feature                         | **Minor** (v1.1.0) |
| **`fix`**      | A bug fix                             | **Patch** (v1.0.1) |
| **`perf`**     | Performance improvement               | **Patch** (v1.0.1) |
| **`refactor`** | Code change (no fix/feature)          | None               |
| **`docs`**     | Documentation only                    | None               |
| **`style`**    | Formatting, missing semi-colons, etc. | None               |
| **`chore`**    | Maintenance/Build tasks               | None               |

### 3. Breaking Changes

If a change modifies the `Common Crate` interface in a way that breaks compatibility (ABI/API changes), you **MUST** add an **`!`** after the type or include `BREAKING CHANGE:` in the footer.

- **Example:** `feat(common)!: rename PluginMod to IPlugin`
- **Result:** Triggers a **Major** release (v1.x.x -> v2.0.0).

### 4. Scoping

Use a `scope` to identify which part of the workspace is affected:

- `(plugin)`: Changes to the shared interface/ABI for plugin.
- `(crusty)`: Changes to the core and main executable.
- `(<plugin-name>)`: Changes to a specific plugin.

### 5. Copilot Pre-Commit Checklist:

- **Logic Check:** Does this change logic? If yes -> Use `feat` or `fix`.
- **Interface Check:** Is the `common` crate modified? If yes -> Evaluate if it's a `BREAKING CHANGE`.
- **Formatting:** Use imperative, present tense (e.g., "add" not "added"), start with lowercase, and no period at the end.
- Good `feat(crusty): add logging`
- Bad `Feat: Added some logging.`

---

### Technical Context for Copilot

- **Language:** Rust
- **Architecture:** Plugin-based using `abi_stable`.
- **Versioning:** Do **not** manually update `version` fields in `Cargo.toml`. The `semantic-release` bot handles this automatically based on commit messages.
