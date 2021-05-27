import * as svelteStore from "svelte/store";
import * as ethers from "ethers";
import EthersSafe, {
  SafeTransactionDataPartial,
} from "@gnosis.pm/safe-core-sdk";
import { push } from "svelte-spa-router";

import * as notification from "./notification";
import * as path from "./path";
import * as wallet from "./wallet";
import * as theGraphApi from "./theGraphApi";
import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import * as proxy from "ui/src/proxy";
import * as urn from "ui/src/urn";
import * as project from "ui/src/proxy/project";

import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";

import * as transaction from "./transaction";

const orgFactoryAbi = [
  "function createOrg(address[], uint256) returns (address)",
  "event OrgCreated(address, address)",
];

const orgAbi = [
  "function owner() view returns (address)",
  "function anchor(bytes32, bytes32, uint8, uint8)",
];

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

export const anchorProject = async (): Promise<void> => {
  const orgAddress = "0x01acd1dded15eadf7ed8de1885a9541c5481eb60";
  const gnosisSafeAddress = "0xb173a59b8f315a4bd36e218207b71dc9d5f79d8b";
  const projectUrn = "rad:git:hnrke3q1pob41qjq5y57xn698xzt86yht74by";
  const commitHash = "900b6cf6cf1ff822a423bb47cecb9eb80738bff4";

  const walletStore = svelteStore.get(wallet.store);
  const safeSdk = await EthersSafe.create(
    ethers,
    gnosisSafeAddress,
    walletStore.signer
  );

  console.log(safeSdk);

  const decodedProjectUrn = urn.parseIdentitySha1(projectUrn);
  const decodedCommitHash = ethers.utils.arrayify(commitHash);

  const paddedProjectUrn = ethers.utils.zeroPad(decodedProjectUrn, 32);
  const paddedCommitHash = ethers.utils.zeroPad(decodedCommitHash, 32);

  console.log(paddedProjectUrn);
  console.log(paddedCommitHash);

  const orgContract = new ethers.Contract(orgAddress, orgAbi);

  console.log("AAAAAAAAAA: ", orgContract);
  const orgContractInstance = await orgContract.populateTransaction.anchor(
    paddedProjectUrn,
    paddedCommitHash,
    ethers.constants.Zero,
    ethers.constants.Zero
  );

  console.log(orgContractInstance);

  const txData = orgContractInstance.data;
  if (!txData) {
    throw new Error("Could not generate transaction");
  }
  console.log("BBBBBBBB: ", txData);

  const partialTx: SafeTransactionDataPartial = {
    to: orgAddress,
    data: txData,
    value: "0",
  };

  // THIS WORKS IF THERE'S ONLY ONE OWNER IN THE SAFE
  console.log("PARTIAL: ", partialTx);
  const safeTransaction = await safeSdk.createTransaction(partialTx);
  const approveTxResponse = await safeSdk.executeTransaction(safeTransaction);
  console.log(approveTxResponse);

  // const safeTransaction = await safeSdk.createTransaction(partialTx);
  // const txHash = await safeSdk.getTransactionHash(safeTransaction);
  // const approveTxResponse = await safeSdk.approveTransactionHash(txHash);
  // await approveTxResponse.wait();
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

interface UnresolvedAnchoredProject {
  anchor: theGraphApi.ProjectAnchor;
}

interface ResolvedAnchoredProject {
  anchor: theGraphApi.ProjectAnchor;
  project: project.Project;
}

type AnchoredProject = UnresolvedAnchoredProject | ResolvedAnchoredProject;

export const orgProjectTabStore =
  svelteStore.writable<AnchoredProject[] | null>(null);

export const fetchAnchoredProjects = async (
  orgAddress: string
): Promise<void> => {
  const ethereumAnchors = await theGraphApi.getOrgProjectAnchors(orgAddress);

  const anchoredProjects: AnchoredProject[] = await Promise.all(
    ethereumAnchors.map(async anchor => {
      try {
        const project = await proxy.client.project.get(anchor.projectId);
        return <ResolvedAnchoredProject>{ anchor, project };
      } catch (error) {
        // TODO: only catch when backend can't find project, reraise other errors
        return <UnresolvedAnchoredProject>{ anchor };
      }
    })
  );

  orgProjectTabStore.set(anchoredProjects);
};
