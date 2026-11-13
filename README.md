> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.

# snafu-gpui: Zed's GPUI for Snafu

This is a fork of [Zed](https://github.com/zed-industries/zed) that exists only to supply
GPUI to [Snafu](https://github.com/stuible/image-snafu-rust). The `snafu` branch is a Zed
stable tag plus Snafu's changes, one commit each, every one titled `snafu: …` and
explaining what it changes and why. Snafu pins one commit of this branch in its
`Cargo.toml`. Only the GPUI crates Snafu uses are built; the rest of Zed is untouched.

Current base: Zed `v1.27.0`. `git log --oneline <base tag>..snafu` lists Snafu's commits.

## Upgrading to a newer Zed release

Snafu aims to take the latest stable GPUI in every production store release. Smaller,
frequent jumps keep the rebase routine; a year-long jump means rewriting hooks.

1. **Pick the release.** Use Zed's latest *stable* tag
   (`gh release view --repo zed-industries/zed`), never a preview or `main`.
2. **Rebase.** In a checkout with `upstream` pointing at zed-industries/zed:

   ```sh
   git fetch upstream tag vX.Y.Z
   git switch snafu
   git rebase --onto vX.Y.Z <old base tag> snafu
   ```

   Resolve conflicts commit by commit, keeping each `snafu:` commit self-contained. If
   upstream now provides what a commit adds, drop that commit and note it in the next
   Snafu commit. Expect the Windows caption host commit (hooks in `gpui_windows`
   `window.rs`, `events.rs` and the run loop) to need the most care, and the others to
   apply cleanly or nearly so.
3. **Check the fork builds.** From Snafu, point its dependencies at this checkout with a
   temporary `[patch."https://github.com/stuible/snafu-gpui"]` section listing each
   GPUI crate's local path, then run `cargo check --all-targets` on macOS and
   `cargo xwin check --target x86_64-pc-windows-msvc --all-targets` for Windows.
4. **Adapt Snafu** to any GPUI API changes. Watch for behaviour changes too: the
   v1.27.0 jump surfaced test-scheduler, closed-window and key-binding changes that
   compiled fine but behaved differently.
5. **Publish and pin.** Force-push `snafu` here (the branch is rewritten on every
   rebase; old pinned commits stay reachable through the `snafu-<base>` tag below), tag
   it, then set Snafu's `rev` to the new commit and remove the temporary patch section:

   ```sh
   git push --force-with-lease origin snafu
   git tag snafu-vX.Y.Z snafu && git push origin snafu-vX.Y.Z
   ```

6. **Refresh Snafu's records:** regenerate `third_party/gpui/SNAFU_GPUI.md` (base tag,
   pinned commit and `git log --format='- `%h` %s' vX.Y.Z..snafu`) and run
   `python3 scripts/rust_notices.py`.
7. **Verify** with Snafu's full before-push checks (fmt, macOS and Windows Clippy,
   script tests, `cargo test`), then launch the app on macOS and install a Windows QA
   package: the native caption, its buttons and dragging, menus, the brush and
   eyedropper cursors, file and drag opening, Help, and rendering.

## Notes

- Zed's GPUI still declares version `0.2.2`; Snafu's dependency gives
  `version = "0.2.2"` so Cargo picks it over a lint fixture with the same name.
- Snafu enables `gpui_platform`'s `font-kit` (macOS draws no text without it) and
  `runtime_shaders` (no Metal toolchain or fxc needed at build time), and turns off its
  default `windows-manifest` because Snafu embeds its own.
- Patches that would suit upstream should be proposed there; each accepted one is a
  commit this branch no longer carries.

---

# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

---

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

### Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... we're hiring! Check out our [jobs](https://zed.dev/jobs) page for open roles.

### Licensing

Zed source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Zed is developed by **Zed Industries, Inc.**, a for-profit company.

If you’d like to financially support the project, you can do so via GitHub Sponsors.
Sponsorships go directly to Zed Industries and are used as general company revenue.
There are no perks or entitlements associated with sponsorship.
