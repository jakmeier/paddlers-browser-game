# An image with a (recent) Rust nightly and cargo-web installed
FROM buildpack-deps:stretch as RustBaseImg
ENV RUSTUP_HOME=/usr/local/rustup \
CARGO_HOME=/usr/local/cargo \
PATH=/usr/local/cargo/bin:$PATH
RUN set -eux; \
url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
wget "$url"; \
chmod +x rustup-init; \
./rustup-init -y --no-modify-path --default-toolchain nightly-2023-10-07; \
rm rustup-init; \
chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
rustup override set nightly-2023-10-07; \
rustup --version; \
cargo --version; \
rustc --version;