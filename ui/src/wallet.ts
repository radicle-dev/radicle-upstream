// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as ethers from "ethers";
import * as svelteStore from "svelte/store";

import * as radToken from "./wallet/radToken";
import * as error from "ui/src/error";
import * as mutexExecutor from "ui/src/mutexExecutor";

import * as ethereum from "ui/src/ethereum";
import {
  Network,
  Environment,
  networkFromChainId,
} from "ui/src/ethereum/environment";
import { WalletConnectSigner } from "ui/src/ethereum/walletConnectSigner";
import { createWalletConnect, QrDisplay } from "ui/src/ethereum/walletConnect";

export { radToken };

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
  connect(qrDisplay: QrDisplay): Promise<void>;
  disconnect(): Promise<void>;
  signer: WalletConnectSigner;
  // Returns the address of the wallet account if the wallet is
  // connected.
  getAddress(): string | undefined;
  destroy(): void;
}

export const accountBalancesStore = svelteStore.writable<{
  eth: ethers.BigNumber | null;
  rad: ethers.BigNumber | null;
}>({ eth: null, rad: null });

const accountBalanceFetch = mutexExecutor.create();

async function updateAccountBalances(
  environment: Environment,
  address: string,
  provider: ethers.providers.Provider
) {
  try {
    const radTokenContract = radToken.connect(provider, environment);

    const result = await accountBalanceFetch.run(async () => {
      const eth = await provider.getBalance(address);
      const rad = await radTokenContract.balanceOf(address);
      return { eth, rad };
    });

    if (result) {
      accountBalancesStore.set(result);
    }
  } catch (err: unknown) {
    error.show(
      new error.Error({
        message: "Failed to get account balances",
        source: err,
      })
    );
  }
}

const walletConnect = createWalletConnect();

function build(environment: Environment, provider: ethereum.Provider): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  const signer = new WalletConnectSigner(walletConnect, provider);

  const unsubscribeStateStore = stateStore.subscribe(state => {
    if (state.status === Status.Connected) {
      updateAccountBalances(environment, state.connected.address, provider);
    }
  });

  // Connect to a wallet using walletconnect
  async function connect(qrDisplay: QrDisplay) {
    if (svelteStore.get(stateStore).status !== Status.NotConnected) {
      throw new Error("A wallet is already connected");
    }

    try {
      stateStore.set({ status: Status.Connecting });
      const connected = await walletConnect.connect(qrDisplay);
      // If we connect succesfully, `stateStore` is updated by the
      // `connection` subscription.
      if (!connected) {
        stateStore.set({ status: Status.NotConnected });
      }
    } catch (unknownErr: unknown) {
      const err = error.fromUnknown(unknownErr);
      stateStore.set({ status: Status.NotConnected, error: err });
      error.show(
        new error.Error({
          code: error.Code.WalletConnectionFailure,
          message: `Failed to connect wallet: ${err.message}`,
          source: err,
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
    subscribe: stateStore.subscribe,
    connect,
    disconnect() {
      return walletConnect.disconnect();
    },
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
  (_, set) => {
    const environment = ethereum.getEnvironment();
    const provider = ethereum.getProvider();
    const wallet = build(environment, provider);
    set(wallet);
    return () => wallet.destroy();
  }
);

export function isConnected(): boolean {
  const walletStore = svelteStore.get(store);
  const wallet = svelteStore.get(walletStore);

  return wallet.status === Status.Connected;
}

// Activate the store so that the wallet is never destroyed when all views
// unsubscribe.
store.subscribe(() => {});
