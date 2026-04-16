[C Language]: https://www.c-language.org/
[Clang Compiler]: https://clang.llvm.org/docs/index.html
[GNU Compiler Collection]: https://gcc.gnu.org/onlinedocs/
[GCC Debugger]: https://www.sourceware.org/gdb/documentation/
[GNU Make]: https://www.gnu.org/software/make/manual/html_node/index.html
[KitWare Make]: https://cmake.org/
[LLVM IR]: https://llvm.org/docs/
[Low Level Debugger]: https://lldb.llvm.org/
[Low Level Linker]: https://lld.llvm.org/

# Build

- [C][C Language]
- [LLVM][LLVM IR]
- [Clang][Clang Compiler]
- [LLDB][Low Level Debugger]
- [LLD][Low Level Linker]
- [GCC][GNU Compiler Collection]
- [GDB][GCC Debugger]
- [Make][GNU Make]
- [CMake][KitWare Make]

## Platforms

- Unix

## Dependencies

- https://github.com/cli/cli

## CMake

```shell
cmake -B ./build

cd ./build

make

./gh-data --help
```

## GNU Make

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

