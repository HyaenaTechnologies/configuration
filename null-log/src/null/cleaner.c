#include "./cleaner.h"

#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>

// Clean Logs by Copying the contents of /dev/null to the Log Files
int8_t clean_logs(char log_path[]) {
  char character_buffer[256];
  DIR *directory = opendir(log_path);
  struct dirent *directory_entry = readdir(directory);
  char *error_message = strerror(errno);
  int index = 0; 

  if (directory == NULL) {
    printf(stderr, "Error Opening Directory: %s\n", error_message);
    printf("Error Value: %d\n", errno);
    printf("\x1b[31;1;3;4mError(1) - Exiting Null Log\x1b[0m");
    exit(1);
  } else {
    while (directory_entry != NULL) {
      printf(character_buffer, "cp /dev/null %s", directory_entry);
      system(character_buffer);
      while (character_buffer[index] != '\0') {
       character_buffer[index] = '\0';
       index = index + 1; 
      }
    }
  }
  
  return 0;
}

