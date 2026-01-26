#include "./parser.h"
#include "help_message.h"
#include "version_message.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Parse Command Line Arguments
int8_t parse_arguments(int length, char *arguments[]) {
  int8_t index;
  
  if (length < 2 || length > 2) {
    printf("\x1b[31;1;3;4mFlag and User Required:\x1b[0m\n");
    for (index = 0; index < length; index = index + 1) {
      printf("%s\n", arguments[index]);
    }
    printf("");
    print_help();
    printf("\x1b[31;1;3;4mError(1) - Exiting GitHub Data Tool\x1b[0m");
    exit(1);
  } else if (strcmp(arguments[1], "--following")) {
    
  } else if (strcmp(arguments[1], "--f")) {
    
  } else if (strcmp(arguments[1], "--repositories")) {
    
  } else if (strcmp(arguments[1], "--r")) {
    
  } else if (strcmp(arguments[1], "--starred")) {
    
  } else if (strcmp(arguments[1], "--s")) {
    
  } else if (strcmp(arguments[1], "version")) {
    print_version();
  } else if (strcmp(arguments[1], "--v")) {
    print_version();
  } else if (strcmp(arguments[1], "help")) {    print_help();
  } else if (strcmp(arguments[1], "--h")) {
    print_help();
  } else if (strcmp(arguments[1], "version")) {
    print_version();
  } else if (strcmp(arguments[1], "--v")) {
    print_version();
  } else {
    printf("\x1b[31;1;3;4mUknown Command or Flag:\x1b[0m\n"); 
    for (index = 0; index < length; index = index + 1) {
      printf("%s\n", arguments[index]);
    }
    printf("");
    print_help();
    printf("\x1b[31;1;3;4mError(1) - Exiting GitHub Data Tool\x1b[0m");
    exit(1);
  }
  
  return 0;
}

