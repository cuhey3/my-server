FROM rust:1.94

WORKDIR /usr/src/app
COPY . .

RUN cargo build --bin matching-server --release

CMD ["./target/release/matching-server"]