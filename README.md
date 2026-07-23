<div align="center">

<h1>
  Froq Build (<code>froq</code>)
</h1>

**Froq Build** is a rebranded, open-source fork of SpaceXAI's Grok Build terminal-based AI coding agent. It runs as a full-screen TUI that understands your codebase, edits files, executes shell commands, searches the web, and manages long-running tasks.

[Installing the released binary](#installing-the-released-binary) ·
[Building from source](#building-from-source) ·
[Documentation](#documentation) ·
[Repository layout](#repository-layout) ·
[Development](#development) ·
[Contributing](#contributing) ·
[License](#license)

**Learn more about the fork at [github.com/marius-patrik/froq](https://github.com/marius-patrik/froq)**

This repository contains the Rust source for the `froq` CLI/TUI and its agent runtime. It is maintained and synchronized with upstream monorepo releases while preserving custom paywall bypasses and build adjustments.

</div>

---

## Key Fork Features

- **Billing Bypass:** Completely disables remote and local subscription gates, removing the "Subscribe Now" paywall/banners.
- **Windows MSVC Optimization:** Custom linker flags configured under `.cargo/config.toml` (e.g., `-C debuginfo=0` and `/DEBUG:NONE`) to significantly reduce memory usage during compilation and prevent OOM link failures.
- **Cross-Platform Compilation:** Fixes conditional imports and Unix-specific device dependencies to support native Windows builds.

---

## Installing the released binary

Prebuilt binaries are published on the [Releases](https://github.com/marius-patrik/froq/releases) page for macOS, Linux, and Windows.

```sh
# Verify installation
froq --version
```

## Building from source

Requirements:

- **Rust** — the toolchain is pinned by [`rust-toolchain.toml`](rust-toolchain.toml); `rustup` installs it automatically on first build.
- **DotSlash** — required so hermetic tools under [`bin/`](bin/) (notably [`bin/protoc`](bin/protoc)) can download and run.
- **protoc** — proto codegen resolves [`bin/protoc`](bin/protoc) via DotSlash, or falls back to a `protoc` on `PATH` or `$PROTOC`.

Windows, macOS, and Linux are supported build hosts. To build and run:

```sh
cargo run -p froq-pager-bin              # build + launch the TUI
cargo build -p froq-pager-bin --release  # release binary: target/release/froq-pager
cargo check -p froq-pager-bin            # fast validation
```

The binary artifact is named `froq-pager`. On first launch, it will run without forcing remote subscription check-ins.

## Documentation

The user guide ships with the pager crate:
[`crates/codegen/froq-pager/docs/user-guide/`](crates/codegen/froq-pager/docs/user-guide/)
— getting started, keyboard shortcuts, slash commands, configuration, theming, MCP servers, skills, plugins, hooks, headless mode, sandboxing, and more.

## Repository layout

| Path | Contents |
|------|----------|
| `crates/codegen/froq-pager-bin` | Composition-root package; builds the `froq-pager` binary |
| `crates/codegen/froq-pager` | The TUI: scrollback, prompt, modals, rendering |
| `crates/codegen/froq-shell` | Agent runtime + leader/stdio/headless entry points |
| `crates/codegen/froq-tools` | Tool implementations (terminal, file edit, search, ...) |
| `crates/codegen/froq-workspace` | Host filesystem, VCS, execution, checkpoints |
| `crates/codegen/...` | The rest of the CLI crate closure (config, MCP, markdown, sandbox, ...) |
| `crates/common/`, `crates/build/`, `prod/mc/` | Small shared leaf crates pulled in by the closure |
| `third_party/` | Vendored upstream source (Mermaid diagram stack) |

> [!IMPORTANT]
> The root `Cargo.toml` (workspace members, dependency versions, lints, profiles) is **generated** — treat it as read-only. Prefer editing per-crate `Cargo.toml` files.

## Development

```sh
cargo check -p <crate>        # always target specific crates; full-workspace builds are slow
cargo test -p froq-config     # per-crate tests
cargo clippy -p <crate>       # lint config: clippy.toml at the repo root
cargo fmt --all               # rustfmt.toml at the repo root
```

## Contributing

Please note that external contributions to the core agent logic are not accepted at this time.

## License

First-party code in this repository is licensed under the **Apache License, Version 2.0** — see [`LICENSE`](LICENSE).

Third-party and vendored code remains under its original licenses. See [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES) for details.
