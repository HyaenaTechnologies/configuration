[CLIDoc]: https://github.com/HyaenaTechnologies/configuration/blob/main/upd/documentation/upd.md
[Rust Language]: https://rust-lang.org

# System Update Daemon

System Update Daemon for the Advanced Package Tool, the Dandified Yellowdog Updater Modified, and Ubuntu Snap

## Features

- APT Upgrade
- DNF Upgrade
- DNF System Release Upgrade
- Snap Refresh
- Ubuntu System Releade Upgrade

## Build

- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]

## Build System Update Daemon

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/upd ./binary

./binary/upd
```

## Install System Update Daemon

```shell
sudo install ./upd /usr/local/bin/
```

