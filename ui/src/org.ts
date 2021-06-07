import * as svelteStore from "svelte/store";
import * as ethers from "ethers";
import EthersSafe from "@gnosis.pm/safe-core-sdk";
import SafeServiceClient from "@gnosis.pm/safe-service-client";
import { OperationType } from "@gnosis.pm/safe-core-sdk-types";

import * as notification from "ui/src//notification";
import * as wallet from "ui/src/wallet";
import * as theGraphApi from "ui/src/theGraphApi";
import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import * as proxy from "ui/src/proxy";
import * as urn from "ui/src/urn";
import * as router from "ui/src/router";
import * as modal from "ui/src/modal";
import type * as project from "ui/src/proxy/project";

import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";

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

export const anchorProject = async (
  orgAddress: string,
  gnosisSafeAddress: string,
  projectUrn: string,
  commitHash: string
): Promise<void> => {
  const walletStore = svelteStore.get(wallet.store);
  const checksummedGnosisSafeAddress =
    ethers.utils.getAddress(gnosisSafeAddress);
  const checksummedOrgAddress = ethers.utils.getAddress(orgAddress);
  const safeSdk = await EthersSafe.create(
    ethers,
    checksummedGnosisSafeAddress,
    walletStore.signer
  );

  // TODO: switch this based on testnet, Gnosis only provides a tx service on
  // rinkeby though
  const safeServiceClient = new SafeServiceClient(
    "https://safe-transaction.rinkeby.gnosis.io"
  );

  const decodedProjectUrn = urn.parseIdentitySha1(projectUrn);
  const decodedCommitHash = ethers.utils.arrayify(`0x${commitHash}`);

  const paddedProjectUrn = ethers.utils.zeroPad(decodedProjectUrn, 32);
  const paddedCommitHash = ethers.utils.zeroPad(decodedCommitHash, 32);

  const orgContract = new ethers.Contract(checksummedGnosisSafeAddress, orgAbi);

  const orgContractInstance = await orgContract.populateTransaction.anchor(
    paddedProjectUrn,
    paddedCommitHash,
    ethers.constants.Zero,
    ethers.constants.Zero
  );

  const txData = orgContractInstance.data;
  if (!txData) {
    throw new Error("Could not generate transaction");
  }

  // WAITING
  notification.info({
    message:
      "Waiting for you to confirm the anchor transaction in your connected wallet",
    showIcon: true,
  });

  const tx = {
    to: checksummedOrgAddress,
    value: "0",
    data: txData,
    operation: OperationType.Call,
  };
  const estimation = await safeServiceClient.estimateSafeTransaction(
    checksummedGnosisSafeAddress,
    tx
  );
  const transaction = await safeSdk.createTransaction({
    ...tx,
    safeTxGas: Number(estimation.safeTxGas),
  });
  const safeTxHash = await safeSdk.getTransactionHash(transaction);

  const signature = await safeSdk.signTransactionHash(safeTxHash);

  try {
    await safeServiceClient.proposeTransaction(
      checksummedGnosisSafeAddress,
      transaction.data,
      safeTxHash,
      signature
    );
  } catch (error) {
    console.log(error);
  }

  // PENDING
  notification.info({
    message:
      "Project anchoring confirmed, your anchored project will appear shortly",
    showIcon: true,
  });

  // TODO: how do we automatically refresh the view to show the anchor?
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
      "Waiting for you to confirm the org creation transaction in your connected wallet",
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
          router.push({
            type: "org",
            address: orgAddress,
            activeTab: "projects",
          });
        },
      },
    ],
  });
  await fetchOrgs();
};

const fetchGnosisSafeAddr = async (
  orgAddress: string,
  provider: ethers.providers.Provider
): Promise<string> => {
  const org = new ethers.Contract(orgAddress, orgAbi, provider);
  const safeAddr: string = await org.owner();

  return safeAddr.toLowerCase();
};

export const fetchOrg = async (
  orgAddress: string
): Promise<{
  orgAddress: string;
  gnosisSafeAddress: string;
}> => {
  const walletStore = svelteStore.get(wallet.store);
  const gnosisSafeAddress = await fetchGnosisSafeAddr(
    orgAddress,
    walletStore.provider
  );
  return { orgAddress, gnosisSafeAddress };
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

export const fetchMembers = async (
  gnosisSafeAddress: string
): Promise<theGraphApi.MemberResponse> => {
  return await theGraphApi.getGnosisSafeMembers(gnosisSafeAddress);
};

export const resolveProjectAnchors = async (
  orgAddress: string
): Promise<{
  anchoredProjects: project.Project[];
  unresolvedAnchors: theGraphApi.ProjectAnchor[];
}> => {
  const anchors = await theGraphApi.getOrgProjectAnchors(orgAddress);

  const anchoredProjects: project.Project[] = [];
  const unresolvedAnchors: theGraphApi.ProjectAnchor[] = [];

  await Promise.all(
    anchors.map(async anchor => {
      try {
        const project = await proxy.client.project.get(anchor.projectId);
        anchoredProjects.push({ ...project, anchor });
      } catch (error) {
        // TODO: only catch when backend can't find project, reraise other errors
        unresolvedAnchors.push(anchor);
      }
    })
  );

  return { anchoredProjects, unresolvedAnchors };
};

export interface ProjectOption {
  title: string;
  value: urn.Urn;
}

export const fetchProjectDropdownOptions = async (): Promise<
  ProjectOption[]
> => {
  const [tracked, contributed] = await Promise.all([
    proxy.client.project.listTracked(),
    proxy.client.project.listContributed(),
  ]);
  const allProjects = [...tracked, ...contributed];

  return allProjects.map(project => {
    return { title: project.metadata.name, value: project.urn };
  });
};

export const openAnchorProjectModal = async (
  orgAddress: string,
  gnosisSafeAddress: string
): Promise<void> => {
  const projects = await fetchProjectDropdownOptions();
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  modal.toggle(ModalAnchorProject, () => {}, {
    projects,
    orgAddress,
    gnosisSafeAddress,
  });
};
