[CLIDoc]: https://github.com/HyaenaTechnologies/configuration/blob/main/upd/documentation/upd.md
[Fleet]: https://www.jetbrains.com/fleet/
[Rust Language]: https://rust-lang.org
[RustRover]: https://jetbrains.com/rust
[VSCode]: https://code.visualstudio.com/doc/

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
- [JetBrains Fleet][Fleet]
- [JetBrains RustRover][RustRover]
- [Visual Studio Code][VSCode]

## Build System Update Daemon

```shell
git clone

cargo build --release --target x86_64-unknown-linux-gnu
mv ./target/release/upd ./binary
./binary/upd
```

## Install System Update Daemon

```shell
echo 'export PATH="$PATH:/usr/local/bin/upd"' >> ~/.bashrc
```
