#include "./following.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Write GitHub Following Data to a Markdown File
uint8_t write_following() {
  char command_buffer[256];
  char output_buffer[256];
  FILE *markdown_file = fopen("./following.md", "w");

  printf("Enter Github API User Command: ");
  fgets(command_buffer, sizeof(command_buffer), stdin);
  
  command_buffer[strcspn(command_buffer, "\n")] = '\0';

  if (markdown_file == NULL) {
    perror("Failed to Open File");
    exit(EXIT_FAILURE);
  }

  fprintf(markdown_file, "# Following\n\n");
  
  FILE *github_cli = popen(command_buffer, "r");

  if (github_cli == NULL) {
    perror("Failed to Open File");
    exit(EXIT_FAILURE);
  } else {
    while (fgets(output_buffer, sizeof(output_buffer), github_cli) != NULL) {
      printf("%s", output_buffer);
      fprintf(markdown_file, "- %s", output_buffer);
    }

    if (feof(github_cli)) {
      printf("End of file Reached");
    }
  }

  pclose(github_cli);
  fclose(markdown_file);

  return 0;
}
