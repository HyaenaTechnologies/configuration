# C Compiler Flags
CFLAGS = -Wall -Wextra

# C Compiler Analysis Flags
ANALYSIS_FLAGS = --analyze -Wall -Wextra

# C Compile Target Triple
TARGET_FLAG = -target x86_64-pc-linux-gnu 

# Build All
all: build

# Compile Package
build: ./src/*.c ./src/*/*.c headers
	clang $(TARGET_FLAG) $(CFLAGS) ./src/*.c ./src/*/*.c -include-pch ./src/*/*.pch -o ./build/gh-data

# Check Package
check: ./src/*.c ./src/*/*.c headers
	clang $(TARGET_FLAG) $(ANALYSIS_FLAGS) ./src/*.c ./src/*/*.c -include-pch ./src/*/*.pch

# Clean Package
clean:
	rm ./build/*
	rm ./src/*/*.pch

# Format Package
format: ./src/*.c ./src/*/*.c ./src/*/*.h 
	clang-format -i ./src/*.c ./src/*/*.c ./src/*/*.h

# Compile Headers
headers: ./src/*/*.h
	clang $(TARGET_FLAG) ./src/*/*.h

# Run Package
run: build
	./build/gh-data --h

