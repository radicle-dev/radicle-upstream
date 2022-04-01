# Upstream Seed Node infrastructure

Provides a GCP instance `seed-node-2` running `radicle-http-api` and
`radicle-git-server` in the cloud. The Upstream team uses the seed for testing
and development purposes.

It has been assigned the following DNS name: seed.upstream.radicle.xyz
  - `radicle-http-api` is running on port tcp:8777
  - `radicle-git-server` is running on port tcp:443


## Logging

You can find logs for the seed [here](https://cloudlogging.app.goo.gl/AEcmLeCyix5iY4AY8).


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


## Updating the services

You can update the binaries to the latest build by running:

```bash
sudo /home/ubuntu/radicle-upstream/infa/seed-node/update-radicle-http-api.sh
sudo /home/ubuntu/radicle-upstream/infa/seed-node/update-radicle-git-server.sh
```
