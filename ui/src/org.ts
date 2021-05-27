import * as svelteStore from "svelte/store";
import * as ethers from "ethers";
import { push } from "svelte-spa-router";

import * as notification from "./notification";
import * as path from "./path";
import * as wallet from "./wallet";
import * as theGraphApi from "./theGraphApi";
import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";

import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";

import * as transaction from "./transaction";

const orgFactoryAbi = [
  "function createOrg(address[], uint256) returns (address)",
  "event OrgCreated(address, address)",
];

const orgAbi = ["function owner() view returns (address)"];

const orgFactoryAddress = (network: ethereum.Environment): string => {
  switch (network) {
    case ethereum.Environment.Local:
      error.show(
        new error.Error({
          code: error.Code.FeatureNotAvailableForGivenNetwork,
          message: "Orgs not available on the Local testnet.",
        })
      );
      return "";
    case ethereum.Environment.Ropsten:
      return "0x2007bcEf1247CD03Bb4262eF420D6487368f473B";
    case ethereum.Environment.Rinkeby:
      return "0xe30aA5594FFB52B6bF5bbB21eB7e71Ac525bB028";
  }
};

export const createOrg = async (owner: string): Promise<void> => {
  const walletStore = svelteStore.get(wallet.store);
  const orgFactory = new ethers.Contract(
    orgFactoryAddress(walletStore.environment),
    orgFactoryAbi,
    walletStore.signer
  );
  notification.info({
    message:
      "Waiting for you to confirm the transaction in your connected wallet",
    showIcon: true,
  });
  // WAITING
  const response: TransactionResponse = await orgFactory.createOrg([owner], 1);

  // PENDING
  notification.info({
    message: "Org creation transaction confirmed, your org will appear shortly",
    showIcon: true,
  });

  const receipt: TransactionReceipt =
    await walletStore.provider.waitForTransaction(response.hash);
  transaction.add(transaction.createOrg(response));

  const iface = new ethers.utils.Interface(orgFactoryAbi);

  let orgAddress: string = "";

  receipt.logs.forEach(log => {
    try {
      const parsed = iface.parseLog(log);

      if (parsed.name === "OrgCreated") {
        orgAddress = parsed.args[0].toLowerCase();
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
          push(path.orgProjects(orgAddress));
        },
      },
    ],
  });
  await fetchOrgs();
};

const getGnosisSafeAddr = async (
  orgAddress: string,
  provider: ethers.providers.Provider
): Promise<string> => {
  const org = new ethers.Contract(orgAddress, orgAbi, provider);
  const safeAddr: string = await org.owner();

  return safeAddr.toLowerCase();
};

export type EthereumAddress = string;

interface OrgScreenStore {
  orgAddress: EthereumAddress;
  gnosisSafeAddress: EthereumAddress;
}

export const orgScreenStore = svelteStore.writable<OrgScreenStore | null>(null);

export const fetchOrg = async (orgAddress: EthereumAddress): Promise<void> => {
  // Don't re-fetch if we already have the org.
  if (svelteStore.get(orgScreenStore)?.orgAddress === orgAddress) {
    return;
  }

  const walletStore = svelteStore.get(wallet.store);
  const gnosisSafeAddress = await getGnosisSafeAddr(
    orgAddress,
    walletStore.provider
  );
  orgScreenStore.set({ orgAddress, gnosisSafeAddress });
};

export const orgSidebarStore = svelteStore.writable<theGraphApi.Org[] | []>([]);

export const fetchOrgs = async (): Promise<void> => {
  const walletStore = svelteStore.get(wallet.store);
  const w = svelteStore.get(walletStore);

  if (w.status !== wallet.Status.Connected) {
    throw new Error(
      "Tried to call fetchOrgs while the wallet wasn't connected"
    );
  }

  const orgs = await theGraphApi.getOrgs(w.connected.account.address);
  orgSidebarStore.set(orgs);
};

interface OrgMemberTabStore extends theGraphApi.MemberResponse {
  gnosisSafeAddress: string;
}

export const orgMemberTabStore =
  svelteStore.writable<OrgMemberTabStore | null>(null);

export const fetchMembers = async (
  gnosisSafeAddress: string
): Promise<void> => {
  const response = await theGraphApi.getGnosisSafeMembers(gnosisSafeAddress);
  orgMemberTabStore.set({ gnosisSafeAddress, ...response });
};
