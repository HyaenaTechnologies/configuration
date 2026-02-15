# C Compiler Flags
CFLAGS = -Wall -Wextra

# C Compiler Analysis Flags
ANALYSIS_FLAGS = --analyze -Wall -Wextra

# Build All
all: build

# Compile Package
build: ./src/*.c ./src/*/*.c headers
	clang $(CFLAGS) ./src/*.c ./src/*/*.c -include-pch ./src/*/*.pch -o ./build/gh-data

# Check Package
check: ./src/*.c ./src/*/*.c headers
	clang $(ANALYSIS_FLAGS) ./src/*.c ./src/*/*.c -include-pch ./src/*/*.pch

# Clean Package
clean:
	rm ./build/*
	rm ./src/*/*.pch

# Format Package
format: ./src/*.c ./src/*/*.c ./src/*/*.h 
	clang-format -i ./src/*.c ./src/*/*.c ./src/*/*.h

# Compile Headers
headers: ./src/*/*.h
	clang ./src/*/*.h

# Run Package
run: build
	./build/gh-data --h

