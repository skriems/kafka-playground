Zeou.rs
=======

> A Rust workspace to handle all zeou related message processing in
> Apache Kafka _blazingly fast_ 🚀

This started as a journey to create kafka message consumers and producers in
Rust based on the [rust-rdkafka][] project which itself uses librdkafka which
is the C library for Kafka from Confluent.

The default runtime of rust-rdkafka is [tokio][] but in order to do some
comparisions (playing around and learning 😎) there are also implementations
using [async-std][].

The most mature worker is `asyncstd` which is badly named but refers to its
runtime which is, you gussed it, `async-std`.

## Structure

There currently two shared libraries in this project

```
lib     # shared code related to kafka and the different runtimes
zeo     # zeou's business logic
```

## Build locally

You can build all workspace packages in dev mode via

```bash
cargo build --workspace
```

and in release mode

```bash
cargo build --workspace --release
```

If you want to build a specific worker

```bash
cargo build --bin asyncstd (--release)
```

notice here that asyncstd is a _binary_, hence the `--bin` flag. If you want to
build the `lib` or `zeou` package (which are libraries), you need to use the
`--lib` flag.

```bash
cargo build --lib zeou (--release)
```

## Run locally

simmilar to the build commands, use the `run` command:

```bash
cargo run --bin asyncstd [ARGS]
```

## Docker

Check the `docker-compose.yml` file to get a grasp how this worker can be used.
You can provide all necessary CLI arguments via the `command` statement or even
better you environment variables.

Build and tag with docker-compose:

```bash
docker-compose build
````

```bash
docker-compose up
````

[rust-rdkafka][https://github.com/fede1024/rust-rdkafka]
[tokio][https://github.com/tokio-rs/tokio]
[async-std][https://github.com/async-rs/async-std]
