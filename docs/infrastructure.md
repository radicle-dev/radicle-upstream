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
* Provides Org Seed Node service (see below)
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
* `v1/master/...` for artifacts build from the `master` branch.

## Upstream Org Seed Node

We're running our own [Org Seed Node][os] for Upstream.

You can connect to the seed node using the handle
`hyncrnppok8iam6y5oemg4fkumj86mc4wsdiirp83z7tdxchk5dbn6@seed.upstream.radicle.xyz:8776`.

The org node uses the following resources
* GCE VM `org-node`
  * 4vCPI, 8GB RAM, 50GB SSD disk
* External IP Address `org-node`
  * bound to the VM instance `org-node`
  * reachable under `seed.upstream.radicle.xyz`
* Firewall rule `org-node` to allow traffic to the `org-node` VM.

### Updating the deployment

To update the deployment to a specific git reference log into the VM and run the
following commands
```bash
sudo su orgnode
cd ~/radicle-client-services
git fetch
git checkout <branch-to-deploy>
docker-compose build
docker-compose --env-file upstream-production-config.env -f docker-compose.yml up --detach
```

### Setup

1. Create an external IP address
```bash
gcloud compute addresses create "org-node" --region europe-north1
```

2. Create a GCP instance:

```bash
gcloud compute instances create org-node \
--project=radicle-upstream \
--zone=europe-north1-a \
--machine-type=e2-custom-4-8192 \
--address="org-node" \
--maintenance-policy=MIGRATE \
--service-account=995532143689-compute@developer.gserviceaccount.com \
--scopes=https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/trace.append \
--tags=org-node,http-server,https-server \
--create-disk=auto-delete=yes,boot=yes,device-name=org-node,image=projects/debian-cloud/global/images/debian-10-buster-v20210916,mode=rw,size=50,type=projects/radicle-upstream/zones/europe-north1-a/diskTypes/pd-ssd \
--no-shielded-secure-boot \
--shielded-vtpm \
--shielded-integrity-monitoring \
--reservation-affinity=any
```

3. Open ports `tcp:80` and `tcp:443` for serving git repositories of projects
   over HTTP and HTTPS, port `tcp:8777` for [serving the source code browsing
   API][sc] and `udp:8776` for the Radicle P2P protocol:

```bash
gcloud compute --project=radicle-upstream firewall-rules create org-node \
--direction=INGRESS \
--priority=1000 \
--network=default --action=ALLOW --rules=tcp:80,tcp:443,tcp:8777,udp:8776 \
--source-ranges=0.0.0.0/0 \
--target-tags=org-node
```

4. Connect to the instance:

```bash
gcloud beta compute ssh --zone "europe-north1-a" "org-node" \
--project "radicle-upstream"
```

5. [Install docker][do]:

```bash
sudo su

apt-get update
apt-get upgrade
apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

echo \
  "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

apt-get update

apt-get install docker-ce docker-ce-cli containerd.io
```

6. Set up radicle-client-services:

```bash
sudo su
apt-get install python3-pip

adduser --disabled-password --disabled-login orgnode
usermod -aG docker orgnode

su orgnode

cd /home/orgnode
pip3 install docker-compose

echo "export PATH=$PATH:/home/orgnode/.local/bin" >>.bashrc
source .bashrc

git clone https://github.com/radicle-dev/radicle-client-services.git

cd radicle-client-services
docker-compose pull

tee upstream-production-config.env << END
RADICLE_ORGS=0xe22450214b02C2416481aC2d3Be51536f7bb1fFf
RADICLE_DOMAIN=seed.upstream.radicle.xyz
ETH_RPC_URL=wss://mainnet.infura.io/ws/v3/7a19a4bf0af84fcc86ffb693a257fad4
RADICLE_SEED_USER=$(id -u orgnode)
END

docker-compose --env-file upstream-production-config.env -f docker-compose.yml up --detach
```



[do]: https://docs.docker.com/engine/install/debian
[os]: https://github.com/radicle-dev/radicle-client-services#setting-up-an-org-seed-node
[sc]: https://app.radicle.network/orgs/upstream.radicle.eth/projects/rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o/3b6b4b3d198c0070c8ba57846ec5e154826207d4
