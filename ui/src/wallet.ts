import WalletConnect from "@walletconnect/client";
import QRCodeModal from "@walletconnect/qrcode-modal";
import WalletConnectWeb3Provider from "@walletconnect/web3-provider";
import Web3 from "web3";
import { writable, Readable } from "svelte/store";

type State =
  | { status: "NOT_CONNECTED" }
  | { status: "CONNECTING" }
  | { status: "CONNECTED"; connected: Connected };

interface Connected {
  chainId: number;
  account: {
    address: string | null;
    balance: string | null;
  };
}

interface Wallet extends Readable<State> {
  disconnect(): Promise<void>;
  connect(): Promise<void>;
}

function makeWallet(): Wallet {
  const stateStore = writable<State>({ status: "NOT_CONNECTED" });

  const connector = new WalletConnect({
    bridge: "https://bridge.walletconnect.org",
    qrcodeModal: QRCodeModal,
  });

  const provider = new WalletConnectWeb3Provider({
    infuraId: "47a262b47b864e69ab21425bacf4c3df",
    // Uncomment this to connect to local node
    rpc: {
      1: "http://localhost:8545",
    },
    connector,
  });

  const web3 = new Web3(provider as any);

  async function connect() {
    stateStore.set({ status: "CONNECTING" });
    // TODO handle error
    const accountAddresses: string[] = (await provider.enable()) as any;
    // TODO handle missing account addresses
    const accountAddress = accountAddresses[0];
    const balance = await web3.eth.getBalance(accountAddress || "");
    const connected = {
      chainId: provider.chainId,
      account: {
        address: accountAddress,
        balance: balance,
      },
    };
    stateStore.set({ status: "CONNECTED", connected });
  }

  if (connector.connected) {
    connect();
  }

  connector.on("disconnect", () => {
    console.log("connector.disconnect");
    // TODO clean up
    stateStore.set({ status: "NOT_CONNECTED" });
  });

  provider.on("chainChanged", (chainId: number) => {
    stateStore.update(state => {
      if (state.status === "CONNECTED") {
        state.connected.chainId = chainId;
      }
      return state;
    });
  });

  provider.on("accountsChanged", async (accountAddresses: string[]) => {
    console.log(accountAddresses);
    const accountAddress = accountAddresses[0];
    const balance = await web3.eth.getBalance(accountAddress);
    stateStore.update(state => {
      if (state.status === "CONNECTED") {
        state.connected.account = {
          address: accountAddress,
          balance: balance,
        };
      }
      return state;
    });
  });

  provider.on("error", (error: any) => {
    console.log("provider.error", error);
  });

  return {
    subscribe: stateStore.subscribe,
    async disconnect(): Promise<void> {
      await provider.disconnect();
      stateStore.set({ status: "NOT_CONNECTED" });
    },
    connect,
  };
}

export const wallet = makeWallet();
