# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Section 06 of a Rust Udemy course, focused on the **package / crate / module** organization model. It is a learning sandbox, not production code. `architecture-rust.org` is the lesson notes (FR) on how packages/crates/modules articulate; `.tex`/`.pdf` are exported renders of that same Org file.

## Layout

Cargo **workspace** (`resolver = "2"`) with a single member, the `my_package` binary crate. The workspace `Cargo.toml` at the root has no `[package]`; all actual crate code lives under `my_package/`.

Note: the root `Cargo.toml` contains `licence = "MIT"` (British spelling) — Cargo ignores it as an unknown key; the valid key is `license`.

## Commands

Run from the repo root (Cargo resolves the workspace):

```bash
cargo run -p my_package      # build + run the binary
cargo build                  # build all members
cargo test                   # run all tests
cargo test <name>            # run a single test by name
cargo clippy                 # lint
cargo fmt                    # format
```

Edition is **2024** for `my_package`.
