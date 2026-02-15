#include "./arguments/parser.h"

#include <stdlib.h>

// Main Entry Point
int main(int argc, char *argv[]) {

  parse_arguments(argc, argv);
  exit(EXIT_SUCCESS);

  return 0;
}
