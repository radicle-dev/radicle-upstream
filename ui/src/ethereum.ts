import { writable as persistentStore } from "svelte-persistent-store/dist/local";

// The environment in which the ethereum-based experiences may run.
export enum Environment {
  // A local node for testing and development. The test wallet we use
  // for this environment may be in the 'Ropsten', 'Mainnet', or other
  // network. However, those would not be the real 'Ropsten' and 'Mainnet'
  // networks, respectively, but simply fake ones for testing purposes.
  Local = "Local",
  // The Ropsten testnet network
  Ropsten = "Ropsten",
  // N.B: We will support 'Mainnet' in the future
}

// The ethereum networks we may parse from a connected wallet across
// all`Environment`s. See `supportedNetwork` to learn which networks
// which Environment supports.
export enum Network {
  Ropsten = "Ropsten",
  Mainnet = "Mainnet",
  Other = "Other",
}

// Inform which `Network`s we support within the given environment.
export function supportedNetwork(environment: Environment): Network {
  switch (environment) {
    case Environment.Local:
      return Network.Mainnet;
    case Environment.Ropsten:
      return Network.Ropsten;
  }
}

// Parse a `Network` value given a `chainId`.
// For reference check https://chainid.network.
export function networkFromChainId(chainId: number): Network {
  switch (chainId) {
    case 1:
      return Network.Mainnet;
    case 3:
      return Network.Ropsten;
    default:
      return Network.Other;
  }
}

// The store where the selected Ethereum environment is persisted.
export const selectedEnvironment = persistentStore<Environment>(
  "ethereum-environment-v0",
  Environment.Ropsten
);
