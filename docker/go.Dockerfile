FROM amd64/go:latest

WORKDIR /go-application

COPY ./ ./

RUN go env
RUN go list
RUN go vet
RUN go fix
RUN go fmt
RUN go mod verify
RUN go mod tidy
RUN go mod graph
RUN go test
RUN go doc
RUN GOARCH=amd64 GOOS=linux go build -o ./binary/go-application ./source/main.go
