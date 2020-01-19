FROM rust:1.40-stretch
RUN rustup target add i686-unknown-linux-musl
COPY src /src
COPY Cargo.toml /
RUN cargo build --target i686-unknown-linux-musl --release

FROM scratch
COPY --from=0 target/i686-unknown-linux-musl/release/rust-cv /
CMD ["/rust-cv"]
