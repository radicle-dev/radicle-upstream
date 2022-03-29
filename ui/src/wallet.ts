// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";
import * as ethers from "ethers";

import * as error from "ui/src/error";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as notification from "ui/src/notification";
import * as radToken from "./wallet/radToken";

import * as ethereum from "ui/src/ethereum";
import {
  Network,
  Environment,
  networkFromChainId,
} from "ui/src/ethereum/environment";
import { WalletConnectSigner } from "ui/src/ethereum/walletConnectSigner";
import * as ethereumDebug from "ui/src/ethereum/debug";
import { createWalletConnect, QrDisplay } from "ui/src/ethereum/walletConnect";
import { INFURA_API_KEY_MAINNET, INFURA_API_KEY_RINKEBY } from "ui/src/config";

export { radToken };

export enum Status {
  Connected = "CONNECTED",
  Connecting = "CONNECTING",
  NotConnected = "NOT_CONNECTED",
}

export type Provider = ethers.providers.Provider & ethers.providers.EnsProvider;

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
  connect(qrDisplay: QrDisplay): Promise<void>;
  disconnect(): Promise<void>;
  provider: Provider;
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

// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function updateAccountBalances(
  environment: Environment,
  address: string,
  provider: ethers.providers.Provider
): Promise<void> {
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
    notification.showException(
      new error.Error({
        message: "Failed to get account balances",
        source: err,
      })
    );
  }
}

function getProvider(environment: Environment): Provider {
  switch (environment) {
    case Environment.Local:
      return new ethers.providers.JsonRpcProvider("http://localhost:8545");
    case Environment.Rinkeby:
      // This account is registered on igor.zuk@protonmail.com.
      return ethers.providers.InfuraProvider.getWebSocketProvider(
        "rinkeby",
        INFURA_API_KEY_RINKEBY
      );
    case Environment.Mainnet:
      // This account is registered on rudolfs@osins.org.
      return ethers.providers.InfuraProvider.getWebSocketProvider(
        "mainnet",
        INFURA_API_KEY_MAINNET
      );
  }
}

const walletConnect = createWalletConnect();

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function build(environment: Environment, provider: Provider): Wallet {
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
  async function connect(qrDisplay: QrDisplay): Promise<void> {
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
      notification.showException(
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

function build_disabled_wallet(
  environment: Environment,
  provider: Provider
): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  return {
    environment,
    subscribe: stateStore.subscribe,
    connect() {
      return Promise.resolve();
    },
    disconnect() {
      return Promise.resolve();
    },
    provider,
    signer: new WalletConnectSigner(walletConnect, provider),
    getAddress: () => undefined,
    destroy() {},
  };
}

export const store: svelteStore.Readable<Wallet> = svelteStore.derived(
  ethereum.selectedEnvironment,
  (environment, set) => {
    const provider = getProvider(environment);
    ethereumDebug.install(provider);

    const wallet = build_disabled_wallet(environment, provider);
    set(wallet);
    return () => wallet.destroy();
  }
);

export function isConnected(): boolean {
  const walletStore = svelteStore.get(store);
  const wallet = svelteStore.get(walletStore);

  return wallet.status === Status.Connected;
}

export function walletAddress(): string | undefined {
  const walletStore = svelteStore.get(store);
  const wallet = svelteStore.get(walletStore);

  if (wallet.status === Status.Connected) {
    return wallet.connected.address;
  }
}

// Activate the store so that the wallet is never destroyed when all views
// unsubscribe.
store.subscribe(() => {});
