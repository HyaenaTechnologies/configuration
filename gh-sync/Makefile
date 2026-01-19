# C Compiler Flags
CFLAGS = -Wall -Wextra

# C Compiler Analysis Flags
ANALYSIS_FLAGS = -Wall -Wextra -fanalyzer

# Build All
all: build

# Compile Package
build: ./src/*.f90
	gfortran $(CFLAGS) ./src/*.f90 ./src/*.f90 -o ./build/gh-sync

# Check Package
check: ./src/*.f90
	gfortran $(ANALYSIS_FLAGS) ./src/*.f90 ./src/*/*.f90 -o ./build/gh-sync

# Clean Package
clean:
	rm ./build/*

# Run Package
run: build
	./build/gh-sync

# Test Package
test: ./tests/*.f90

