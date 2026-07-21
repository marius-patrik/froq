# Fork notes

This is a fork of [xai-org/grok-build](https://github.com/xai-org/grok-build) —
xAI's Grok Build coding agent (Rust, Apache-2.0). It exists so the CLI can be
modified locally and rebuilt on Windows, replacing the vendor-installed binary
under `~/.grok/bin/`.

## Upstream does not accept contributions

`CONTRIBUTING.md` states the repo takes no external pull requests or patches,
issues and discussions are **disabled** on the upstream repo, no CLA is offered,
and upstream has never received a PR. `SECURITY.md` routes vulnerabilities to
HackerOne only — it is not a channel for build bugs.

So fixes below are carried here indefinitely. They are written to be
upstream-quality anyway, on their own branches, in case that policy ever
changes.

## Carried patches

Each lives on its own branch off `main` so it stays a clean, sendable diff.

### `fix/windows-protoc-dev-stdout`

`crates/build/xai-proto-build/src/lib.rs` invoked protoc with
`--dependency_out=/dev/stdout` and `--descriptor_set_out=/dev/null`. Neither
path exists on Windows, so protoc exited non-zero and every crate whose build
script calls `compile_protos()` panicked:

```
/dev/stdout: No such file or directory
called `Result::unwrap()` on an `Err` value: protoc command failed
```

Fixed by routing both through real files under `OUT_DIR` and reading the
dependency manifest from disk. Unix behavior is unchanged — protoc echoes the
descriptor path verbatim as the make-rule target on every platform, so the
existing parsing still applies. Verified on Windows; **not yet exercised on
Linux or macOS**, which matters if this is ever sent upstream.

## Building on Windows

Prerequisites, none of which upstream documents for this platform:

- **MSVC C++ toolchain** — Visual Studio 2022 Build Tools with
  `Microsoft.VisualStudio.Workload.VCTools`. `rust-toolchain.toml` lists only
  Linux targets, so the MSVC path is not exercised upstream.
- **Rust 1.92.0** — pinned by `rust-toolchain.toml`; rustup installs it
  automatically inside this tree.
- **A real `protoc`** — the vendored `bin/protoc` is a 1.6 KB
  [dotslash](https://dotslash-cli.com) manifest, not a binary. Without dotslash
  installed it cannot execute. Install protoc separately and point `$PROTOC` at
  it.

Then:

```powershell
$env:PROTOC = "<path to>\protoc.exe"
cargo build -p xai-grok-pager-bin --release `
  --config profile.release.debug=0 `
  --config profile.release.strip=true `
  --config profile.release.incremental=false `
  --config profile.release.codegen-units=16
```

**The `--config` flags are required, not optional.** The workspace sets
`incremental = true` in `[profile.release]`, which makes Cargo use 256 codegen
units. Linking 257 objects' worth of debug types overflows the PDB type stream
and the build dies at the very end with:

```
LINK : fatal error LNK1318: Unexpected PDB error; LIMIT (12)
```

This is harmless on Linux and macOS, which do not use PDB at all. The flags are
passed on the command line rather than committed to `Cargo.toml` because
lowering codegen units and dropping debug info would be the wrong default for
Linux builds.

The binary is produced as `target/release/xai-grok-pager.exe`. The vendor
install uses two byte-identical copies of it, `grok.exe` and `agent.exe`.

## Syncing with upstream

```sh
git fetch upstream
git merge upstream/main        # on main
```

The `upstream` remote is fetch-only — its push URL is set to `DISABLED` so a
stray `git push upstream` cannot fire. After a sync, re-apply or rebase each
carried branch above and confirm the Windows build still links.
