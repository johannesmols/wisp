# Wisp task runner — install: winget install Casey.Just

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
    @just --list

# ── Type checking ───────────────────────────────────────────────────────────

# Type-check entire workspace (fast; no codegen)
check:
    cargo check --workspace

# Verify wisp-core compiles to wasm32 (ensures it stays browser-compatible)
check-wasm:
    cargo check -p wisp-core --target wasm32-unknown-unknown

# ── Formatting ──────────────────────────────────────────────────────────────

# Format everything: Prettier (via prek) + rustfmt
fmt:
    prek run --all-files prettier
    cargo fmt

# Check formatting without modifying files (used in CI)
fmt-check:
    prek run --all-files prettier --hook-stage manual
    cargo fmt --check

# ── Linting ─────────────────────────────────────────────────────────────────

# Clippy across all targets; treats warnings as errors
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# ── Testing ──────────────────────────────────────────────────────────────────

# Run all tests with nextest (install: cargo install cargo-nextest)
test:
    cargo nextest run --workspace

# Run tests for a single crate, e.g.: just test-crate wisp-core
test-crate crate:
    cargo nextest run -p {{ crate }}

# ── Dev servers ─────────────────────────────────────────────────────────────

# Serve web build with hot-reload
serve-web:
    dx serve --package web

# Serve desktop build
serve-desktop:
    dx serve --package desktop

# ── CI gate (run locally before pushing) ────────────────────────────────────

ci:
    just check && just check-wasm && just lint && just test
