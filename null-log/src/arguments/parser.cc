#include "./parser.hh"
#include "../null/cleaner.hh"
#include "./help_message.hh"
#include "./version_message.hh"

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Parse Command Line Arguments
uint8_t parse_arguments(size_t length, char *arguments[]) {
  int8_t index;

  if (length < 1 || length > 1) {
    printf("\x1b[31;1;3;4mCommand or Flag Required but not Both:\x1b[0m\n");
    for (index = 0; index < length; index = index + 1) {
      printf("%s\n", arguments[index]);
    }
    printf("");
    print_help();
    printf("\x1b[31;1;3;4mError(1) - Exiting Null Log\x1b[0m");
    exit(1);
  } else if (strcmp(arguments[1], "help")) {
    print_help();
  } else if (strcmp(arguments[1], "--h")) {
    print_help();
  } else if (strcmp(arguments[1], "version")) {
    print_version();
  } else if (strcmp(arguments[1], "--v")) {
    print_version();
  } else {
    clean_logs(arguments[1]);
  }

  return 0;
}
