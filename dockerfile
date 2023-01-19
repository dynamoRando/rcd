FROM rust as build
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl

# Create a new empty shell project
RUN USER=root cargo new --bin rcd
WORKDIR /rcd

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rcd-client ./rcd-client
COPY ./rcdx ./rcdx
COPY ./rcdproto ./rcdproto
COPY ./rcdt ./rcdt
COPY ./rcd-admin ./rcd-admin
COPY ./rcd-common ./rcd-common
COPY ./rcd-enum ./rcd-enum
COPY ./rcd-core ./rcd-core
COPY ./rcd-http-common ./rcd-http-common
COPY ./rcd-query ./rcd-query
COPY ./rcd-http-common ./rcd-http-common
COPY ./rcd-messages ./rcd-messages
COPY ./rcd-sqlite ./rcd-sqlite
COPY ./rcd-http ./rcd-http
COPY ./rcd-grpc ./rcd-grpc
COPY ./rcd-sqlite ./rcd-sqlite
COPY ./rcdcli ./rcdcli
COPY ./rcd-foo ./rcd-foo

# Build for release.
# RUN rm ./target/release/*
RUN cargo build --release

# The final base image
FROM debian:stable-slim
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
    openssl libglib2.0-dev libgtk2.0-dev

# Copy from the previous build
COPY --from=build /rcd/target/release/rcdx /usr/src/rcdx
COPY --from=build /rcd/target/release/rcdt /usr/src/rcdt
COPY --from=build /rcd/target/release/rcdcli /usr/src/rcdcli

EXPOSE 50051/tcp
EXPOSE 50052/tcp
EXPOSE 50055/tcp

# Run the binary
CMD ["/usr/src/rcdx"]

# To build, run this
# docker build -t rcd .