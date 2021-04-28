import * as ethers from "ethers";
import { push } from "svelte-spa-router";

import * as notification from "./notification";
import * as path from "./path";

import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";

import * as transaction from "./transaction";

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

export const createOrg = async (
  owner: string,
  signer: ethers.Signer,
  provider: ethers.providers.Provider
): Promise<void> => {
  const orgFactory = new ethers.Contract(
    addresses.orgFactory.ropsten,
    orgFactoryAbi,
    signer
  );
  notification.info({
    message:
      "Waiting for you to confirm the transaction in your connected wallet",
    showIcon: true,
  });
  // WAITING
  const response: TransactionResponse = await orgFactory.createOrg(owner);

  // PENDING
  notification.info({
    message: "Org creation transaction confirmed, your org will appear shortly",
    showIcon: true,
  });

  const receipt: TransactionReceipt = await provider.waitForTransaction(
    response.hash
  );
  transaction.add(transaction.createOrg(response));

  const iface = new ethers.utils.Interface(orgFactoryAbi);

  let orgAddress: string = "";

  receipt.logs.forEach(log => {
    try {
      const parsed = iface.parseLog(log);

      if (parsed.name === "OrgCreated") {
        orgAddress = parsed.args[0];
      }
    } catch {
      // Ignore parsing errors.
    }
  });

  if (!orgAddress) {
    throw new Error("Org not found in interface logs");
  }

  // SUCCESS
  notification.info({
    message: `Org ${orgAddress} has been created`,
    showIcon: true,
    actions: [
      {
        label: "Go to org",
        handler: () => {
          push(path.org(orgAddress));
        },
      },
    ],
  });
};

export const getSafeAddr = async (
  orgAddress: string,
  provider: ethers.providers.Provider
): Promise<string | null> => {
  const org = new ethers.Contract(orgAddress, orgAbi, provider);
  const safeAddr: string = await org.owner();

  return safeAddr;
};
