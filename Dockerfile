FROM rust:jessie
RUN rustup target add i686-linux-musl
COPY src /src
COPY Cargo.toml /
RUN cargo build --target i686-linux-musl --release

FROM scratch
COPY --from=0 target/i686-linux-musl/release/rust-cv /
CMD ["/rust-cv"]
