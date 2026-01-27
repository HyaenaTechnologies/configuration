#include "./cleaner.h"

#include <dirent.h>
#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

// Check if a Document is a Regular File
int is_file(struct dirent *directory_entry) {
  struct stat status_buffer;
  int file_status = stat(directory_entry->d_name, &status_buffer);
  if (file_status != 0) {
    return S_ISREG(status_buffer.st_mode);
  }
  return file_status;
}

// Clean Logs by Copying the contents of /dev/null to the Log Files
int8_t clean_logs(char log_path[]) {
  DIR *directory = opendir(log_path);
  struct dirent *directory_entry = readdir(directory);
  char *error_message = strerror(errno);
  struct stat status_buffer;

  if (directory == NULL) {
    printf(stderr, "Error Opening Directory: %s\n", error_message);
    printf("Error Value: %d\n", errno);
    printf("\x1b[31;1;3;4mError(1) - Exiting Null Log\x1b[0m");
    exit(1);
  } else {
    while (directory_entry != NULL) {
      if (is_file(directory_entry)) {
        char character_buffer[256] = "";
        printf(character_buffer, "cp /dev/null %s", directory_entry);
        system(character_buffer);
      } else {
        printf(stderr, "Directory Entry is not a File: Skipping...");
      }
    }
  }

  return 0;
}
