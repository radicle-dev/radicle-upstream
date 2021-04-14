import * as ethers from "ethers";
import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";
import * as svelteStore from "svelte/store";

const orgFactoryAbi = ["function createOrg(address) returns (address)"];

const addresses = {
  orgFactory: {
    ropsten: "0xe30aA5594FFB52B6bF5bbB21eB7e71Ac525bB028",
  },
};

export enum Status {
  Initial = "INITIAL", // Initial state, nothing has happened yet
  Waiting = "WAITING", // Waiting for signature from wallet
  Pending = "PENDING", // Waiting for transaction to be mined
  Success = "SUCCESS", // Transaction mined and successful
  Failed = "FAILED", // Transaction failed
}

export const store: svelteStore.Writable<Status> = svelteStore.writable(
  Status.Initial
);

export async function createOrg(
  owner: string,
  signer: ethers.Signer
): Promise<string> {
  console.log("createOrg", owner, signer);
  const orgFactory = new ethers.Contract(
    addresses.orgFactory.ropsten,
    orgFactoryAbi,
    signer
  );
  store.set(Status.Waiting);

  try {
    const response: TransactionResponse = await orgFactory.createOrg(owner);
    // NEVER REACHED!
    console.log(response);
    store.set(Status.Pending);
    const receipt: TransactionReceipt = await response.wait();
    console.log(receipt);

    store.set(Status.Success);
  } catch (e) {
    store.set(Status.Failed);
  }

  return "";
}
