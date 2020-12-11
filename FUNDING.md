

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


#### Development environment

In the development environment, we set up these three pieces as follows:

- A local WalletConnect test wallet instance

  This instance will play the role of a real wallet:
    - It provides a couple of test accounts that we use in development.
    - The transactions triggered in the app will be prompted here for the user
      to approve or reject.

- A local ganachi instance

  Ganachi provides an Ethereum RPC client for testing and development. We have
  the Radicle Contracts deployed to and running on this instance. Here, we also
  set an initial balance of the account we choose for development and debugging
  purposes.

![Radicle Funding Development Set up][dev-setup]

**Getting started**

- Install `node.js` v12

  Recommendation: Use `nvm` to install and use a specific `node` version.

- Run `yarn install` within `radicle-upstream`

- Install [walletconnect-test-wallet][wctw]

  - `git clone git@github.com:radicle-dev/walletconnect-test-wallet.git`
  - `cd walletconnect-test-wallet`
  - `npm install`

- Set up the local test ethereum account

  - Run `npm run start` within `walletconnect-test-wallet`.
    It should open the test wallet in your browser at `localhost:3000`.
  - Copy the full address provided at the top of the page.
  - Now, in `radicle-upstream`, run:
    - `mkdir sandbox`
    - `touch sandbox/.local-eth-account`
  - Finally, paste the copied address in that file

- Install `ganache-cli`:
  `npm install -g ganache-cli`

**Running**

With everything installed and set up, run the following commands in different
tabs:

- `npm run start` within `walletconnect-test-wallet`
- `yarn start:ethereum` within `radicle-upstream`
- `yarn start` within `radicle-upstream`


[wcw]: https://walletconnect.org/apps/
[wctw]: https://github.com/radicle-dev/walletconnect-test-wallet
[rc]: https://github.com/radicle-dev/radicle-contracts

[dev-setup]: ./funding-dev-setup.svg "Radicle Funding Development Set up"
