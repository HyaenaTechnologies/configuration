FROM amd64/rust:latest

WORKDIR /rust-application

COPY ./ ./

RUN cargo test
RUN cargo build
