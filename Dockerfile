# rust uses debian buster as base image
FROM rust:1.64 as build

# install dependencies for building rdkafka
RUN apt-get update && apt-get install -y \
    make \
    cmake \
    # pthreads
    libc6-dev \
    # zlib for rdkafka's libz feature
    zlib1g-dev \
    # for rdkafka's ssl feature
    libssl-dev \
    # for rdkafka's gssapi feature
    libsasl2-dev \
    # for rdkafka's zstd-pkg-config feature
    libzstd-dev \
    && rm -rf /var/lib/apt/lists/*

# copy over the workspace
COPY . /app
WORKDIR /app

# fetch dependencies to cache them
RUN cargo fetch

# build the workspace
RUN cargo build --workspace --release

RUN rm /app/target/release/deps/asyncstd*
# RUN rm /app/target/release/deps/tokio-consumer*
# RUN rm /app/target/release/deps/tokio-stream*

RUN cargo build --workspace --release

FROM debian:bullseye-slim

COPY --from=build /app/target/release/asyncstd .
# COPY --from=build /app/target/release/tokio-stream .
# COPY --from=build /app/target/release/tokio-consumer .

CMD ["./asyncstd", "process"]
