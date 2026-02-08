#ifndef CLEANER_H
#define CLEANER_H

#include <stdint.h>

// Clean Logs by Copying the contents of /dev/null to the Log Files
uint8_t clean_logs(char log_path[]);

#endif
