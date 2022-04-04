# Upstream Seed

A light-weight seed node that tracks and replicates the configured Radicle
projects.

## Getting Started

Download binaries for the latest `main` branch builds.

* <https://storage.googleapis.com/radicle-upstream-build-artifacts/v1/main/x86_64-linux/upstream-seed>
* <https://storage.googleapis.com/radicle-upstream-build-artifacts/v1/main/x86_64-darwin/upstream-seed>

The seed is run with the Project URNs it is supposed to replicate as a
parameter.

```bash
upstream-seed --project rad:git:hkfoo
```

The seed will output its Peer ID. Together with the hosts IP address it gives
you the P2P address `hkpeerid@1.2.3.4:8776`. In Upstream on the “Network” page
add this address as a seed.

If your Upstream instance provides the project `hkfoo` it will now be replicated
by the seed. Anyone that connects to your seed will be able to replicate the
project from the seed.

## Features

* Radicle peer based on [`librad`][librad] that participates in gossip.
* Tracks and replicates projects configured via the `--project` option from all
  peers that provide a copy of the project.
* Asks for updates to tracked projects and replicates updates if they are
  available.
* Replicates person identities associated with owners of the tracked projects.
* Announces all projects it tracks and all peers that it tracks for these
  projects whenever a new peer connects.

[librad]: https://github.com/radicle-dev/radicle-link/tree/master/librad
