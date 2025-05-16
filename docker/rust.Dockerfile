FROM amd64/rust:alpine

WORKDIR /rust-application

COPY ./ ./

RUN cargo update \ 
cargo check \ 
cargo doc \ 
cargo test \ 
cargo build --release --target x86_64-unknown-linux-gnu \ 
mv ./target/x86_64-unknown-linux-gnu/release/rust-application ./binary

FROM amd64/alpine:latest

WORKDIR /rust-application

COPY --from=builder ./ ./

EXPOSE 8080:8080/tcp

CMD ["./binary/rust-application", "serve"]
