[Fortran Language]: https://fortran-lang.org/
[Clang Compiler]: https://clang.llvm.org/docs/index.html
[CLIDoc]: https://github.com/HyaenaTechnologies/tools-utilities/blob/main/gh-sync/documentation/gh-sync.md
[GNU Compiler Collection]: https://gcc.gnu.org/onlinedocs/
[GCC Debugger]: https://www.sourceware.org/gdb/documentation/
[LLVM IR]: https://llvm.org/docs/
[Low Level Debugger]: https://lldb.llvm.org/
[Low Level Linker]: https://lld.llvm.org/
[Rust Language]: https://rust-lang.org

# GitHub Synchronize

Synchronize GitHub Forks

## Dependencies

- https://github.com/cli/cli

## Features

- Synchronize Forks

## Build

- [Fortran][Fortran Language]
- [LLVM][LLVM IR]
- [Clang][Clang Compiler]
- [LLDB][Low Level Debugger]
- [LLD][Low Level Linker]
- [GCC][GNU Compiler Collection]
- [GDB][GCC Debugger]
- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]

### Cargo Package Manager

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/gh-sync ./binary

sudo ./binary/gh-sync --h
```

### Fortran Package Manager

```shell
fpm build

fpm test

fpm run

fpm clean
```

### GNU Make

```shell
make build

make check

make test
```

### LLVM Fortran Compiler

```shell
## Compile
flang ./src/*.f90 ./src/*/*.f90 -o ./build/gh-sync

./build/gh-sync --help

## Analyze
--analyze: Run the static analyzer
-Wall: All Warnings
-Wextra: Extra Warnings
-fsanitize=address: AddressSanitizer, a memory error detector.
-fsanitize=thread: ThreadSanitizer, a data race detector.
-fsanitize=memory: MemorySanitizer, a detector of uninitialized reads. Requires instrumentation of all program code.
-fsanitize=undefined: UndefinedBehaviorSanitizer, a fast and compatible undefined behavior checker.
-fsanitize=type: TypeSanitizer, a detector for strict aliasing violations.
-fsanitize=dataflow: DataFlowSanitizer, a general data flow analysis.
-fsanitize=cfi: control flow integrity checks. Requires -flto.
-fsanitize=kcfi: kernel indirect call forward-edge control flow integrity.
-fsanitize=safe-stack: safe stack protection against stack-based memory corruption errors.
-fsanitize=realtime: RealtimeSanitizer, a real-time safety checker.
```

### GNU Fortran Compiler

```shell
## Compile
gfortran ./src/*.f90 ./src/*/*.f90 -o ./build/gh-sync

./build/gh-sync --help

## Analyze
-fanalyzer
-Wall: All Warnings
-Wextra: Extra Warnings
-Wanalyzer-allocation-size
-Wanalyzer-deref-before-check
-Wanalyzer-double-fclose
-Wanalyzer-double-free
-Wanalyzer-exposure-through-output-file
-Wanalyzer-exposure-through-uninit-copy
-Wanalyzer-fd-access-mode-mismatch
-Wanalyzer-fd-double-close
-Wanalyzer-fd-leak
-Wanalyzer-fd-phase-mismatch
-Wanalyzer-fd-type-mismatch
-Wanalyzer-fd-use-after-close
-Wanalyzer-fd-use-without-check
-Wanalyzer-file-leak
-Wanalyzer-free-of-non-heap
-Wanalyzer-imprecise-fp-arithmetic
-Wanalyzer-infinite-loop
-Wanalyzer-infinite-recursion
-Wanalyzer-jump-through-null
-Wanalyzer-malloc-leak
-Wanalyzer-mismatching-deallocation
-Wanalyzer-null-argument
-Wanalyzer-null-dereference
-Wanalyzer-out-of-bounds
-Wanalyzer-overlapping-buffers
-Wanalyzer-possible-null-argument
-Wanalyzer-possible-null-dereference
-Wanalyzer-putenv-of-auto-var
-Wanalyzer-shift-count-negative
-Wanalyzer-shift-count-overflow
-Wanalyzer-stale-setjmp-buffer
-Wanalyzer-tainted-allocation-size
-Wanalyzer-tainted-array-index
-Wanalyzer-tainted-assertion
-Wanalyzer-tainted-divisor
-Wanalyzer-tainted-offset
-Wanalyzer-tainted-size
-Wanalyzer-undefined-behavior-ptrdiff
-Wanalyzer-undefined-behavior-strtok
-Wanalyzer-unsafe-call-within-signal-handler
-Wanalyzer-use-after-free
-Wanalyzer-use-of-pointer-in-stale-stack-frame
-Wanalyzer-use-of-uninitialized-value
-Wanalyzer-va-arg-type-mismatch
-Wanalyzer-va-list-exhausted
-Wanalyzer-va-list-leak
-Wanalyzer-va-list-use-after-va-end
-Wanalyzer-write-to-const
-Wanalyzer-write-to-string-literal
```

## Install GitHub Synchronize

```shell
sudo install ./gh-sync /usr/local/bin
```

