import WalletConnect from "@walletconnect/client";
import * as svelteStore from "svelte/store";
import * as ethers from "ethers";
import * as ethersBytes from "@ethersproject/bytes";
import {
  Deferrable,
  defineReadOnly,
  resolveProperties,
} from "@ethersproject/properties";
import type {
  Provider,
  TransactionRequest,
} from "@ethersproject/abstract-provider";

import * as modal from "../src/modal";
import * as path from "../src/path";
import * as notification from "../src/notification";

export enum Status {
  Connected = "CONNECTED",
  Connecting = "CONNECTING",
  NotConnected = "NOT_CONNECTED",
}

export type State =
  | { status: Status.NotConnected; error?: Error }
  | { status: Status.Connecting }
  | { status: Status.Connected; connected: Connected };

export interface Connected {
  account: Account;
}

export interface Account {
  address: string;
  balance: string;
}

export interface Wallet extends svelteStore.Readable<State> {
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  signer: ethers.Signer;
}

export const provider = new ethers.providers.JsonRpcProvider(
  "http://localhost:8545"
);

export function build(): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  const qrCodeModal = {
    open: (uri: string, _cb: unknown, _opts?: unknown) => {
      uriStore.set(uri);
      modal.toggle(path.walletQRCode());
    },
    close: async () => {
      // N.B: this is actually called when the connection is established,
      // not when the modal is closed per se.
      modal.hide();
    },
  };

  let walletConnect = new WalletConnect({
    bridge: "https://bridge.walletconnect.org",
    qrcodeModal: qrCodeModal,
  });

  const signer = new WalletConnectSigner(walletConnect, provider);

  window.ethereumDebug = new EthereumDebug(provider);

  // Connect to a wallet using walletconnect
  async function connect() {
    if (svelteStore.get(stateStore).status !== Status.NotConnected) {
      throw new Error("Already connected");
    }

    try {
      await walletConnect.connect();
    } catch (error) {
      stateStore.set({ status: Status.NotConnected, error });
      notification.error(
        `Failed to connect wallet: ${error.toString().replace("Error: ", "")}`
      );
    }
    await initialize();
  }

  async function disconnect() {
    console.log("Disconnect");
    await walletConnect.killSession();
    // We need to reinitialize `WalletConnect` until this issue is fixed:
    // https://github.com/WalletConnect/walletconnect-monorepo/pull/370
    walletConnect = new WalletConnect({
      bridge: "https://bridge.walletconnect.org",
      qrcodeModal: qrCodeModal,
    });
    signer.walletConnect = walletConnect;
    stateStore.set({ status: Status.NotConnected });
  }

  async function initialize() {
    if (!walletConnect.connected) {
      return;
    }

    try {
      stateStore.set({ status: Status.Connecting });
      const accountAddress = await signer.getAddress();
      const balance = await signer.getBalance();
      const connected = {
        account: {
          address: accountAddress,
          balance: ethToDai(balance.toString()),
        },
      };
      stateStore.set({ status: Status.Connected, connected });
    } catch (error) {
      console.error(error);
      stateStore.set({ status: Status.NotConnected, error });
    }
  }

  initialize();

  return {
    subscribe: stateStore.subscribe,
    connect,
    disconnect,
    signer,
  };
}

declare global {
  interface Window {
    ethereumDebug: EthereumDebug;
  }
}

class WalletConnectSigner extends ethers.Signer {
  public walletConnect: WalletConnect;
  private _provider: ethers.providers.Provider;

  private sessionUpdateListener = () => {
    return undefined;
  };

  constructor(walletConnect: WalletConnect, provider: Provider) {
    super();
    defineReadOnly(this, "provider", provider);
    this._provider = provider;
    this.walletConnect = walletConnect;
    this.walletConnect.on("session_update", this.sessionUpdateListener);
  }

  async getAddress(): Promise<string> {
    const accountAddress = this.walletConnect.accounts[0];
    if (!accountAddress) {
      throw new Error("no account address");
    }
    return accountAddress;
  }

  async signMessage(_message: ethers.Bytes | string): Promise<string> {
    throw new Error("not implemented");
  }

  async signTransaction(
    transaction: Deferrable<TransactionRequest>
  ): Promise<string> {
    const tx = await resolveProperties(transaction);
    const from = tx.from || (await this.getAddress());
    const nonce = await this._provider.getTransactionCount(from);
    const signedTx = await this.walletConnect.signTransaction({
      from,
      to: tx.to,
      value: bigNumberishToPrimitive(tx.value || 0),
      gasLimit: bigNumberishToPrimitive(tx.gasLimit || 200 * 1000),
      gasPrice: bigNumberishToPrimitive(tx.gasPrice || 0),
      nonce,
      data: bytesLikeToString(tx.data),
    });
    return signedTx;
  }

  connect(provider: Provider): ethers.Signer {
    return new WalletConnectSigner(this.walletConnect, provider);
  }
}

function bigNumberishToPrimitive(
  bn: ethers.BigNumberish | undefined
): string | undefined {
  if (bn === undefined) {
    return undefined;
  } else {
    return ethers.BigNumber.from(bn).toString();
  }
}

function bytesLikeToString(
  bytes: ethersBytes.BytesLike | undefined
): string | undefined {
  if (bytes === undefined) {
    return undefined;
  } else {
    return ethersBytes.hexlify(bytes);
  }
}

class EthereumDebug {
  private provider: ethers.providers.JsonRpcProvider;

  constructor(provider: ethers.providers.JsonRpcProvider) {
    this.provider = provider;
  }
  async mineBlocks(blocks = 1) {
    while (blocks) {
      blocks -= 1;
      await this.provider.send("evm_mine", []);
    }
  }

  async setBlockTime(seconds = 5) {
    await this.provider.send("evm_setTime", [seconds]);
  }

  async increaseTime(seconds = 5) {
    await this.provider.send("evm_increaseTime", [seconds]);
  }
}

// URI store for the URI used to build the connecting QRCode.
export const uriStore = svelteStore.writable<string | undefined>(undefined);

export function formattedBalance(balance: number): string {
  return balance.toLocaleString("de-DE");
}

export function ethToDai(eth: string): string {
  return ethers.BigNumber.from(eth)
    .div(10 ^ 18)
    .toString();
}

// The wallet singleton
export const wallet = build();
