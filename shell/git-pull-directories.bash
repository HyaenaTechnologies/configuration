#!/bin/env bash

# Run Git Pull in Every Git Repository in the Current Directory and Print the Output
find ./. -name .git -execdir git pull origin \; | echo

