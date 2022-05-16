FROM rust:1.60 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

# create a new empty shell project
RUN USER=root cargo new --bin ez-q
WORKDIR /ez-q

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/ez_q*
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc-debian10

# copy the build artifact from the build stage
COPY --from=build /ez-q/target/release/ez-q /

# set the startup command to run your binary
ENTRYPOINT ["/ez-q"]
