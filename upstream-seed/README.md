# Upstream Seed

A light-weight seed node that tracks and replicates the configured Radicle
projects.

## Features

* Radicle peer based on [`librad`][librad] that participates in gossip.
* Tracks projects configured via the `--project` option. Only data
  of the first remote that provides the project is replicated. Data of
  subsequently discovered copies is not replicated at the moment.
* Asks for updates to tracked projects and replicates updates if they are
  available.
* Replicates person identities associated with owners of the tracked projects.
* Announces all projects it tracks and all peers that it tracks for these
  projects whenever a new peer connects.

[librad]: https://github.com/radicle-dev/radicle-link/tree/master/librad
