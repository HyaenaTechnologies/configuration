FROM amd64/rust:alpine

WORKDIR /rust-application

COPY ./ ./

RUN cargo update \ 
cargo check \ 
cargo doc \ 
cargo test \ 
cargo build --release \ 
mv ./target/release/rust-application ./binary

FROM amd64/alpine:latest

WORKDIR /rust-application

COPY --from=builder ./ ./

EXPOSE 80:8080/tcp

RUN ./binary/rust-application
