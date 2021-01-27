import * as svelteStore from "svelte/store";
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

// The union of supported networks across all `Environment`s.
// We may only support a subset of them within specific environments.
export enum Network {
  Ropsten = "Ropsten",
  Mainnet = "Mainnet",
  Other = "Other",
}

// Check whether we support the `selectedNetwork` in the `environment` in which we are running.
export function supportedNetwork(
  environment: Environment,
  selectedNetwork: Network
): boolean {
  switch (environment) {
    case Environment.Local:
      return selectedNetwork === Network.Mainnet;
    case Environment.Ropsten:
      return selectedNetwork === Network.Ropsten;
  }
}

// The store where the selected Ethereum environment is persisted.
export const selected = persistentStore<Environment>(
  "ethereum-environment-v0",
  Environment.Local
);

export function current(): Environment {
  return svelteStore.get(selected);
}
