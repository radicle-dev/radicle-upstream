import WalletConnect from "@walletconnect/client";
import * as svelteStore from "svelte/store";

import type { Big } from "big.js";
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
  TransactionResponse,
} from "@ethersproject/abstract-provider";

import * as daiToken from "../src/funding/daiToken";
import * as error from "../src/error";
import * as ethereum from "../src/ethereum";
import * as modal from "../src/modal";

import ModalWalletQRCode from "../Modal/Wallet/QRCode.svelte";

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
  account: Account;
  network: ethereum.Network;
}

export interface Account {
  address: string;
  daiBalance: Big;
  ethBalance: Big;
}

export interface Wallet extends svelteStore.Readable<State> {
  environment: ethereum.Environment;
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  provider: ethers.providers.Provider;
  signer: ethers.Signer;
  account(): Account | undefined;
  destroy(): void;
}

function getProvider(
  environment: ethereum.Environment
): ethers.providers.Provider {
  switch (environment) {
    case ethereum.Environment.Local:
      return new ethers.providers.JsonRpcProvider("http://localhost:8545");
    case ethereum.Environment.Ropsten:
      return new ethers.providers.InfuraProvider(
        "ropsten",
        "66fa0f92a54e4d8c9483ffdc6840d77b"
      );
  }
}

function build(
  environment: ethereum.Environment,
  provider: ethers.providers.Provider
): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  function onModalHide(): void {
    const store = svelteStore.get(stateStore);
    if (store.status === Status.NotConnected) {
      reinitWalletConnect();
    }
  }

  function newWalletConnect(): WalletConnect {
    return new WalletConnect({
      bridge: "https://bridge.walletconnect.org",
      qrcodeModal: qrCodeModal,
    });
  }

  // We need to reinitialize `WalletConnect` until this issue is fixed:
  // https://github.com/WalletConnect/walletconnect-monorepo/pull/370
  function reinitWalletConnect() {
    walletConnect = newWalletConnect();
    signer.walletConnect = walletConnect;
  }

  const qrCodeModal = {
    open: (uri: string, _cb: unknown, _opts?: unknown) => {
      uriStore.set(uri);
      modal.toggle(ModalWalletQRCode, onModalHide);
    },
    close: () => {
      // N.B: this is actually called when the connection is established,
      // not when the modal is closed per se.
      stateStore.set({ status: Status.Connecting });
      modal.hide();
    },
  };

  let walletConnect = newWalletConnect();
  const signer = new WalletConnectSigner(
    walletConnect,
    provider,
    environment,
    disconnect
  );
  const daiTokenContract = daiToken.connect(
    signer,
    daiToken.daiTokenAddress(environment)
  );

  // Connect to a wallet using walletconnect
  async function connect() {
    if (svelteStore.get(stateStore).status !== Status.NotConnected) {
      throw new Error("A wallet is already connected");
    }

    try {
      await walletConnect.connect();
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
    loadAccountData();
  }

  // Load the data of the connected account.
  async function loadAccountData() {
    try {
      const accountAddress = await signer.getAddress();
      const daiBalance = await daiTokenContract
        .balanceOf(accountAddress)
        .then(ethereum.toBaseUnit);
      const ethBalance = await provider
        .getBalance(accountAddress)
        .then(ethereum.toBaseUnit);
      const chainId = walletConnect.chainId;

      const connected = {
        account: {
          address: accountAddress,
          daiBalance,
          ethBalance,
        },
        network: ethereum.networkFromChainId(chainId),
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
    if (svelteStore.get(stateStore).status === Status.Connected) {
      loadAccountData();
    }
  }, REFRESH_INTERVAL_MILLIS);

  function account(): Account | undefined {
    const state = svelteStore.get(stateStore);
    if (state.status === Status.Connected) {
      return state.connected.account;
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
    account,
    destroy() {
      clearInterval(refreshInterval);
    },
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
  private _environment: ethereum.Environment;

  constructor(
    walletConnect: WalletConnect,
    provider: Provider,
    environment: ethereum.Environment,
    onDisconnect: () => void
  ) {
    super();
    defineReadOnly(this, "provider", provider);
    this._provider = provider;
    this._environment = environment;
    this.walletConnect = walletConnect;
    this.walletConnect.on("disconnect", onDisconnect);
  }

  async getAddress(): Promise<string> {
    const accountAddress = this.walletConnect.accounts[0];
    if (!accountAddress) {
      throw new Error(
        "The connected wallet has no accounts or there is a connection problem"
      );
    }
    return accountAddress;
  }

  async signMessage(_message: ethers.Bytes | string): Promise<string> {
    throw new Error("not implemented");
  }

  async sendTransaction(
    transaction: Deferrable<TransactionRequest>
  ): Promise<TransactionResponse> {
    // When using a local Ethereum environment, we want our app to send
    // the transaction to the local Ethereum node and have the external
    // wallet just sign the transaction. In all other environments, we
    // want the external wallet to submit the transaction to the network.
    if (this._environment === ethereum.Environment.Local) {
      return super.sendTransaction(transaction);
    }

    const tx = await resolveProperties(transaction);
    const from = tx.from || (await this.getAddress());

    const txHash = await this.walletConnect.sendTransaction({
      from,
      to: tx.to,
      value: BigNumberToPrimitive(tx.value),
      data: bytesLikeToString(tx.data),
    });

    return {
      from,
      value: ethers.BigNumber.from(tx.value || 0),
      get chainId(): number {
        throw new Error("this should never be called");
      },
      get nonce(): number {
        throw new Error("this should never be called");
      },
      get gasLimit(): ethers.BigNumber {
        throw new Error("this should never be called");
      },
      get gasPrice(): ethers.BigNumber {
        throw new Error("this should never be called");
      },
      data: bytesLikeToString(tx.data) || "",
      hash: txHash,
      confirmations: 1,
      wait: () => {
        throw new Error("this should never be called");
      },
    };
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
      value: BigNumberToPrimitive(tx.value || 0),
      gasLimit: BigNumberToPrimitive(tx.gasLimit || 200 * 1000),
      gasPrice: BigNumberToPrimitive(tx.gasPrice || 0),
      nonce,
      data: bytesLikeToString(tx.data),
    });
    return signedTx;
  }

  connect(_provider: Provider): ethers.Signer {
    throw new Error("WalletConnectSigner.connect should never be called");
  }
}

function BigNumberToPrimitive(
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
  return balance.toLocaleString("us-US");
}

export const store: svelteStore.Readable<Wallet> = svelteStore.derived(
  ethereum.selectedEnvironment,
  (environment, set) => {
    const provider = getProvider(environment);
    if (provider instanceof ethers.providers.JsonRpcProvider) {
      window.ethereumDebug = new EthereumDebug(provider);
    }

    const wallet = build(environment, provider);
    set(wallet);
    return () => wallet.destroy();
  }
);

// Activate the store so that the wallet is never destroyed when all views
// unsubscribe.
//
// eslint-disable-next-line @typescript-eslint/no-empty-function
store.subscribe(() => {});
