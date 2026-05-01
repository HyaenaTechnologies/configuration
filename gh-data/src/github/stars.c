#include "./stars.h"

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Write GitHub Stars Data to a ASCII Doc File
uint8_t write_stars() {
  char command_buffer[256];
  char output_buffer[512];
  size_t command_length = strnlen(command_buffer, sizeof(command_buffer));

  printf("Enter Github API User Command: ");

  if (fgets(command_buffer, sizeof(command_buffer), stdin) == NULL) {
    perror("Error Reading Input");
    exit(EXIT_FAILURE);
  } else if (command_length > 256) {
    perror("Command Too Long");
    exit(EXIT_FAILURE);
  }

  FILE *adoc_file = fopen("./stars.adoc", "w");

  if (adoc_file == NULL) {
    perror("Failed to Open File");
    exit(EXIT_FAILURE);
  } else {
    fprintf(adoc_file, "= Stars\n\n");
  }

  FILE *github_cli = popen(command_buffer, "r");

  if (github_cli == NULL) {
    perror("Failed to Open File");
    exit(EXIT_FAILURE);
  } else {
    while (fgets(output_buffer, sizeof(output_buffer), github_cli) != NULL) {
      // Print Output Line by Line
      printf("%s", output_buffer);
      // Write Output to the ASCII Doc File, Line by Line
      fprintf(adoc_file, "* %s", output_buffer);
      fprintf(adoc_file, "");
    }

    if (feof(github_cli)) {
      printf("\n\nEnd of file Reached");
    }
  }

  pclose(github_cli);
  fclose(adoc_file);

  return 0;
}
