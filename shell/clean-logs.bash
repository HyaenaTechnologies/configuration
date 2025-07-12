#!/bin/env bash

# Clean Logs
for LOG_FILE in $(find /var/log/ -type f)
do
    cp /dev/null ${LOG_FILE}
done

