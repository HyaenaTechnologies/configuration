.PHONY: build
build:
	GOARCH=amd64 GOOS=linux go build -o ./binary/upr ./source/main.go
