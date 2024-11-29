FROM amd64/rust:latest

WORKDIR /rust_application

COPY ./ ./

RUN cargo build
RUN cargo test

ENTRYPOINT ["/bin/linux/rust_application"]