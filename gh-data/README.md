[C Language]: https://www.c-language.org/
[Clang Compiler]: https://clang.llvm.org/docs/index.html
[CLIDoc]: https://github.com/HyaenaTechnologies/configuration/blob/main/gh-data/documentation/gh-data.md
[GNU Compiler Collection]: https://gcc.gnu.org/onlinedocs/
[GCC Debugger]: https://www.sourceware.org/gdb/documentation/
[GNU Make]: https://www.gnu.org/software/make/manual/html_node/index.html
[LLVM IR]: https://llvm.org/docs/
[Low Level Debugger]: https://lldb.llvm.org/
[Low Level Linker]: https://lld.llvm.org/

# GitHub Data

Write GitHub Profile Data to Markdown Files

## Dependencies

- https://github.com/cli/cli

## Features

- GitHub Following
- GitHub Repositories
- Git Stars

## Platforms

- Unix

## Build

- [C][C Language]
- [LLVM][LLVM IR]
- [Clang][Clang Compiler]
- [LLDB][Low Level Debugger]
- [LLD][Low Level Linker]
- [GCC][GNU Compiler Collection]
- [GDB][GCC Debugger]
- [Make][GNU Make]
- [Command Line Documentation][CLIDoc]

### GNU Make

```shell
make build

make check

make clean

make format

make run
```

## LLVM Clang

```shell
## Compile
clang ./src/*/*.h

clang ./src/*.c ./src/*/*.c -include-pch ./src/*/*.pch -o ./build/gh-data

rm ./src/*/*.pch

./build/gh-data

## Analyze
--analyze: Run the static analyzer
-Wall: All Warnings
-Wextra: Extra Warnings
```

## GNU Compiler Collection

```shell
## Compile
gcc ./src/*/*.h

gcc ./src/*.c ./src/*/*.c -include ./src/*/*.pch -o ./build/gh-data

rm ./src/*/*.pch

./build/gh-data

## Analyze
-fanalyzer
-Wall: All Warnings
-Wextra: Extra Warnings
```

## Install GitHub Data

```shell
sudo install ./gh-data /usr/local/bin/
```

