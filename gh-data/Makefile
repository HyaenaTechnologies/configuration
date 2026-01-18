# C Compiler Flags
CFLAGS = -Wall -Wextra

# C Compiler Analysis Flags
ANALYSIS_FLAGS = -Wall -Wextra -fanalyzer

# Build All
all: build

# Compile Package
build: ./src/*.f90
	gfortran $(CFLAGS) ./src/*.f90 ./src/*.f90 -o ./build/gh-data

# Check Package
check: ./src/*.f90
	gfortran $(ANALYSIS_FLAGS) ./src/*.f90 ./src/*/*.f90 -o ./build/gh-data

# Clean Package
clean:
	cd ./build
	rm ./*

# Run Package
run: build
	cd ./build
	./gh-data

# Test Package
test: ./tests/*.f90

