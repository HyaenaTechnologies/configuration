#include "./help_message.h"

#include <stdint.h>
#include <stdio.h>

// Print Help Message
uint8_t print_help(void) {
  printf("\t\x1b[32;1;3;4mGitHub Data Tool\x1b[0m\n\n"
         "\x1b[32;1;3mCommands:\tDescription:\x1b[0m\n\n"
         "\x1b[32;3mhelp\x1b[0m\t\tPrint Commands and Flags\n"
         "\x1b[32;3mversion\x1b[0m\t\tPrint Version Number\n\n"
         "\x1b[32;1;3mFlags:\t\tDescription:\x1b[0m\n\n"
         "\x1b[32;3m--following\x1b[0m\tWrite Following Data\n"
         "\x1b[32;3m--f\x1b[0m\t\tWrite Following Data\n"
         "\x1b[32;3m--h\x1b[0m\t\tPrint Commands and Flags\n"
         "\x1b[32;3m--repositories\x1b[0m\tWrite Repository Data\n"
         "\x1b[32;3m--r\x1b[0m\t\tWrite Repository Data\n"
         "\x1b[32;3m--starred\x1b[0m\tWrite Starred Data\n"
         "\x1b[32;3m--s\x1b[0m\t\tWrite Starred Data\n"
         "\x1b[32;3m--v\x1b[0m\t\tPrint Version Number\n");

  return 0;
}
