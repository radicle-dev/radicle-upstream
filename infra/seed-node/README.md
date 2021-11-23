# Upstream Seed Node infrastructure

Provides a GCP instance `seed-node-2` running `upstream-seed` in the cloud. The
Upstream team uses the seed to collaborate.

The peer address is `hydyq6xmgp3amt44z41n6cbods1osx73j5z6fky5xx4yx33afycyfc@34.88.37.244:8776`.

## Configuration

The list of projects the seed node tracks is set in `/etc/upstream-seed.env`.

## Logging

You can find logs for the seed [here](https://cloudlogging.app.goo.gl/AEcmLeCyix5iY4AY8).

## Update the binary

You can update the `upstream-seed` binary to the latest build of the `main`
branch by running

```bash
/home/ubuntu/radicle-upstream/infa/seed-node/update-upstream-seed.sh
```

The script accepts a commit hash as an optional argument. If provided, it
downloads the binary build for the given commit hash.

## Setup

To setup the instance, first log into the instance.

```bash
gcloud compute ssh ubuntu@seed-node-2 --zone=europe-north1-a
```

Then run the following commands

```bash
git clone https://github.com/radicle-dev/radicle-upstream
cd radicle-upstream
sudo infra/seed-node/setup.sh
```

The setup script can be re-run at a later point to upgrade the deployment.
