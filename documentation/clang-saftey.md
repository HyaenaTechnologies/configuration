# Clang Safety Flags

- --analyze: Run the static analyzer
- -fsanitize-address-globals-dead-stripping: Enable linker dead stripping of globals in AddressSanitizer
- -fsanitize-address-use-after-scope: Enable use-after-scope detection in AddressSanitizer
- -fsanitize-address-use-odr-indicator: Enable ODR indicator globals to avoid false ODR violation reports in partially sanitized programs at the cost of an increase in binary size
- -fsanitize-cfi-cross-dso: Enable control flow integrity (CFI) checks for cross-DSO calls.
- -fsanitize-memory-param-retval: Enable detection of uninitialized parameters and return values
- -fsanitize-memory-track-origins: Enable origins tracking in MemorySanitizer
- -fsanitize-memory-use-after-dtor: Enable use-after-destroy detection in MemorySanitizer
- -Wall
- -Wextra

# LLDB Notes

- The debugger can be started in several modes.

  Passing an executable as a positional argument prepares lldb to debug the
  given executable. To disambiguate between arguments passed to lldb and
  arguments passed to the debugged executable, arguments starting with a - must
  be passed after --.

    lldb --arch x86_64 /path/to/program program argument -- --arch armv7

  For convenience, passing the executable after -- is also supported.

    lldb --arch x86_64 -- /path/to/program program argument --arch armv7

  Passing one of the attach options causes lldb to immediately attach to the
  given process.

    lldb -p <pid>
    lldb -n <process-name>

  Passing --repl starts lldb in REPL mode.

    lldb -r

  Passing --core causes lldb to debug the core file.

    lldb -c /path/to/core

  Command options can be combined with these modes and cause lldb to run the
  specified commands before or after events, like loading the file or crashing,
  in the order provided on the command line.

    lldb -O 'settings set stop-disassembly-count 20' -o 'run' -o 'bt'
    lldb -S /source/before/file -s /source/after/file
    lldb -K /source/before/crash -k /source/after/crash

  Note: In REPL mode no file is loaded, so commands specified to run after
  loading the file (via -o or -s) will be ignored.

