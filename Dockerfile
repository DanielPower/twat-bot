FROM rust:1.78
COPY ./ ./
RUN cargo build --release
CMD ["/bin/bash", "-c", "./start.sh"]
