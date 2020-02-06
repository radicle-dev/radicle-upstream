# Proxy

A background service that implements all business logic tying together the
radicle protocol and radicle registry. It provides a GraphQL API to the
frontend.


### Run

To start up the binary you can run: `cargo run -- local`. After that the API is
served on `http://127.0.0.1:8080/graphql`.

To use an in-memory Radicle client, run `cargo run -- memory`.

You can open the GraphQL explorer in the browser at `http://localhost:8080`.

To use the `radicle-registry` node as the backend for the proxy run `cargo run
-- registry`. See [`radicle-registry`][rr] for information on how to
build and run the registry node.


### Testing

Before running the test suite, download the test fixtures:

```sh
git submodule update --recursive
```

Then run tests as normal:

```sh
cargo test
```

[rr]: https://github.com/radicle-dev/radicle-registry#building-and-running-the-node
