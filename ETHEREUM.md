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
### Funding

Three moving pieces back the Radicle Funding experiences:

- An Ethereum wallet, owned by the user
- A connection between the Radicle Upstream (the app) and said Ethereum wallet
- The Radicle Contracts (Ethereum smart-contracts)

The funding experiences provided in the Radicle Upstream are Ethereum-based,
meaning that actions such as adding users to the list of receivers, collect
incoming support funds, and others, translate into ethereum transactions.

For users to approve those transactions originated in the app, they need to
establish a connection between an Ethereum wallet and the app. We provide this
capability through a WalletConnect integration. A substantial number of
Ethereum wallets support WalletConnect. You can find the complete list
[here][wcw].

Once a wallet is connected to the app, the funding experiences become available
to the user. The user can now set up and edit their support, receive funds,
etc.  All of these actions translate into transactions the user must review,
(i.e., approve or reject) on their connected wallet.

These same transactions are provided and ran by the [Radicle Contracts][rc],
our custom Ethereum smart-contracts.

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
  it. The "DAI" token used in this environment is our custom Radicle Token.
  See the [local environment][loc] section to learn how to get set up.

2. `Rinkeby`

  In this environment, the app will plug itself with the Radicle Contracts
  deployed in the Rinkeby network and Infura is used as a service provider. We
  recommend that you use a real wallet to ensure that we are testing using
  real-world conditions.

  You will need [Rinkeby Eth] to pay the incurring transactions fees. No real
  money will be used.

  We're using the official Rinkeby DAI contract:
    0x5592ec0cfb4dbc12d3ab100b257153436a1f0fea

  Faucet: https://app.compound.finance/ -> DAI -> Withdraw -> Faucet

3. `Mainnet`

  Production environment where everything costs real money. So far only the Org
  and Attestation features is available on `Mainnet`, the contracts for the
  funding (aka token streams) feature are not yet deployed.

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

![Radicle Funding Development Set up][dev-setup]

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
- Once the app is running, enable the funding feature in the Upstream settings



[wcw]:https://walletconnect.org/wallets/
[wctw]:https://github.com/radicle-dev/walletconnect-test-wallet
[rc]:https://github.com/radicle-dev/radicle-contracts
[dev-setup]:./funding-dev-setup.svg "Radicle Funding Development Set up"
[org]: https://radicle.community/t/feature-update-orgs/2132
[fauc]: https://faucet.rinkeby.io
[loc]: #local-environment
[gnosis]: https://help.gnosis-safe.io/en/articles/3876456-what-is-gnosis-safe
