FROM rust as build

RUN cargo install cargo-make
RUN cargo install wasm-pack
RUN cargo install cargo-generate
RUN cargo install geckodriver

RUN cargo new rust-playground --bin

WORKDIR /rust-playground

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release

COPY . /rust-playground/
RUN cargo make build-release

FROM rust:1.49
COPY --from=build /rust-playground/target/release/actix-api /rust-playground/rust-playground
CMD ["./rust-playground/rust-playground"]

