// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";
import * as ethers from "ethers";

import * as daiToken from "ui/src/funding/daiToken";
import * as error from "ui/src/error";
import * as mutexExecutor from "ui/src/mutexExecutor";

import * as ethereum from "ui/src/ethereum";
import {
  Network,
  Environment,
  networkFromChainId,
} from "ui/src/ethereum/environment";
import { WalletConnectSigner } from "ui/src/ethereum/walletConnectSigner";
import * as ethereumDebug from "ui/src/ethereum/debug";
import { createWalletConnect } from "ui/src/ethereum/walletConnect";

export enum Status {
  Connected = "CONNECTED",
  Connecting = "CONNECTING",
  NotConnected = "NOT_CONNECTED",
}

export type State =
  | { status: Status.NotConnected; error?: globalThis.Error }
  | { status: Status.Connecting }
  | { status: Status.Connected; connected: Connected };

export interface Connected {
  address: string;
  network: Network;
}

export interface Wallet extends svelteStore.Readable<State> {
  environment: Environment;
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  provider: ethers.providers.Provider;
  signer: ethers.Signer;
  // Returns the address of the wallet account if the wallet is
  // connected.
  getAddress(): string | undefined;
  destroy(): void;
}

export const accountBalancesStore = svelteStore.writable<{
  dai: ethers.BigNumber | null;
  eth: ethers.BigNumber | null;
}>({ dai: null, eth: null });

const accountBalanceFetch = mutexExecutor.create();

async function updateAccountBalances(
  environment: Environment,
  address: string,
  provider: ethers.providers.Provider
) {
  try {
    const daiTokenContract = daiToken.connect(
      provider,
      daiToken.daiTokenAddress(environment)
    );
    const result = await accountBalanceFetch.run(async () => {
      const dai = await daiTokenContract.balanceOf(address);
      const eth = await provider.getBalance(address);
      return { eth, dai };
    });
    if (result) {
      accountBalancesStore.set(result);
    }
  } catch (err) {
    error.show(err);
  }
}

function getProvider(environment: Environment): ethers.providers.Provider {
  switch (environment) {
    case Environment.Local:
      return new ethers.providers.JsonRpcProvider("http://localhost:8545");
    case Environment.Rinkeby:
      // This account is registered on igor.zuk@protonmail.com.
      return ethers.providers.InfuraProvider.getWebSocketProvider(
        "rinkeby",
        "de5e2a8780c04964950e73b696d1bfb1"
      );
    case Environment.Mainnet:
      // This account is registered on rudolfs@osins.org.
      return ethers.providers.InfuraProvider.getWebSocketProvider(
        "mainnet",
        "7a19a4bf0af84fcc86ffb693a257fad4"
      );
  }
}

const walletConnect = createWalletConnect();

function build(
  environment: Environment,
  provider: ethers.providers.Provider
): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  const signer = new WalletConnectSigner(walletConnect, provider, environment);

  const unsubscribeStateStore = stateStore.subscribe(state => {
    if (state.status === Status.Connected) {
      updateAccountBalances(environment, state.connected.address, provider);
    }
  });

  // Connect to a wallet using walletconnect
  async function connect() {
    if (svelteStore.get(stateStore).status !== Status.NotConnected) {
      throw new Error("A wallet is already connected");
    }

    try {
      stateStore.set({ status: Status.Connecting });
      const connected = await walletConnect.connect();
      // If we connect succesfully, `stateStore` is updated by the
      // `connection` subscription.
      if (!connected) {
        stateStore.set({ status: Status.NotConnected });
      }
    } catch (e) {
      stateStore.set({ status: Status.NotConnected, error: e });
      error.show(
        new error.Error({
          code: error.Code.WalletConnectionFailure,
          message: `Failed to connect wallet: ${e
            .toString()
            .replace("Error: ", "")}`,
          source: error.fromJsError(e),
        })
      );
      return;
    }
  }

  const unsubConnection = walletConnect.connection.subscribe(connection => {
    if (connection) {
      stateStore.set({
        status: Status.Connected,
        connected: {
          address: connection.accountAddress,
          network: networkFromChainId(connection.chainId),
        },
      });
    } else {
      stateStore.set({ status: Status.NotConnected });
    }
  });

  // Periodically refresh the wallet data
  const REFRESH_INTERVAL_MILLIS = 60000;
  const refreshInterval = setInterval(() => {
    const state = svelteStore.get(stateStore);
    if (state.status === Status.Connected) {
      updateAccountBalances(environment, state.connected.address, provider);
    }
  }, REFRESH_INTERVAL_MILLIS);

  function getAddress(): string | undefined {
    const state = svelteStore.get(stateStore);
    if (state.status === Status.Connected) {
      return state.connected.address;
    }

    return undefined;
  }

  return {
    environment,
    subscribe: stateStore.subscribe,
    connect,
    disconnect() {
      return walletConnect.disconnect();
    },
    provider,
    signer,
    getAddress,
    destroy() {
      unsubConnection();
      unsubscribeStateStore();
      clearInterval(refreshInterval);
    },
  };
}

export const store: svelteStore.Readable<Wallet> = svelteStore.derived(
  ethereum.selectedEnvironment,
  (environment, set) => {
    const provider = getProvider(environment);
    ethereumDebug.install(provider);

    const wallet = build(environment, provider);
    set(wallet);
    return () => wallet.destroy();
  }
);

// Activate the store so that the wallet is never destroyed when all views
// unsubscribe.
store.subscribe(() => {});
