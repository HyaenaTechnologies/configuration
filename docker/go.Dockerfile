FROM amd64/go:latest

WORKDIR /go-application

COPY ./ ./

RUN go test
RUN GOOS=linux GOARCH=amd64 go build -o ./binary/go-application ./source/main.go
