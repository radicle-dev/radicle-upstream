import * as ethers from "ethers";
import * as svelteStore from "svelte/store";

const orgFactoryAbi = ["function createOrg(address) returns (address)"];

const addresses = {
  orgFactory: {
    ropsten: "0x0000000000000000000000000000000000000000",
  },
};

export enum Status {
  Initial = "INITIAL", // Initial state, nothing has happened yet
  Waiting = "WAITING", // Waiting for signature from wallet
  Pending = "PENDING", // Waiting for transaction to be mined
  Success = "SUCCESS", // Transaction mined and successful
  Failed = "FAILED", // Transaction failed
}

export const store: svelteStore.Readable<Status> = svelteStore.writable(
  Status.Initial
);

export async function createOrg(
  owner: ethers.Address,
  signer: ethers.Signer
): void {
  const orgFactory = new ethers.Contract(
    addresses.orgFactory.ropsten,
    orgFactoryAbi,
    signer
  );
  store.set(Status.Waiting);

  try {
    const response = await orgFactory.createOrg(owner);
    store.set(Status.Pending);
    await response.wait();
    store.set(Status.Success);
  } catch (e) {
    store.set(Status.Failed);
  }
}
