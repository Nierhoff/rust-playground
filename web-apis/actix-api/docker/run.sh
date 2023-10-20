cd ..
# export DOCKER_DEFAULT_PLATFORM=linux/arm64/v8
cargo build --release
# --target=aarch64-unknown-linux-gnu
rm docker/target/rust-playground-app
# cp ../../target/aarch64-unknown-linux-gnu/release/actix-api docker/target/rust-playground-app
cp ../../target/release/actix-api docker/target/rust-playground-app
cd docker
docker image rm -f nierhoff/rust-playground/actix-api:latest
docker buildx build --platform linux/arm64/v8 . -t nierhoff/rust-playground/actix-api:latest
docker run nierhoff/rust-playground/actix-api:latest
