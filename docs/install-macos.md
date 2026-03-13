# `unfk` macOS Installation Guide

## Homebrew

```bash
brew tap keathmilligan/tap
brew install keathmilligan/tap/unfk
```

Stay up-to-date with `brew upgrade unfk`.

## Shell Installer

```bash
curl -fsSL https://packages.keathmilligan.net/unfk/install.sh | sh
```

This will install `unfk` into `~/.local/bin`.

## cargo

If you have Rust development tools installed:

```bash
cargo install unfk
```

## dmg Installer

Download the signed `.dmg` installer for your platform architecture (Apple Silicon `aarch64` or Intel `x86_64`) directly from the [GitHub Releases](https://github.com/keathmilligan/unfk/releases) page.

## Binary

Download the macOS binary archive for your architecture (Apple Silicon `aarch64` or Intel `x86_64`) from the [GitHub Releases](https://github.com/keathmilligan/unfk/releases) page.

Extract the `unfk` binary into a directory in your `PATH`.

