[CLIDoc]: https://github.com/HyaenaTechnologies/tools-utilities/blob/main/null-log/documentation/null-log.md
[Rust Language]: https://rust-lang.org

# Null Log

Clean Log File Contents - Retain Log Files

## Dependencies

- Unix /dev/null

## Features

- Clean Logs
- Retain Logs

## Build

- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/null-log ./binary

./binary/null-log --h
```

## Install Null Log

```shell
mv ./null-log ~/
```

