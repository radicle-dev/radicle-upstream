// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// This module provides a facade for WalletConnect that makes it easier to use.

import { Mutex } from "async-mutex";
import Connector from "@walletconnect/client";
import type { ITxData } from "@walletconnect/types";

import * as Error from "ui/src/error";
import * as modal from "ui/src/modal";
import ModalWalletQRCode from "ui/Modal/Wallet/QRCode.svelte";
import * as Bacon from "ui/src/bacon";

// Data provided by a connected wallet
export interface Connection {
  accountAddress: string;
  chainId: number;
}

export interface WalletConnect {
  // Emits an event when the wallet is disconnected. The disconnection
  // may be triggered by the app or by the wallet.
  disconnected: Bacon.EventStream<void>;

  // If we are connected to a wallet, return the connection information.
  getConnection(): Connection | undefined;

  // Start the connection flow by showing the modal with the connection
  // data. Returns `true` if the connection has been succesfully
  // established and `false` if the user closed the modal without
  // connecting.
  connect(): Promise<boolean>;

  disconnect(): Promise<void>;

  // Sign a message with the key for `address`. `message` must be
  // hex-encoded.
  signMessage(address: string, message: string): Promise<string>;
  // Submit a transaction to the network via the wallet and return the
  // transaction hash. Requires user confirmation. Throws when no
  // wallet is connected.
  sendTransaction(tx: ITxData): Promise<string>;
  // Have the user sign a transaction and return the signed transaction
  // encoded as hex string. Throws when no wallet is connected.
  signTransaction(tx: ITxData): Promise<string>;
}

export function createWalletConnect(): WalletConnect {
  return new WalletConnectClient();
}

export class WalletConnectClient implements WalletConnect {
  private connector: Connector;
  private disconnectedBus = new Bacon.Bus<void>();
  // Mutex to synchronize connection and disconnection.
  private connectionMutex = new Mutex();

  public disconnected: Bacon.EventStream<void>;

  constructor() {
    this.connector = createConnector();
    this.disconnected = this.disconnectedBus.toEventStream();
    this.reinit();
  }

  getConnection(): Connection | undefined {
    if (this.connector.connected) {
      return {
        chainId: this.connector.chainId,
        accountAddress: this.connector.accounts[0],
      };
    } else {
      return undefined;
    }
  }

  async connect(): Promise<boolean> {
    return tryRunExclusive(this.connectionMutex, async () => {
      try {
        await this.connector.connect();
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

  signMessage(address: string, message: string): Promise<string> {
    return this.connector.signMessage([address, message]);
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
    this.connector = createConnector();
    // We should remove the `disconnect` listener from the previous
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
        this.disconnectedBus.push();
      });
    });
  }
}

function createConnector(): Connector {
  // This is set to true if the WalletConnect code closes the modal
  // instead of the user. This prevents calling back into WalletConnect
  // with `onClose` and aborting the connection.
  let modalClosedByWalletConnect = false;

  return new Connector({
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

// Try to run `f` with `mutex.runExclusive()`. Throws an error if
// `mutex` is locked now instead of waiting for it to be unlocked.
function tryRunExclusive<T>(mutex: Mutex, f: () => Promise<T>): Promise<T> {
  if (mutex.isLocked()) {
    throw new Error.Error({ message: "Mutex is locked" });
  } else {
    return mutex.runExclusive(f);
  }
}
