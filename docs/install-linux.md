# `unfk` Linux Installation Guide

Choose the best installation option for your distro.

## Shell Installer (all distros)

```bash
curl -fsSL https://packages.keathmilligan.net/unfk/install.sh | sh
```

This will install `unfk` into `~/.local/bin`.

## cargo (all distros)

If you have Rust development tools installed:

```bash
cargo install unfk
```

## Homebrew (all distros)

Homebrew is also supported on Linux. If you have it installed:

```bash
brew tap keathmilligan/tap
brew install keathmilligan/tap/unfk
```

## apt (Debian / Ubuntu)

```bash
curl -fsSL https://install.keathmilligan.dev/gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/keathmilligan.gpg
echo "deb [signed-by=/etc/apt/keyrings/keathmilligan.gpg] https://install.keathmilligan.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/keathmilligan.list
sudo apt update
sudo apt install unfk
```

Stay up to date with:

```
sudo apt upgrade unfk
```

## dnf / rpm (Fedora / RHEL / CentOS)

```bash
sudo curl -o /etc/yum.repos.d/keathmilligan.repo https://install.keathmilligan.dev/rpm/keathmilligan.repo
sudo dnf install unfk
```

Stay up to date with:

```
sudo dnf upgrade unfk
```

## AUR (Arch Linux)

```bash
yay -S unfk-bin
```

## Binary

Download the linux binary archive for your architecture (Intel `x86_64` or ARM `aarch64`) from the [GitHub Releases](https://github.com/keathmilligan/unfk/releases) page.

Extract the `unfk` binary into a directory in your `PATH`.

