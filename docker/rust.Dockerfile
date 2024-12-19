FROM amd64/rust:latest

WORKDIR /rust_application

COPY ./ ./

RUN cargo test
RUN cargo build

ENTRYPOINT ["/bin/linux/rust_application"]