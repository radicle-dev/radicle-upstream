import WalletConnect from "@walletconnect/client";
import { convertUtf8ToHex } from "@walletconnect/utils";
import QRCodeModal from "@walletconnect/qrcode-modal";
import WalletConnectWeb3Provider from "@walletconnect/web3-provider";
import Web3 from "web3";
import { writable } from "svelte/store";

import { Wallet, Status, State } from "../wallet";

export function build(): Wallet {
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
    try {
      stateStore.set({ status: Status.Connecting });

      const accountAddresses: string[] = (await provider.enable()) as string[];
      if (accountAddresses.length === 0) return;
      const accountAddress = accountAddresses[0];
      const balance = accountAddress
        ? await web3.eth.getBalance(accountAddress)
        : "";
      const connected = {
        chainId: provider.chainId,
        account: {
          address: accountAddress,
          balance: balance,
        },
      };
      stateStore.set({ status: Status.Connected, connected });
    } catch (e) {
      stateStore.set({ status: Status.NotConnected });
      console.log("Failed to connect: ", e);
    }
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

  async function testTransfer(value: number) {
    stateStore.subscribe(async state => {
      if (state.status === Status.Connected) {
        if (!connector) {
          console.log("Connector is undefined, stopping here.");
          stateStore.set({ status: Status.NotConnected });
          return;
        }

        const tx = {
          from: state.connected.account.address!,
          to: "0xC257274276a4E539741Ca11b590B9447B26A8051",
          nonce: 1,
          gasLimit: 1,
          gasPrice: 2,
          value,
        };

        try {
          const result = await connector.sendTransaction(tx);
          console.log("Result is ", result);
        } catch (e) {
          console.error("Failed to sendTransaction: ", e);
        }
      }
    });
  }

  async function testSign(msg: string) {
    stateStore.subscribe(async state => {
      if (state.status === Status.Connected) {
        if (!connector) {
          console.log("Connector is undefined, stopping here.");
          stateStore.set({ status: Status.NotConnected });
          return;
        }

        const address = state.connected.account.address;
        const hexMsg = convertUtf8ToHex(msg);
        const msgParams = [hexMsg, address];
        console.log("msgParams", msgParams);

        try {
          const result = await connector.signPersonalMessage(msgParams);
          console.log("Result is ", result);
        } catch (e) {
          console.error("Failed to signPersonalMessage: ", e);
        }
      }
    });
  }

  connector.on("disconnect", () => {
    console.log("connector.disconnect");
    stateStore.set({ status: Status.NotConnected });
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
    testTransfer,
    testSign,
  };
}
