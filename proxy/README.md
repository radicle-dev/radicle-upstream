# Proxy

A background service that implements all business logic tying together the
radicle code collaboration protocol. It also provides the `git-remote-rad` git
helper.

### Run

To start up the proxy binary you can run:
```sh
cargo run --bin radicle-proxy
```

To get a list of available CLI options:
```sh
cargo run --bin radicle-proxy -- --help
```

### Testing

Before running the test suite, download the test fixtures:

```sh
../scripts/test-setup.sh
```

Then run tests as usual:

```sh
cargo test
```
