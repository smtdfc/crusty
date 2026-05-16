# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/smtdfc/crusty/releases/tag/crusty_plugin_telegram-v0.1.1) - 2026-05-16

### Bug Fixes

- *(crusty_plugin_telegram)* update author name in PluginInfo for Crusty Plugin Telegram
- update author name in PluginInfo for Crusty Plugin Telegram
- update author name in plugin info to reflect team attribution
- update plugin version to use the package version from Cargo.toml

### Features

- add chrono dependency and enhance session management with async traits
- add dotenv support and enhance plugin communication with session ID
- enhance AIProxy with host and is_local fields; add dashboard URL functionality
- Introduce CrustyError for better error handling across the application
- restructure project to include new Telegram plugin and session management

### Other

- *(crusty_plugin_telegram)* bump version to 0.1.1
- *(other)* Bump crusty_plugin_telegram v0.1.0
- *(other)* Release crusty_plugin_telegram v0.1.0
- *(other)* Release crusty_plugin_telegram v0.1.0
- *(other)* Bump crusty_plugin v0.1.0, crusty_plugin_telegram v0.1.0
- *(other)* Release crusty_plugin v0.1.0, crusty_plugin_telegram v0.1.0
- *(crusty_plugin_telegram)* enhance plugin description for clarity and consistency in metadata
- *(other)* standardize log message and improve message sending format in bot module
- *(other)* update build workflow for R2 deployment and enhance dependency management
- *(other)* update build process and remove deprecated plugin files

## [0.1.0](https://github.com/smtdfc/crusty/releases/tag/crusty_plugin_example-v0.1.0) - 2026-05-10

### Added

- implement plugin system with installation and metadata handling

## v0.1.0 (2026-05-16)

<csr-id-34f6e7b34eb4fcc3fa03ed385b51d2cd51a3ad1e/>
<csr-id-d84e5034a6ea14cc1f27ae8f197a284d6a641253/>
<csr-id-0926ef92358c7078cf064115ba7c4d432d747506/>
<csr-id-5f49518a9590fbae32695a94a89c15b1a4b0b44a/>

### Refactor

 - <csr-id-34f6e7b34eb4fcc3fa03ed385b51d2cd51a3ad1e/> enhance plugin description for clarity and consistency in metadata
 - <csr-id-d84e5034a6ea14cc1f27ae8f197a284d6a641253/> standardize log message and improve message sending format in bot module
 - <csr-id-0926ef92358c7078cf064115ba7c4d432d747506/> update build workflow for R2 deployment and enhance dependency management
 - <csr-id-5f49518a9590fbae32695a94a89c15b1a4b0b44a/> update build process and remove deprecated plugin files

### Bug Fixes

 - <csr-id-4bf81b347057d1d845a263c032ae00d7388325b9/> update author name in plugin info to reflect team attribution
 - <csr-id-00d8abdf45046341b134c2c173d985500a26523d/> update plugin version to use the package version from Cargo.toml
 - <csr-id-b6401e3425420e5858a4b1499a989d51809c2162/> update author name in PluginInfo for Crusty Plugin Telegram
 - <csr-id-ee235fa1f03260107902cfc244cb9f5a155ce28d/> update author name in PluginInfo for Crusty Plugin Telegram

### New Features

 - <csr-id-c2f2c154b9c9c29deb3ca68dbb939484f29e7336/> Introduce CrustyError for better error handling across the application
   - Added a new `CrustyError` enum to encapsulate various error types, improving error management throughout the codebase.
   - Updated functions in `session.rs`, `store.rs`, `ai_proxy`, and `cli` modules to return `Result` types using `CrustyError`.
   - Enhanced logging with `tracing` for better debugging and error tracking.
   - Refactored existing error handling to utilize the new error type, ensuring consistent error reporting.
   - Implemented database initialization and message saving in the `MemoryStore`.
   - Improved proxy management in the `AIProxy` trait and its implementations.
   - Updated CLI commands to handle errors gracefully and provide user feedback.
 - <csr-id-3ddb14b793aa8a044165c4c772f911c78dee0116/> restructure project to include new Telegram plugin and session management
   - Added `crusty_plugin_telegram` to the workspace with basic bot functionality.
   - Implemented session management with SQLite support for storing chat history.
   - Refactored `ChatAgent` to utilize session history instead of internal history.
   - Introduced `MemoryStore` for handling database connections and message storage.
   - Updated CLI commands to support session creation and management.
   - Enhanced `print_banner` to display session information.
   - Added database interaction methods for saving and retrieving messages.
   - Updated configuration to include store settings for SQLite.
   - Cleaned up unused code and comments across various modules.
 - <csr-id-89b735188c3cf69fc9e54a2b2129c3d10088346a/> add chrono dependency and enhance session management with async traits
   - Added chrono as a dependency in Cargo.toml for date and time handling.
   - Refactored session management to use Arc and Mutex for thread-safe history storage.
   - Updated SQL queries to include session ID and created_at fields.
   - Enhanced plugin communication with new message callback functionality.
   - Improved error handling and logging throughout the codebase.
 - <csr-id-11003bfd3733d791d97957e723f51ce4650d8f0f/> add dotenv support and enhance plugin communication with session ID
 - <csr-id-e3b08bb9f9a4bb88500de7c02baf29ce90797e6c/> enhance AIProxy with host and is_local fields; add dashboard URL functionality

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crusty_plugin_telegram v0.1.0 ([`c0f7efe`](https://github.com/smtdfc/crusty/commit/c0f7efe12a31a233272fb7c2fc126443ccd6a56b))
    - Update author name in PluginInfo for Crusty Plugin Telegram ([`ee235fa`](https://github.com/smtdfc/crusty/commit/ee235fa1f03260107902cfc244cb9f5a155ce28d))
    - Release crusty_plugin_telegram v0.1.0 ([`f6ec9a4`](https://github.com/smtdfc/crusty/commit/f6ec9a4fc62faa182bd586915e34d2d8fa82138f))
    - Update author name in PluginInfo for Crusty Plugin Telegram ([`b6401e3`](https://github.com/smtdfc/crusty/commit/b6401e3425420e5858a4b1499a989d51809c2162))
</details>

