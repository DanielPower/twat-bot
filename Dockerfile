FROM rust:1.72
COPY ./ ./
RUN cargo build --release
RUN cargo install sqlx-cli --no-default-features --features sqlite
CMD ["/bin/bash", "-c", "./start.sh"]
