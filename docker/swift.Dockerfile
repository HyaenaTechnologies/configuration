FROM amd64/swift:latest

WORKDIR /swift_application

COPY ./ ./

RUN swift build
RUN swift test

ENTRYPOINT ["/bin/linux/swift_application"]