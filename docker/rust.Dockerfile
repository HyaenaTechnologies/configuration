FROM amd64/rust:alpine

WORKDIR /rust-application

COPY ./ ./

RUN cargo test
RUN cargo build
