# Infrastructure for the Upstream team

The Upstream team owns the following infrastructure.

## GCP project `radicle-upstream`

* Owners are @rudolfs, @juliendonck and @geigerzaehler
* @geigerzaehler is paying for it
* Provides the `radicle-upstream-releases` storage bucket.
  * We serve the content of the bucket through GCP load balancers.
  * The DNS records (IPv4) pointing `releases.radicle.xyz` to the load balancer
    are managed by the Radicle Foundation.
  * Certificates for the loadbalancers are automatically provisioned by GCP.
* Provides infra for Github Actions Artifacts (see below)

## Github Actions Artifacts

Infrastructure for storing public build artifacts at predictable locations

* GCP service account `github-actions-radicle-upstream`
  * Credentials are exposed as build secret `GCP_SECRET_KEY` in Github Actions
* GCP storage bucket `radicle-upstream-build-artifacts` in region
  `europe-west1`.
    * `github-actions-radicle-upstream` has `storage.objectAdmin` role so it can
      upload and overwrite files to the bucket.
    * `allUsers` have `storage.objectViewer` role. This means public read access
      to the bucket.

The storage bucket uses the following schema to store artifacts.
* `v1/by-commit/<commit-sha>/...` for artifacts build from a certain commit.
* `v1/main/...` for artifacts build from the `main` branch.

## seed-node instance

The seed node instance runs the Radicle Git server and the Radicle source
browsing service.

The seed node uses the following resources
* GCE VM `seed-node-2`
  * Zone `europe-north1-a`
  * 100GB disk
  * Ubuntu LTS 21.04
  * n1-standard-1 (1vCPU, 3.75GB RAM)
  * Network tag `seed-node`
* External IP Address `seed-node-2`
  * bound to the VM instance `seed-node-2`
  * reachable under `34.88.37.244` or `seed.upstream.radicle.xyz`
* Firewall rule `seed-node` to allow traffic to the VM.

For more details see the [readme](../infra/seed-node/README.md).
