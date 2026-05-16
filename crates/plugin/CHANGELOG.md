# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/smtdfc/crusty/releases/tag/crusty_plugin-v0.1.0) - 2026-05-16

### Features

- add chrono dependency and enhance session management with async traits
- add dotenv support and enhance plugin communication with session ID
- Introduce CrustyError for better error handling across the application
- enhance plugin system with chat functionality and configuration handling
- implement plugin system with installation and metadata handling

### Other

- *(other)* Bump crusty_plugin v0.1.0, crusty_plugin_telegram v0.1.0
- *(other)* Release crusty_plugin v0.1.0, crusty_plugin_telegram v0.1.0
- *(other)* release
- *(other)* Refactor Crusty Agent: Restructure project into crates, implement AI proxy functionality, and enhance CLI commands

## [0.1.0](https://github.com/smtdfc/crusty/releases/tag/crusty_plugin-v0.1.0) - 2026-05-10

### Other

- Refactor Crusty Agent: Restructure project into crates, implement AI proxy functionality, and enhance CLI commands

## v0.1.0 (2026-05-16)

<csr-id-86a45774e74a849e020b50d2dd050547a677c9ca/>

### New Features

 - <csr-id-89b735188c3cf69fc9e54a2b2129c3d10088346a/> add chrono dependency and enhance session management with async traits
   - Added chrono as a dependency in Cargo.toml for date and time handling.
   - Refactored session management to use Arc and Mutex for thread-safe history storage.
   - Updated SQL queries to include session ID and created_at fields.
   - Enhanced plugin communication with new message callback functionality.
   - Improved error handling and logging throughout the codebase.
 - <csr-id-11003bfd3733d791d97957e723f51ce4650d8f0f/> add dotenv support and enhance plugin communication with session ID
 - <csr-id-c2f2c154b9c9c29deb3ca68dbb939484f29e7336/> Introduce CrustyError for better error handling across the application
   - Added a new `CrustyError` enum to encapsulate various error types, improving error management throughout the codebase.
   - Updated functions in `session.rs`, `store.rs`, `ai_proxy`, and `cli` modules to return `Result` types using `CrustyError`.
   - Enhanced logging with `tracing` for better debugging and error tracking.
   - Refactored existing error handling to utilize the new error type, ensuring consistent error reporting.
   - Implemented database initialization and message saving in the `MemoryStore`.
   - Improved proxy management in the `AIProxy` trait and its implementations.
   - Updated CLI commands to handle errors gracefully and provide user feedback.
 - <csr-id-fa6d6e80f728f7f75542f6d11eb21dbbb493cb93/> enhance plugin system with chat functionality and configuration handling
 - <csr-id-480215ab7a5183e458d7dc9859f8c4562dc96037/> implement plugin system with installation and metadata handling

### Chore

 - <csr-id-86a45774e74a849e020b50d2dd050547a677c9ca/> release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crusty_plugin v0.1.0, crusty_plugin_telegram v0.1.0 ([`dfdcc19`](https://github.com/smtdfc/crusty/commit/dfdcc1926449e36945526195c1e7549b232195a1))
    - Add chrono dependency and enhance session management with async traits ([`89b7351`](https://github.com/smtdfc/crusty/commit/89b735188c3cf69fc9e54a2b2129c3d10088346a))
    - Add dotenv support and enhance plugin communication with session ID ([`11003bf`](https://github.com/smtdfc/crusty/commit/11003bfd3733d791d97957e723f51ce4650d8f0f))
    - Introduce CrustyError for better error handling across the application ([`c2f2c15`](https://github.com/smtdfc/crusty/commit/c2f2c154b9c9c29deb3ca68dbb939484f29e7336))
    - Enhance plugin system with chat functionality and configuration handling ([`fa6d6e8`](https://github.com/smtdfc/crusty/commit/fa6d6e80f728f7f75542f6d11eb21dbbb493cb93))
    - Implement plugin system with installation and metadata handling ([`480215a`](https://github.com/smtdfc/crusty/commit/480215ab7a5183e458d7dc9859f8c4562dc96037))
    - Merge pull request #1 from smtdfc/release-plz-2026-05-10T06-48-14Z ([`3310886`](https://github.com/smtdfc/crusty/commit/331088685ac46b613dd0c016d6a267235a3bcc28))
    - Release ([`86a4577`](https://github.com/smtdfc/crusty/commit/86a45774e74a849e020b50d2dd050547a677c9ca))
</details>

