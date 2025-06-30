#!/bin/env bash

# Run Git Pull Origin in All Git Repositories in the Current Directory and Print the Output
find ./ -name .git -execdir git pull origin \; | echo

