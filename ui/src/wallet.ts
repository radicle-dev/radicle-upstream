import { Readable } from "svelte/store";
import * as svelteStore from "svelte/store";
import * as ethers from "ethers";

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
  account: {
    address: string;
    balance: string;
  };
}

export interface Wallet extends Readable<State> {
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  signer: ethers.Signer;
}

export function build(): Wallet {
  const stateStore = svelteStore.writable<State>({
    status: Status.NotConnected,
  });

  const provider = new ethers.providers.JsonRpcProvider(
    "http://localhost:8545"
  );
  const signer = provider.getSigner(0);

  window.ethereumDebug = new EthereumDebug(provider);

  // Connect to a wallet using walletconnect
  async function connect() {
    if (svelteStore.get(stateStore).status !== Status.NotConnected) {
      throw new Error("Already connected");
    }
  }

  async function initialize() {
    try {
      stateStore.set({ status: Status.Connecting });

      const accountAddress = await signer.getAddress();
      const balance = await signer.getBalance();
      const connected = {
        account: {
          address: accountAddress,
          balance: balance.toString(),
        },
      };
      stateStore.set({ status: Status.Connected, connected });
    } catch (error) {
      stateStore.set({ status: Status.NotConnected, error });
      throw error;
    }
  }

  initialize();

  return {
    subscribe: stateStore.subscribe,
    connect,
    async disconnect() {
      console.warn("Not implemented");
      return undefined;
    },
    signer,
  };
}

declare global {
  interface Window {
    ethereumDebug: EthereumDebug;
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
}
