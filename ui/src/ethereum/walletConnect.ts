// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// This module provides a facade for WalletConnect that makes it easier to use.

import { Mutex } from "async-mutex";
import Connector from "@walletconnect/client";
import type { ITxData, ISessionStatus } from "@walletconnect/types";
import * as svelteStore from "svelte/store";
import { isEqual } from "lodash";
import * as ethers from "ethers";

import * as Error from "ui/src/error";
import {
  config,
  isCypressTestEnv,
  INFURA_API_KEY_RINKEBY,
} from "ui/src/config";
import * as bacon from "ui/src/bacon";

// Data provided by a connected wallet
export interface Connection {
  accountAddress: string;
  chainId: number;
}

// Interface for showing the WalletConnect connection URL.
//
// The `abort` callback allows the user to abort the connection
// process.
export interface QrDisplay {
  show: (uri: string, abort: () => void) => void;
}

export interface WalletConnect {
  // Holds the connection state. This is updated whenever a wallet
  // connects or disconnects or updates its parameters.
  connection: svelteStore.Readable<Connection | undefined>;

  // Start the connection flow by calling `show` on `qrDisplay` to
  // display the session URI. Returns `true` if the connection has been
  // succesfully established and `false` if `qrDisplay` called the
  // `abort` callback.
  connect(qrDisplay: QrDisplay): Promise<boolean>;

  disconnect(): Promise<void>;

  // Sign a message according to [EIP-191][1] with the key for `address`
  //
  // [1]: https://eips.ethereum.org/EIPS/eip-191
  signMessage(address: string, message: Uint8Array): Promise<string>;

  // Sign typed data for the given address as specified in
  // [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
  signTypedData(address: string, typedData: unknown): Promise<string>;

  // Submit a transaction to the network via the wallet and return the
  // transaction hash. Requires user confirmation. Throws when no
  // wallet is connected.
  sendTransaction(tx: ITxData): Promise<string>;
  // Have the user sign a transaction and return the signed transaction
  // encoded as hex string. Throws when no wallet is connected.
  signTransaction(tx: ITxData): Promise<string>;
}

// Create a `WalletConnect` instance. Creates a test instance when
// running in Cypress.
export function createWalletConnect(): WalletConnect {
  if (isCypressTestEnv) {
    return new TestClient(
      "image napkin cruise dentist name plunge crisp muscle nest floor vessel blush",
      1
    );
  } else if (config.isDev && config.testWalletMnemonic) {
    return new TestClient(config.testWalletMnemonic, 4);
  } else {
    return new WalletConnectClient();
  }
}

export class WalletConnectClient implements WalletConnect {
  private connector: Connector;
  // Mutex to synchronize connection and disconnection.
  private connectionMutex = new Mutex();
  private _connection = svelteStore.writable<Connection | undefined>(undefined);
  private qrDisplayRequest = new bacon.Bus<{
    uri: string;
    onClose: () => void;
  }>();

  public connection: svelteStore.Readable<Connection | undefined>;

  constructor() {
    // `this.connector` is set by `reinit()`
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.connector = undefined as any;
    this.reinit();
    this.connection = this._connection;
    if (this.connector.connected) {
      this._connection.set({
        chainId: this.connector.chainId,
        accountAddress: this.connector.accounts[0],
      });
    }
  }

  async connect(qrDisplay: QrDisplay): Promise<boolean> {
    return tryRunExclusive(this.connectionMutex, async () => {
      this.qrDisplayRequest.first().onValue(({ uri, onClose }) => {
        qrDisplay.show(uri, onClose);
      });
      try {
        const sessionStatus = await this.connector.connect();
        this.setConnection(sessionStatus);
        return true;
      } catch (e) {
        this.reinit();
        if (e.message.includes("User close")) {
          return false;
        } else {
          throw e;
        }
      }
    });
  }

  async disconnect(): Promise<void> {
    return tryRunExclusive(this.connectionMutex, async () => {
      await this.connector.killSession().catch(() => {
        // When the user disconnects wallet-side, calling `killSession`
        // app-side trows an error because the wallet has already closed
        // its socket. Therefore, we simply ignore it.
      });
      this.reinit();
    });
  }

  signMessage(address: string, message: Uint8Array): Promise<string> {
    const messageDigest = ethers.utils.hashMessage(message);
    return this.connector.signMessage([address, messageDigest]);
  }

  signTypedData(address: string, typedData: unknown): Promise<string> {
    return this.connector.signTypedData([address, JSON.stringify(typedData)]);
  }

  sendTransaction(tx: ITxData): Promise<string> {
    return this.connector.sendTransaction(tx);
  }

  signTransaction(tx: ITxData): Promise<string> {
    return this.connector.sendTransaction(tx);
  }

  // It’s not possible to re-use WalletConnect connector instances so
  // we have to create a new one after errors and disconnects
  //
  // See
  // https://github.com/WalletConnect/walletconnect-monorepo/issues/538
  // and
  // https://github.com/WalletConnect/walletconnect-monorepo/pull/370#issuecomment-776038638
  private reinit() {
    this.connector = new Connector({
      bridge: "https://bridge.walletconnect.org",
      qrcodeModal: {
        open: (uri: string, onClose, _opts?: unknown) => {
          this.qrDisplayRequest.push({ uri, onClose });
        },
        close: () => {},
      },
    });

    const clientMeta = {
      name: "Radicle Upstream",
      description: "Desktop client for Radicle",
      url: "http://radicle.xyz",
      icons: ["https://radicle.xyz/img/radicle-walletconnect-icon.png"],
    };

    // @ts-expect-error: Electron owerwrites window APIs, which means that when
    // setting `clientMeta` via the Connector params they get overwritten and
    // return `undefined`.
    this.connector._clientMeta = clientMeta;

    // We should remove the event listeners from the previous
    // instance but WalletConnect does not yet support this.
    //
    // https://github.com/WalletConnect/walletconnect-monorepo/issues/340
    this.connector.on("disconnect", () => {
      // WalletConnect only clears the local storage _after_ the
      // "disconnect" event is fired. If we call `reinit()`
      // immediately, the new instance will use the state in local
      // storage which is still connected.
      setTimeout(() => {
        this.reinit();
        this.setConnection(undefined);
      });
    });

    this.connector.on("session_update", (_error, { params }) => {
      this.setConnection(params[0]);
    });
  }

  private setConnection(sessionStatus: ISessionStatus | undefined) {
    let connection;
    if (sessionStatus) {
      connection = {
        chainId: sessionStatus.chainId,
        accountAddress: sessionStatus.accounts[0],
      };
    }
    const previousConnection = svelteStore.get(this._connection);
    if (!isEqual(previousConnection, connection)) {
      this._connection.set(connection);
    }
  }
}

// WalletConnect test client that is backed by an in-memory wallet and
// automatically signs and submits transactions without user
// interaction.
export class TestClient implements WalletConnect {
  private _connection: svelteStore.Writable<Connection | undefined> =
    svelteStore.writable(undefined);
  private chainId: number;
  private wallet: ethers.Wallet;
  private provider: ethers.providers.Provider;

  public connection: svelteStore.Readable<Connection | undefined>;

  // Create a new `TestClient`.
  //
  // If `chain` id is 1 (i.e. mainnet) then we submit transactions to a
  // local node. Otherwise, we submit transaction to the Rinkeby
  // testnet.
  constructor(mnemonic: string, chainId: number) {
    this.connection = svelteStore.derived(this._connection, x => x);
    this.wallet = ethers.Wallet.fromMnemonic(mnemonic);
    this.chainId = chainId;
    if (chainId === 1) {
      this.provider = new ethers.providers.JsonRpcProvider(
        "http://localhost:8545"
      );
    } else {
      this.provider = ethers.providers.InfuraProvider.getWebSocketProvider(
        "rinkeby",
        INFURA_API_KEY_RINKEBY
      );
    }
  }

  async connect(): Promise<boolean> {
    this._connection.set({
      accountAddress: this.wallet.address,
      chainId: this.chainId,
    });
    return true;
  }

  async disconnect(): Promise<void> {
    this._connection.set(undefined);
  }

  async signMessage(address: string, message: Uint8Array): Promise<string> {
    if (svelteStore.get(this.connection) === undefined) {
      throw new Error.Error({
        message: "Cannot sign message. Wallet is not connected",
      });
    }

    if (address.toLowerCase() !== this.wallet.address.toLowerCase()) {
      throw new Error.Error({
        message: "TestClient.signMessage is not implemented",
      });
    }

    return this.wallet.signMessage(message);
  }

  signTypedData(_address: string, _typedData: unknown): Promise<string> {
    throw new Error.Error({
      message: "TestClient.signTypedData is not implemented",
    });
  }

  async sendTransaction(tx: ITxData): Promise<string> {
    const signedTx = await this.signTransaction(tx);
    const response = await this.provider.sendTransaction(signedTx);
    return response.hash;
  }

  async signTransaction(tx: ITxData): Promise<string> {
    if (svelteStore.get(this.connection) === undefined) {
      throw new Error.Error({
        message: "Cannot sign transaction. Wallet is not connected",
      });
    }
    const nonce =
      tx.nonce || (await this.provider.getTransactionCount(tx.from));
    const gasPrice = tx.gasPrice || (await this.provider.getGasPrice());

    const txRequest = {
      to: tx.to,
      from: tx.from,
      nonce,
      gasPrice,
      data: tx.data,
    };

    const gasLimit =
      tx.gasLimit || (await this.provider.estimateGas(txRequest));

    return this.wallet.signTransaction({
      ...txRequest,
      gasLimit,
    });
  }
}

// Try to run `f` with `mutex.runExclusive()`. Throws an error if
// `mutex` is locked now instead of waiting for it to be unlocked.
function tryRunExclusive<T>(mutex: Mutex, f: () => Promise<T>): Promise<T> {
  if (mutex.isLocked()) {
    throw new Error.Error({ message: "Mutex is locked" });
  } else {
    return mutex.runExclusive(f);
  }
}
