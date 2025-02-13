#!/bin/zsh

GOARCH=amd64 GOOS=linux go build -o ./binary/htdinet ./source/main.go
