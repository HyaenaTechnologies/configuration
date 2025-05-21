#!/bin/env bash

for LOG_FILES in $(find /var/log/ -type f)
do
    cp /dev/null $LOG_FILES
done

