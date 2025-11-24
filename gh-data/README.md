[CLIDoc]: https://github.com/HyaenaTechnologies/tools-utilities/blob/main/gh-data/documentation/gh-data.md
[Rust Language]: https://rust-lang.org

# GitHub Data

Write GitHub Profile Data to Markdown Files

## Dependencies

- https://github.com/cli/cli

## Features

- GitHub Following
- GitHub Repositories
- Git Stars

## Build

- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/gh-data ./binary

sudo ./binary/gh-data --h
```

## Install System Update Daemon

```shell
sudo install ./gh-data /usr/local/bin/
```

