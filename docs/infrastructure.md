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
* Provides a container registry under `gcr.io/radicle-upstream`.
