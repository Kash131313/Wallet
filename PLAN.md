# Project Plan for **AckiNest** (multi‑wallet)

## Overview
We will build a cross‑platform wallet that keeps all the features of the official Acki Nacki Wallet and adds:
- A master password protecting an encrypted store of many accounts (24‑word seed phrases).
- Instant switching between accounts without logging out.
- Optional **multisig** support (multiple signatures for a single address).
- Easy linking of external DApps and miners, exactly as the official wallet does.

## Stack (chosen for simplicity and wide device support)
| Layer | Technology | Reason |
|-------|------------|--------|
| Core language | **Rust** | Same language used by the official ; safe, fast, easy to compile for all targets. |
| Desktop UI | **Tauri** + **React** | Official wallet already uses Tauri; React gives fast development and small bundles. |
| Mobile UI | **React Native** with a Rust bridge (/) | One code‑base for Android and iOS, re‑uses the Rust core. |
| Encryption |  (password‑derived key) +  (authenticated encryption) | Proven Rust crates, simple API, strong security. |
| Persistent storage | Encrypted JSON file (). Can be switched to **SQLCipher** later if needed. |
| Network |  crate | Communicates with the official GraphQL endpoint . |
| Blockchain SDK | **tvmlabs/tvm‑sdk** (Rust) | Official SDK for address derivation, signing and transaction building. |
| Build tools | Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]
       cargo [+toolchain] [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...

Options:
  -V, --version                  Print version info and exit
      --list                     List installed commands
      --explain <CODE>           Provide a detailed explanation of a rustc error message
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring [possible values: auto, always, never]
  -C <DIRECTORY>                 Change to DIRECTORY before doing anything (nightly-only)
      --locked                   Assert that `Cargo.lock` will remain unchanged
      --offline                  Run without accessing the network
      --frozen                   Equivalent to specifying both --locked and --offline
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
                                 details
  -h, --help                     Print help

Commands:
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary
    uninstall   Uninstall a Rust binary
    ...         See all commands with --list

See 'cargo help <command>' for more information on a specific command., ,  (Android),  + Xcode (iOS) | Standard Rust toolchain, works on all platforms. |

## Architecture (high‑level)
- **rust‑core** ( crate)
  - : load/save encrypted store, add/import accounts, select active account.
  - : KDF, AEAD, key derivation from seed phrase.
  - : thin GraphQL wrapper for balance queries and sending transactions.
  -  (optional): wrapper around the  contract from .
- **frontend** (Tauri + React)
  - **Accounts screen** – list name, short address, balance, button *Set active*.
  - **Add account** – name + 24‑word seed input, validation, address generation.
  - **Master password screen** – first‑run setup, change password, auto‑lock timer.
  - **DApp / Miner integration** – list linked apps, deep‑link handling (same as official wallet).
  - **Settings** – export seed after re‑auth, backup/restore encrypted store.
- **mobile** (React Native) re‑uses the same Rust core through a native module; UI mirrors desktop experience.

## Phases
### Phase 0 – Research (completed)
- Extracted  from the official APK.
- Collected official sources (see ).

### Phase 1 – Requirements & Design (in progress)
- Confirm functional requirements (all official features + master‑password store + multisig).
- Finalise UI framework choices (React for web/desktop, React Native for mobile).

### Phase 2 – MVP (core + basic UI)
1. Scaffold repository ().
2. Implement encryption module with unit tests.
3. Add account import (seed → address) using .
4. Store accounts in encrypted JSON file.
5. UI: list accounts, add account, set active account.
6. Show balance by GraphQL query.
7. Master‑password flow (setup, unlock, auto‑lock).

### Phase 3 – Multisig & Advanced Features
- Integrate  contract from .
- UI for creating a multisig wallet, managing cosigners.
- Deep‑link handling for external DApps/miners.
- Backup / restore encrypted store.

### Phase 4 – Cross‑platform Packaging
- Desktop builds for Windows, macOS, Linux ().
- Android build ().
- iOS build ( + Xcode archive).
- Produce unsigned binaries for manual distribution.

### Phase 5 – Testing & Documentation
- Unit tests for crypto, wallet manager, network layer.
- Integration tests with a testnet node ().
- End‑to‑end UI tests (Playwright for desktop, Detox for mobile).
- Write user guide, developer README, API docs.

## Security notes
- Master password never stored; only a Argon2‑derived key lives in memory.
- Random 16‑byte salt stored next to the encrypted file.
- Auto‑lock after configurable inactivity.
- All traffic over HTTPS (official GraphQL endpoint already uses TLS).

## Next actions (need your confirmation)
1. Approve the **stack** (Rust + Tauri + React + React Native). If you prefer a different UI library, tell me.
2. Confirm that **multisig support** should be in Phase 3 (you said it’s needed).
3. Approve the **file names**:  (this file) and  (status log).
4. Any extra “super‑functional” ideas (biometric unlock, QR‑code import/export, etc.)?

Once you confirm, I will create  with the current status and start the scaffolding.

## Recent modifications (offline support)

- Made `tvm-sdk` and `reqwest` optional and added a `real` feature flag.
- Implemented conditional compilation in `src/network.rs`: real implementations behind `#[cfg(feature = "real")]`, stub implementations for offline builds.
- Updated `Cargo.toml` accordingly.
- This allows the project to compile without network access, using base64‑encoded seed as address and a zero‑balance stub.
