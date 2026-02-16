#include "./parser.h"

#include "../github/following.h"
#include "../github/repositories.h"
#include "../github/stars.h"
#include "./help_message.h"
#include "./version_message.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Parse Command Line Arguments
uint8_t parse_arguments(int length, char *arguments[]) {
  int index;

  if (length == 1 || length > 2) {
    printf("\x1b[31;1;3;4mFlag and User Required:\x1b[0m\n\n");
    for (index = 0; index < length; index = index + 1) {
      printf("%s\n\n", arguments[index]);
    }
    print_help();
    exit(EXIT_FAILURE);
  } else if (strcmp(arguments[1], "--following") == 0) {
    write_following();
  } else if (strcmp(arguments[1], "--f") == 0) {
    write_following();
  } else if (strcmp(arguments[1], "help") == 0) {
    print_help();
  } else if (strcmp(arguments[1], "--h") == 0) {
    print_help();
  } else if (strcmp(arguments[1], "--repositories") == 0) {
    write_repositories();
  } else if (strcmp(arguments[1], "--r") == 0) {
    write_repositories();
  } else if (strcmp(arguments[1], "--starred") == 0) {
    write_starred();
  } else if (strcmp(arguments[1], "--s") == 0) {
    write_starred();
  } else if (strcmp(arguments[1], "version") == 0) {
    print_version();
  } else if (strcmp(arguments[1], "--v") == 0) {
    print_version();
  } else {
    printf("\x1b[31;1;3;4mUknown Command or Flag:\x1b[0m\n\n");
    for (index = 0; index < length; index = index + 1) {
      printf("%s\n\n", arguments[index]);
    }
    print_help();
    exit(EXIT_FAILURE);
  }

  return 0;
}
