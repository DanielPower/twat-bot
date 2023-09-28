FROM rust:1.72
COPY ./ ./
RUN cargo build --release
RUN cargo install sqlx-cli --no-default-features --features sqlite
RUN sqlx database create
RUN sqlx migrate run
CMD ["./target/release/twatbot"]
