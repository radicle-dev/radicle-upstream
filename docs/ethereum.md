# Ethereum development

# Test Wallet

The app can be run with a test wallet that automatically connects and signs
transaction without user interaction on the Rinkeby network. To use the test
wallet set the following environment variable.

```bash
RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC="foo bar" yarn run start
```

The value of the variable is the mnemonic phrase used to derive your private
key.

## Features based on Ethereum

### Orgs

Radicle orgs allow users to maintain an auditable and transparent history of
project state on Ethereum. This enables new workflows around your main branch
that is now anchored and secured on Ethereum. Anchors can be used to trigger
off-chain actions like the distribution of developer rewards, software
releases, or any job you’d like. Org contracts may be controlled by arbitrary
addresses. At the moment we deploy a [Gnosis safe contract][gnosis] as the
controller when creating an org.

Read more about Orgs [here][org].

## Ethereum environments

1. `Local`

  In this environment Radicle Contracts are deployed to a development Ethereum
  node running locally and we use a local test wallet that can communicate with
  it. See the [local environment][loc] section to learn how to get set up.

2. `Rinkeby`

  In this environment, the app will plug itself with the Radicle Contracts
  deployed in the Rinkeby network and Infura is used as a service provider. We
  recommend that you use a real wallet to ensure that we are testing using
  real-world conditions.

  You will need [Rinkeby Eth] to pay the incurring transactions fees. No real
  money will be used.

3. `Mainnet`

  Production environment where everything costs real money. So far the Org and
  Attestation features are available on `Mainnet`.

### Local environment

**⚠️ This section is outdated and will be rewritten ⚠️**

In the local environment, we set up these three pieces as follows:

- A local WalletConnect test wallet instance

  This instance will play the role of a real wallet:
    - It provides a couple of test accounts that we use in development.
    - The transactions triggered in the app will be prompted here for the user
      to approve or reject.

- A local ganache instance

  Ganache provides a local Ethereum RPC client for testing and development. We
  deploy the Radicle Contracts to this instance. Here, we also set an initial
  balance of the account we choose for development purposes.

  For peace of mind, note that this instance has no connection to other
  networks such as mainnet or testnet. Therefore, no real assets used ever.
  Feel free to play around!

**Getting started**

- Install [walletconnect-test-wallet][wctw]

  - `git clone git@github.com:radicle-dev/walletconnect-test-wallet.git`
  - `cd walletconnect-test-wallet`
  - `npm install`

- Set up the local test ethereum account

  - Run `npm run start` within `walletconnect-test-wallet`. It should open the
    test wallet in your browser at `localhost:3000`.

  - Copy the full Ethereum address provided at the top of the page.

- Now, in `radicle-upstream`, run:

  - `yarn install`
  - `mkdir sandbox`
  - `touch sandbox/.local-eth-account`
  - Finally, paste the copied address in the previous step into this file.

**Running**

With everything installed and set up, run the following commands in different
tabs:

- `npm run start` within `walletconnect-test-wallet`
- `./scripts/ethereum-dev-node.ts` in `radicle-upstream`
- `RADICLE_UPSTREAM_EXPERIMENTAL=true yarn start` within `radicle-upstream`



[wcw]:https://walletconnect.org/wallets/
[wctw]:https://github.com/radicle-dev/walletconnect-test-wallet
[rc]:https://github.com/radicle-dev/radicle-contracts
[org]: https://radicle.community/t/feature-update-orgs/2132
[fauc]: https://faucet.rinkeby.io
[loc]: #local-environment
[gnosis]: https://help.gnosis-safe.io/en/articles/3876456-what-is-gnosis-safe
