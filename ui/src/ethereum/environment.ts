// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// The Ethereum environments we support and may connect to.
export enum Environment {
  // A local node for testing and development. The test wallet we use
  // for this environment may be in the 'Rinkeby', 'Mainnet', or other
  // network. However, those would not be the real 'Rinkeby' and 'Mainnet'
  // networks, respectively, but simply fake ones for testing purposes.
  Local = "Local",
  // The Rinkeby testnet for testing Orgs and Gnosis Safe functionality
  Rinkeby = "Rinkeby",
  // Production deployment
  Mainnet = "Mainnet",
}

// The ethereum networks we may parse from a connected wallet across
// all `Environment`s. See `supportedNetwork` to learn which networks
// each Environment supports.
export enum Network {
  Rinkeby = "Rinkeby",
  Mainnet = "Mainnet",
  Other = "Other",
}

// Inform which `Network`s we support within the given environment.
export function supportedNetwork(environment: Environment): Network {
  switch (environment) {
    case Environment.Local:
      return Network.Mainnet;
    case Environment.Rinkeby:
      return Network.Rinkeby;
    case Environment.Mainnet:
      return Network.Mainnet;
  }
}

// Parse a `Network` value given a `chainId`.
// For reference check https://chainid.network.
export function networkFromChainId(chainId: number): Network {
  switch (chainId) {
    case 1:
      return Network.Mainnet;
    case 4:
      return Network.Rinkeby;
    default:
      return Network.Other;
  }
}
