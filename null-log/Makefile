# C Compiler Flags
CFLAGS = -Wall -Wextra

# C Compiler Analysis Flags
ANALYSIS_FLAGS = -Wall -Wextra -fanalyzer

# Build All
all: build

# Compile Package
build: ./src/*.f90
	gfortran $(CFLAGS) ./src/*.f90 ./src/*.f90 -o ./build/null-log

# Check Package
check: ./src/*.f90
	gfortran $(ANALYSIS_FLAGS) ./src/*.f90 ./src/*/*.f90 -o ./build/null-log

# Clean Package
clean:
	rm ./build/*

# Run Package
run: build
	cd ./build
	./null-log

# Test Package
test: ./tests/*.f90

