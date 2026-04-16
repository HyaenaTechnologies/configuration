# CMAKE Minimum Version
cmake_minimum_required(VERSION "4.0")

# Project Declaration
project(
  "GitHubData"
  DESCRIPTION "Write GitHub Profile Data to Markdown Files"
  VERSION "1.0.0"
  LANGUAGES "C"
)

# C Compiler
set(CMAKE_C_COMPILER "/usr/bin/clang")

# C Compiler Flags
set(CMAKE_C_FLAGS "-Wall -Wextra")

# Source Files
file(
  GLOB
  SOURCE_FILES
    "./src/*.c"
    "src/*/*.c"
)

# Header Files
file(
  GLOB
  HEADER_FILES "src/*/*.h"
)

# Target Executable
add_executable("gh-data")

# Target Sources
target_sources(
  "gh-data"
  PUBLIC
    ${SOURCE_FILES}
)

# Target Headers
target_precompile_headers(
  "gh-data"
  PUBLIC
    ${HEADER_FILES}
)

