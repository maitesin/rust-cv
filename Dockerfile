FROM rust:1.19.0
WORKDIR /usr/src/rust-cv
COPY . .
RUN cargo install
CMD ["rust-cv"]
