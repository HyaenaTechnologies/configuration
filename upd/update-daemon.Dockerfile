FROM rust:alpine

WORKDIR /upd

COPY ./ ./

RUN cargo check \ 
cargo build --release --target x86_64-unknown-linux-musl \ 
mv ./target/x86_64-unknown-linux-gnu/release/upd ./binary

FROM alpine:latest

WORKDIR /upd

COPY --from=builder ./binary ./binary 

