## Radicle Funding

Welcome to the developer's documentation on the Radicle Funding.

### Overview

Three moving pieces back the Radicle Funding experiences:

- An Ethereum wallet, owned by the user
- A connection between the Radicle Upstream (the app) and said Ethereum wallet
- The Radicle Contracts (Ethereum smart-contracts)

The funding experiences provided in the Radicle Upstream are Ethereum-based,
meaning that actions such as adding users to the list of receivers, collect
incoming support funds, and others, translate into ethereum transactions.

For users to approve those transactions originated in the app, they need to
establish a connection between an Ethereum wallet and the app. We provide this
capability through a WalletConnect integration. A substantial number of Ethereum
wallets support WalletConnect. [You can find the complete list here][wcw].

Once a wallet is connected to the app, the funding experiences become available
to the user. The user can now set up and edit their support, receive funds, etc.
All of these actions translate into transactions the user must review, (i.e.,
approve or reject) on their connected wallet.

These same transactions are provided and ran by the [Radicle Contracts][rc], our
custom Ethereum smart-contracts.

### Ethereum environments

The funding experiences can take place in three different environments:

1. `Local`

  In this environment, the Radicle Contracts are deployed to development Ethereum
  node running locally and we use a local test wallet that can communicate with it.
  The "DAI" token used in this environment is our custom Radicle Token. See the
  [local environment](#local-environment) section to learn how to get set up.


2. `Ropsten`

  In this environment, the app will plug itself with the Radicle Contracts deployed
  in the Ropsten network and Infura is used as a service provider. We recommend that
  you use a real wallet to ensure that we are testing using real-world conditions.

  You will need fake Ropsten Eth (aka `rEth`) to pay the incurring transactions fees.
  No real money will be charged.

  As in the `Local` environment, here the "DAI" token used is also our custom Radicle
  Token, deployed in the Ropsten network. Reach out to the team to get some Radicle
  Tokens into your account.

3. `Mainnet`

  In this environment, to be introduced in the future once we are ready to go mainnet,
  the wallet will plug itself to the Radicle Contracts (to be) deployed in the Ethereum
  Mainnet. This environment is not currently supported.


#### Local environment

In the local environment, we set up these three pieces as follows:

- A local WalletConnect test wallet instance

  This instance will play the role of a real wallet:
    - It provides a couple of test accounts that we use in development.
    - The transactions triggered in the app will be prompted here for the user
      to approve or reject.

- A local ganache instance

  Ganache provides a local Ethereum RPC client for testing and development. We deploy the
  Radicle Contracts to this instance. Here, we also set an initial
  balance of the account we choose for development purposes.

  For peace of mind, note that this instance has no connection to other networks
  such as mainnet or testnet. Therefore, no real assets used ever. Feel free
  to play around!

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
- `yarn ethereum:start` within `radicle-upstream`
- `RADICLE_UPSTREAM_EXPERIMENTAL=true yarn start` within `radicle-upstream`
- Once the app is running, enable the funding feature in the Upstream settings


[wcw]:https://walletconnect.org/wallets/
[wctw]:https://github.com/radicle-dev/walletconnect-test-wallet
[rc]:https://github.com/radicle-dev/radicle-contracts
[dev-setup]:./funding-dev-setup.svg "Radicle Funding Development Set up"
