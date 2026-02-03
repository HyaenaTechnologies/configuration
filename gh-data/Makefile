# C Compiler Flags
CFLAGS = -Wall -Wextra

# C Compiler Analysis Flags
ANALYSIS_FLAGS = --analyze -Wall -Wextra

# Build All
all: build

# Compile Package
build: ./src/*.cc ./src/*/*.cc headers
	clang++ $(CFLAGS) ./src/*.cc ./src/*/*.cc -include-pch ./src/*/*.pch -o ./build/gh-data

# Check Package
check: ./src/*.cc ./src/*/*.cc headers
	clang++ $(ANALYSIS_FLAGS) ./src/*.cc ./src/*/*.cc -include-pch ./src/*/*.pch -o ./build/gh-data

# Clean Package
clean:
	rm ./build/*
	rm ./src/*/*.pch

# Format Package
format: ./src/*.cc ./src/*/*.cc ./src/*/*.hh 
	clang-format -i ./src/*.cc ./src/*/*.cc ./src/*/*.hh

# Compile Headers
headers: ./src/*/*.hh
	clang++ ./src/*/*.hh

# Run Package
run: build
	./build/gh-data --help

