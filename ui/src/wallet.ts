import { Readable } from "svelte/store";

import * as walletconnect from "./wallet/walletconnect";

export enum Status {
  Connected = "CONNECTED",
  NotConnected = "NOT_CONNECTED",
}

export type State =
  | { status: Status.NotConnected }
  | { status: Status.Connected; connected: Connected };

export interface Connected {
  chainId: number;
  account: {
    address: string | null;
    balance: string | null;
  };
}

export interface Wallet extends Readable<State> {
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  testSign(): Promise<void>;
}

export enum Connector {
  WalletConnect,
}

// Build a new Wallet using the indicated Connector alternative
export function build(connector: Connector = Connector.WalletConnect): Wallet {
  switch (connector) {
    case Connector.WalletConnect:
      return walletconnect.build();
  }
}
