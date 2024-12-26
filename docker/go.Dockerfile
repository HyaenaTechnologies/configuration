FROM amd64/go:latest

WORKDIR /go-application

COPY ./ ./

RUN go test
RUN go build
