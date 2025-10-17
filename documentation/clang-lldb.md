# Clang Notes

## Clang Analysis Flags

```shell
--analyze: Run the static analyzer
```

## Clang Compilation Flags

```shell
-CC: Include comments from within macros in preprocessed output
-C: Include comments in preprocessed output
-c: Only run preprocess, compile, and assemble steps
-o <file>: Write output to <file>
-include-pch <file>: Include precompiled header file
-include <file>: Include file before parsing
````

## Clang Diagnostics Flags

```shell
-fdiagnostics-absolute-paths: Print absolute paths in diagnostics
-fdiagnostics-show-hotness: Enable profile hotness information in diagnostic line
-fdiagnostics-show-note-include-stack: Display include stacks for diagnostic notes
-fdiagnostics-show-option: Print option name with mappable diagnostics
-fdiagnostics-show-template-tree: Print a template comparison tree for differing templates
```

## Clang Profiling Flags

```shell
-fmemory-profile: Enable heap memory profiling
```

## Clang Sanitization Flags

```shell
-fsanitize-address-use-after-scope: Enable use-after-scope detection in AddressSanitizer
-fsanitize-address-use-odr-indicator: Enable ODR indicator globals to avoid false ODR violation reports in partially sanitized programs at the cost of an increase in binary size
-fsanitize-cfi-cross-dso: Enable control flow integrity (CFI) checks for cross-DSO calls.
-fsanitize-memory-param-retval: Enable detection of uninitialized parameters and return values
-fsanitize-memory-track-origins: Enable origins tracking in MemorySanitizer
-fsanitize-memory-use-after-dtor: Enable use-after-destroy detection in MemorySanitizer
````

## Clang Warning Flags

```shell
-Wall: Enable All Warnings
-Wextra: Enable Extra Warnings
````

# LLDB Notes

## LLDB Attatching Flags

```shell
--attach-name <name>: Tells the debugger to attach to a process with the given name.
--attach-pid <pid>: Tells the debugger to attach to a process with the given pid.
-n <value>: Alias for --attach-name
-p <value>:  Alias for --attach-pid
--wait-for: Tells the debugger to wait for a process with the given pid or name to launch before attaching.
-w: Alias for --wait-for
````

## LLDB Option Flags

```shell
--arch <architecture>: Tells the debugger to use the specified architecture when starting and running the program.
-a <value>: Alias for --arch
--debug: Tells the debugger to print out extra information for debugging itself.
-d: Alias for --debug
--file <filename>: Tells the debugger to use the file "\<filename\>" as the program to be debugged.
-f <value>: Alias for --file
```

## LLDB REPL Flags

```shell
--repl: Runs lldb in REPL mode with a stub process.
-r: Alias for --repl
```

## LLDB Examples

```shell
EXAMPLES:
  The debugger can be started in several modes.
  
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
```

