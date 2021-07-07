// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import WalletConnect from "@walletconnect/client";
import * as svelteStore from "svelte/store";
import * as ethers from "ethers";

import * as daiToken from "ui/src/funding/daiToken";
import * as error from "ui/src/error";
import * as modal from "ui/src/modal";
import * as mutexExecutor from "ui/src/mutexExecutor";

import ModalWalletQRCode from "ui/Modal/Wallet/QRCode.svelte";

import * as ethereum from "ui/src/ethereum";
import {
  Network,
  Environment,
  networkFromChainId,
} from "ui/src/ethereum/environment";
import { WalletConnectSigner } from "ui/src/ethereum/walletConnectSigner";
import * as ethereumDebug from "ui/src/ethereum/debug";

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
      return new ethers.providers.InfuraProvider(
        "rinkeby",
        "de5e2a8780c04964950e73b696d1bfb1"
      );
    case Environment.Mainnet:
      // This account is registered on rudolfs@monadic.xyz.
      return new ethers.providers.InfuraProvider(
        "mainnet",
        "7a19a4bf0af84fcc86ffb693a257fad4"
      );
  }
}

function build(
  environment: Environment,
  provider: ethers.providers.Provider
): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  // We need to reinitialize `WalletConnect` until this issue is fixed:
  // https://github.com/WalletConnect/walletconnect-monorepo/pull/370
  function reinitWalletConnect() {
    walletConnect = newWalletConnect();
    signer.walletConnect = walletConnect;
  }

  let walletConnect = newWalletConnect();
  const signer = new WalletConnectSigner(
    walletConnect,
    provider,
    environment,
    disconnect
  );

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
      await walletConnect.connect();
    } catch (e) {
      if (e.message.includes("User close")) {
        reinitWalletConnect();
        return;
      }
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
    await initialize();
  }

  async function disconnect() {
    await walletConnect.killSession().catch(() => {
      // When the user disconnects wallet-side, calling `killSession`
      // app-side trows an error because the wallet has already closed
      // its socket. Therefore, we simply ignore it.
    });

    stateStore.set({ status: Status.NotConnected });
    reinitWalletConnect();
  }

  async function initialize() {
    stateStore.set({ status: Status.Connecting });
    setAccountData();
  }

  async function setAccountData() {
    try {
      const accountAddress = await signer.getAddress();
      const chainId = walletConnect.chainId;

      const connected = {
        address: accountAddress,
        network: networkFromChainId(chainId),
      };
      stateStore.set({ status: Status.Connected, connected });
    } catch (error) {
      stateStore.set({ status: Status.NotConnected, error });
    }
  }

  if (walletConnect.connected) {
    initialize();
  }

  // Periodically refresh the wallet data
  const REFRESH_INTERVAL_MILLIS = 3000;
  const refreshInterval = setInterval(() => {
    const state = svelteStore.get(stateStore);
    if (state.status === Status.Connected) {
      setAccountData();
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
    disconnect,
    provider,
    signer,
    getAddress,
    destroy() {
      unsubscribeStateStore();
      clearInterval(refreshInterval);
    },
  };
}

function newWalletConnect(): WalletConnect {
  // This is set to true if the WalletConnect code closes the modal
  // instead of the user. This prevents calling back into WalletConnect
  // with `onClose` and aborting the connection.
  let modalClosedByWalletConnect = false;

  return new WalletConnect({
    bridge: "https://radicle.bridge.walletconnect.org",
    qrcodeModal: {
      open: (uri: string, onClose, _opts?: unknown) => {
        modal.toggle(
          ModalWalletQRCode,
          () => {
            if (!modalClosedByWalletConnect) {
              onClose();
            }
          },
          {
            uri,
          }
        );
      },
      close: () => {
        modalClosedByWalletConnect = true;
        modal.hide();
      },
    },
  });
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
