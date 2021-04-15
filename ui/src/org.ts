import * as ethers from "ethers";
import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";
import * as svelteStore from "svelte/store";

const orgFactoryAbi = [
  "function createOrg(address) returns (address)",
  "event OrgCreated(address)",
];

const orgAbi = ["function owner() view returns (address)"];

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
  signer: ethers.Signer,
  provider: ethers.providers.Provider
): Promise<string | null> {
  console.log("createOrg", owner, signer);
  const orgFactory = new ethers.Contract(
    addresses.orgFactory.ropsten,
    orgFactoryAbi,
    signer
  );
  store.set(Status.Waiting);

  try {
    const response: TransactionResponse = await orgFactory.createOrg(owner);
    console.log(response);
    store.set(Status.Pending);
    const receipt: TransactionReceipt = await provider.waitForTransaction(
      response.hash
    );
    console.log(receipt);

    const iface = new ethers.utils.Interface(orgFactoryAbi);
    console.log(iface);

    let orgAddr: string = "";
    receipt.logs.forEach(log => {
      const parsed = iface.parseLog(log);
      if (parsed.name === "OrgCreated") {
        orgAddr = parsed.args[0];
        return;
      }
    });
    if (!orgAddr) {
      throw "No 'OrgCreated' event";
    }
    console.log("orgAddr", orgAddr);

    store.set(Status.Success);

    return orgAddr;
  } catch (e) {
    console.log(e);
    store.set(Status.Failed);
  }

  return null;
}

export async function getOrgSafeAddr(
  orgAddr: string,
  provider: ethers.providers.Provider
): Promise<string | null> {
  const org = new ethers.Contract(orgAddr, orgAbi, provider);
  const safeAddr: string = await org.owner();

  return safeAddr;
}
