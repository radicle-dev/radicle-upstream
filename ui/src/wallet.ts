import WalletConnect from "@walletconnect/client";
import { convertUtf8ToHex } from "@walletconnect/utils";
import QRCodeModal from "@walletconnect/qrcode-modal";
import WalletConnectWeb3Provider from "@walletconnect/web3-provider";
import Web3 from "web3";
import { writable, Readable } from "svelte/store";

export enum Status {
  Connected = "CONNECTED",
  NotConnected = "NOT_CONNECTED",
}

type State =
  | { status: Status.NotConnected }
  | { status: Status.Connected; connected: Connected };

interface Connected {
  chainId: number;
  account: {
    address: string | null;
    balance: string | null;
  };
}

interface Wallet extends Readable<State> {
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  testSign(): Promise<void>;
}

export function makeWallet(): Wallet {
  const stateStore = writable<State>({ status: Status.NotConnected });
  const connector = new WalletConnect({
    bridge: "https://bridge.walletconnect.org",
    qrcodeModal: QRCodeModal,
  });
  const provider = new WalletConnectWeb3Provider({
    infuraId: "47a262b47b864e69ab21425bacf4c3df",
    // Comment this below to connect to the mainnet instead
    rpc: {
      1: "http://localhost:8545",
    },
    connector,
  });
  const web3 = new Web3(provider as any);

  // Connect to a wallet using walletconnect
  async function connect() {
    // TODO handle error
    const accountAddresses: string[] = (await provider.enable()) as any;
    // TODO handle missing account addresses
    console.log("Connect accountAddresses", accountAddresses);
    const accountAddress = accountAddresses[0];
    const balance = await web3.eth.getBalance(accountAddress || "");
    const connected = {
      chainId: provider.chainId,
      account: {
        address: accountAddress,
        balance: balance,
      },
    };
    stateStore.set({ status: Status.Connected, connected });
  }

  // Connect automatically if the underlying connection is still open.
  if (connector.connected) {
    connect();
  }

  // TODO(nuno): this isn't working as expected, fix it.
  async function disconnect() {
    await provider.disconnect();
    await connector.killSession();

    stateStore.set({ status: Status.NotConnected });
  }

  async function testSign() {
    stateStore.subscribe(async state => {
      if (state.status === Status.Connected) {
        if (!connector) {
          console.log("Connector is undefined, stopping here.");
          return;
        }

        const address = state.connected.account.address;
        const message = "My email is john@doe.com - 1537836206101";
        const hexMsg = convertUtf8ToHex(message);
        const msgParams = [hexMsg, address];
        console.log("msgParams", msgParams);

        const result = await connector.signPersonalMessage(msgParams);
        console.log("Result is ", result);
      }
    });
  }

  connector.on("disconnect", () => {
    console.log("connector.disconnect");
  });

  provider.on("chainChanged", (chainId: number) => {
    stateStore.update(state => {
      if (state.status === Status.Connected) {
        state.connected.chainId = chainId;
      }
      return state;
    });
  });

  provider.on("accountsChanged", async (accountAddresses: string[]) => {
    const accountAddress = accountAddresses[0];
    const balance = await web3.eth.getBalance(accountAddress);
    stateStore.update(state => {
      if (state.status === Status.Connected) {
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
    connect,
    disconnect,
    testSign,
  };
}
