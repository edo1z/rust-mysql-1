FROM rust:1.58.0
WORKDIR /app
RUN cargo install cargo-watch
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["/usr/local/bin/api"]