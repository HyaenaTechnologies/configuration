FROM amd64/rust:alpine

WORKDIR /rust-application

COPY ./ ./

RUN cargo test \ 
cargo build
