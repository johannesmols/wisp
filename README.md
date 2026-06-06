# Wisp

Personal driving heatmap explorer. Multi-source (GPX, Strava, Connected Cars, Google Timeline),
offline-capable, runs as a web app or native desktop/mobile app via [Dioxus](https://dioxuslabs.com/).

## Prerequisites

Install these once per machine. After each `winget` install, **open a new terminal** so the
updated PATH takes effect.

```powershell
winget install Rustlang.Rustup      # Rust toolchain manager
rustup update stable                # ensure stable is current

winget install Casey.Just           # just task runner
winget install j178.Prek            # prek pre-commit hook manager

cargo install --locked cargo-nextest         # faster test runner (used by `just test`)
cargo install dioxus-cli            # dx CLI (used by `just serve-*`)
```

> `cargo install` compiles from source and takes a few minutes. If you have
> [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) installed, replace
> `cargo install` with `cargo binstall` to download pre-built binaries instead.

The `wasm32-unknown-unknown` target is added automatically from `rust-toolchain.toml` on first
build — no manual `rustup target add` needed.

## First-time setup

```powershell
git clone <repo>
cd wisp
prek install   # registers pre-commit hooks from .pre-commit-config.yaml — run once per clone
```

## Running things

All common tasks are in the `justfile`. Run `just` with no arguments to list them:

```
just check              # cargo check --workspace (fast type-check, no codegen)
just check-wasm         # verify wisp-core compiles to wasm32 (keeps it browser-safe)
just fmt                # format everything: rustfmt + Prettier via prek
just fmt-check          # same checks without modifying files (used in CI)
just lint               # clippy --all-targets -D warnings
just test               # run all tests with cargo-nextest
just test-crate <name>  # run tests for one crate, e.g. just test-crate wisp-core
just serve-web          # dx serve --package web (hot-reload)
just serve-desktop      # dx serve --package desktop
just ci                 # fmt-check + check + check-wasm + lint + test (run before pushing)
```

## Project structure

```
wisp/
├─ packages/           # Dioxus deployment targets + shared layers
│  ├─ web/             # Web build (wasm32)
│  ├─ desktop/         # Native desktop build
│  ├─ mobile/          # Mobile build
│  ├─ ui/              # Shared Dioxus components
│  └─ api/             # Shared Dioxus server functions
└─ crates/             # Domain + ports (hexagonal architecture)
   ├─ core/            # Pure domain types: Trip, TripId, SourceKind, Bbox
   ├─ contract/        # TripService trait + DTOs (the main "interface assembly")
   ├─ store/           # TripStore port (persistence)
   ├─ sources/         # TripSource port (external data fetch)
   └─ ingest/          # Orchestrator: fetch → normalise → persist
```

## Pre-commit hooks

Hooks run automatically on `git commit` after `prek install`. They cover:

- **Prettier** — formats `.rs`, `.md`, `.json`, `.toml`, `.css`
- **cargo fmt** — checks Rust formatting workspace-wide

To run them manually against all files:

```powershell
prek run --all-files
```
