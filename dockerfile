FROM rust as build
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl

# Create a new empty shell project
RUN USER=root cargo new --bin rcd
WORKDIR /rcd

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/rcd*
RUN cargo build --release

# The final base image
FROM debian:stable-slim
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
    openssl libglib2.0-dev libgtk2.0-dev

# Copy from the previous build
COPY --from=build /rcd/target/release/rcd /usr/src/rcd

EXPOSE 50051/tcp
EXPOSE 50052/tcp

# Run the binary
CMD ["/usr/src/rcd"]

# To build, run this
# docker build -t rcd .