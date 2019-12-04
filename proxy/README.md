# AppProxy

Intermediate serving a specialised API to the radicle-upstream frontend via
GraphQL.

### Run

To start up the binary you can run: `cargo run -- local`. After that the API is
served on `http://127.0.0.1:8080/graphql`.

You can open the GraphQL explorer in the browser at `http://localhost:8080`.

To use the `radicle-registry` node as the backend for the proxy run `cargo run
-- registry`. See [`radicle-registry`][run-registry] for information on how to
build and run the registry node.

[run-registry]: https://github.com/radicle-dev/radicle-registry#building-and-running-the-node
