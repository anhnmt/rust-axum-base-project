FROM rust:slim-buster as build
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.toml ./

# copy your source tree
COPY ./src ./src

# this build step will cache your dependencies
# build for release
RUN cargo build --release

# our final base
FROM debian:buster-slim

WORKDIR /app

# copy the build artifact from the build stage
COPY --from=build /app/target/release/rust-axum-base-project .

# set the startup command to run your binary
ENTRYPOINT ["./rust-axum-base-project"]