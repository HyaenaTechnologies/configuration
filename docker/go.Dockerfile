FROM amd64/go:alpine

WORKDIR /go-application

COPY ./ ./

RUN go env \ 
go list \ 
go vet \ 
go fix \ 
go fmt \ 
go mod verify \ 
go mod tidy \ 
go mod graph \ 
go test \ 
go doc \ 
GOARCH=amd64 GOOS=linux go build -o ./binary/go-application ./source/main.go

FROM amd64/alpine:latest

WORKDIR /go-application

COPY --from=builder ./binary ./binary

EXPOSE 8080:8080/tcp

CMD ["./binary/go-application", "serve"]
