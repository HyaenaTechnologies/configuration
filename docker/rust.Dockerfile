FROM amd64/rust:alpine

WORKDIR /rust-application

COPY ./ ./

RUN cargo test \ 
cargo build


FROM amd64/alpine:latest

WORKDIR /rust-application

COPY --from=builder ./ ./

RUN ./binary/rust-application
