#include "./version_message.h"

#include <stdint.h>
#include <stdio.h>

// Print Version Message
uint8_t print_version(void) {
  char version_number[6] = u8"1.0.0";

  printf("\x1b[32;1;3;4mGithub Data Tool\x1b[0m\n\n");
  printf("\x1b[32;1;3mVersion Number:%s\x1b[0m\n", version_number);

  return 0;
}
