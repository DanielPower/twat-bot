FROM rust:1.72
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/twatbot"]
