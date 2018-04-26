# Manifesto

A tiny service for generating media asset manfiests.

## Usage

Please [install the Rust toolchain](https://rustup.rs) for your platform.

Once installed, Manifesto can be compiled from the project root:

```sh
$ cargo build --release
```

This will build the binaries and save them in `target/release/...`.

To execute the directory indexer, use the following invocation:

```sh
./target/release/manfiesto-index path/to/media/ output.json
```

To execute the directory hasher, use the following invocation:

```sh
./target/release/manfiesto-hash output.json hashed_files/
```
