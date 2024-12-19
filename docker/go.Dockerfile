FROM amd64/go:latest

WORKDIR /go_application

COPY ./ ./

RUN go test
RUN go build

ENTRYPOINT ["/bin/linux/go_application"]