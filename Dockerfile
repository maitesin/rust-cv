FROM scratch
COPY target/i686-unknown-linux-musl/release/rust-cv /
CMD ["/rust-cv"]
