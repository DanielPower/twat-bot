FROM rust:1.78
COPY ./ ./
RUN cargo build --release
RUN cargo install sqlx-cli --no-default-features --features sqlite
CMD ["/bin/bash", "-c", "./start.sh"]
