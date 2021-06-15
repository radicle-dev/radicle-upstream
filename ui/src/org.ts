import * as ethers from "ethers";
import * as multihash from "multihashes";
import * as svelteStore from "svelte/store";
import EthersSafe from "@gnosis.pm/safe-core-sdk";
import SafeServiceClient from "@gnosis.pm/safe-service-client";
import { OperationType } from "@gnosis.pm/safe-core-sdk-types";

import type {
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/providers";
import type * as project from "ui/src/project";
import type { Org, Member, MemberResponse } from "ui/src/org/theGraphApi";

import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import * as ipc from "ui/src/ipc";
import * as modal from "ui/src/modal";
import * as notification from "ui/src/notification";
import * as proxy from "ui/src/proxy";
import * as router from "ui/src/router";
import * as transaction from "./transaction";
import * as urn from "ui/src/urn";
import * as wallet from "ui/src/wallet";

import {
  getOrgs,
  getGnosisSafeMembers,
  getOrgProjectAnchors,
} from "ui/src/org/theGraphApi";

export type { Member, MemberResponse };

import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";

const orgFactoryAbi = [
  "function createOrg(address[], uint256) returns (address)",
  "event OrgCreated(address, address)",
];

const orgAbi = [
  "function owner() view returns (address)",
  "function anchor(bytes32, bytes, uint8)",
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
      return "0xc074cDd9541960B0AA72D90c8bC642F6ae9C4032";
    case ethereum.Environment.Rinkeby:
      return "0x57962Eb188146A4942c44545a97478d64dc9e4A5";
    case ethereum.Environment.Mainnet:
      return "0x7c4D590168A9019995e072f9c1eCfc1fc356bEd4";
  }
};

export const openOnGnosisSafe = (
  gnosisSafeAddress: string,
  view: "transactions" | "settings"
): void => {
  const walletStore = svelteStore.get(wallet.store);

  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Gnosis Safe links are not supported on the Local testnet",
      });
    case ethereum.Environment.Ropsten:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Gnosis Safe links are not supported on the Ropsten testnet",
      });
      break;
    case ethereum.Environment.Rinkeby:
      ipc.openUrl(
        `https://rinkeby.gnosis-safe.io/app/#/safes/${gnosisSafeAddress}/${view}`
      );
      break;
    case ethereum.Environment.Mainnet:
      ipc.openUrl(
        `https://gnosis-safe.io/app/#/safes/${gnosisSafeAddress}/${view}`
      );
      break;
  }
};

const createSafeServiceClient = (): SafeServiceClient => {
  const walletStore = svelteStore.get(wallet.store);
  let uri;

  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message:
          "Pending Gnosis Safe transactions are not available on the Local testnet.",
      });
    case ethereum.Environment.Ropsten:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message:
          "Pending Gnosis Safe transactions are not available on the Ropsten testnet.",
      });
    case ethereum.Environment.Rinkeby:
      uri = "https://safe-transaction.rinkeby.gnosis.io";
      break;
    case ethereum.Environment.Mainnet:
      uri = "https://safe-transaction.gnosis.io";
      break;
  }

  return new SafeServiceClient(uri);
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

  const safeServiceClient = createSafeServiceClient();

  const encodedProjectUrn = ethers.utils.zeroPad(
    urn.parseIdentitySha1(projectUrn),
    32
  );
  const encodedCommitHash = multihash.encode(
    ethers.utils.arrayify(`0x${commitHash}`),
    "sha1"
  );

  const orgContract = new ethers.Contract(checksummedGnosisSafeAddress, orgAbi);

  const orgContractInstance = await orgContract.populateTransaction.anchor(
    encodedProjectUrn,
    encodedCommitHash,
    ethers.constants.Zero
  );

  const txData = orgContractInstance.data;
  if (!txData) {
    throw new error.Error({
      code: error.Code.OrgCreateCouldNotGenerateTx,
      message: "Could not generate transaction",
    });
  }

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

  await safeServiceClient.proposeTransaction(
    checksummedGnosisSafeAddress,
    transaction.data,
    safeTxHash,
    signature
  );

  notification.info({
    message:
      "Your anchored project will appear once the quorum of members have confirmed the transaction",
    showIcon: true,
    actions: [
      {
        label: "View on Gnosis Safe",
        handler: () => {
          openOnGnosisSafe(gnosisSafeAddress, "transactions");
        },
      },
    ],
  });

  router.push({ type: "org", address: orgAddress, activeTab: "projects" });
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
  const response: TransactionResponse = await orgFactory.createOrg([owner], 1);

  transaction.add(transaction.createOrg(response));
  notification.info({
    message: "Org creation transaction confirmed, your org will appear shortly",
    showIcon: true,
  });

  const receipt: TransactionReceipt =
    await walletStore.provider.waitForTransaction(response.hash);

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
    throw new error.Error({
      code: error.Code.OrgCreateNotFoundInInterfaceLogs,
      message: "Org not found in interface logs",
    });
  }

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

export const orgSidebarStore = svelteStore.writable<Org[]>([]);

export const fetchOrgs = async (): Promise<void> => {
  const walletStore = svelteStore.get(wallet.store);
  const w = svelteStore.get(walletStore);

  if (w.status !== wallet.Status.Connected) {
    throw new error.Error({
      code: error.Code.OrgFetchOrgsCalledWithNoWallet,
      message: "Tried to call fetchOrgs while the wallet wasn't connected",
    });
  }

  const orgs = await getOrgs(w.connected.account.address);
  orgSidebarStore.set(orgs);
};

// Information about an org and the safe that controls it.
interface OrgWithSafe {
  orgAddress: string;
  gnosisSafeAddress: string;
  members: Member[];
  threshold: number;
}

export const fetchOrg = async (orgAddress: string): Promise<OrgWithSafe> => {
  const walletStore = svelteStore.get(wallet.store);
  const gnosisSafeAddress = await fetchGnosisSafeAddr(
    orgAddress,
    walletStore.provider
  );
  const { members, threshold } = await getGnosisSafeMembers(gnosisSafeAddress);
  return { orgAddress, gnosisSafeAddress, members, threshold };
};

// Return all anchors for the org where the anchoring transactions are
// still pending
const fetchPendingAnchors = async (
  org: OrgWithSafe
): Promise<project.PendingAnchor[]> => {
  const checksummedGnosisSafeAddress = ethers.utils.getAddress(
    org.gnosisSafeAddress
  );

  const safeServiceClient = createSafeServiceClient();

  const txs = (
    await safeServiceClient.getPendingTransactions(checksummedGnosisSafeAddress)
  ).results;

  const isAnchor = (
    anchor: project.PendingAnchor | undefined
  ): anchor is project.PendingAnchor => !!anchor;

  const pendingAnchors = txs
    .map(tx => {
      if (!tx.data) {
        return;
      }
      const iface = new ethers.utils.Interface(orgAbi);
      const parsedTx = iface.parseTransaction({ data: tx.data });

      if (parsedTx.name === "anchor") {
        const [encodedProjectUrn, encodedCommitHash] = parsedTx.args;
        const projectId = urn.identitySha1Urn(
          ethers.utils.arrayify(`0x${encodedProjectUrn.slice(26)}`)
        );
        const byteArray = ethers.utils.arrayify(encodedCommitHash);
        const decodedMultihash = multihash.decode(byteArray);
        const decodedCommitHash = ethers.utils
          .hexlify(decodedMultihash.digest)
          .replace(/^0x/, "");
        const anchor: project.Anchor = {
          type: "pending",
          projectId,
          commitHash: decodedCommitHash,
          threshold: org.threshold,
          orgAddress: org.orgAddress,
          confirmations: tx.confirmations ? tx.confirmations.length : 0,
        };
        return anchor;
      }
    })
    .filter<project.PendingAnchor>(isAnchor);

  return pendingAnchors;
};

// Return project information for all anchors of an org. If the project
// of an anchor is not replicated by radicle link we include it in
// `unresolvedAnchors`.
//
// Includes anchors from transactions that have not been confirmed yet.
export const resolveProjectAnchors = async (
  org: OrgWithSafe
): Promise<{
  anchoredProjects: project.Project[];
  unresolvedAnchors: project.Anchor[];
}> => {
  const pendingAnchors = await fetchPendingAnchors(org);
  const confirmedAnchors = await getOrgProjectAnchors(org.orgAddress);
  const anchors: project.Anchor[] = [...pendingAnchors, ...confirmedAnchors];

  const anchoredProjects: project.Project[] = [];
  const unresolvedAnchors: project.Anchor[] = [];

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

  // Show pending projects first.
  anchoredProjects.sort((a, b) => {
    if (!a.anchor || !b.anchor) {
      return 0;
    }

    if (a.anchor.type === "pending" && b.anchor.type === "pending") {
      return 0;
    } else if (a.anchor.type === "pending" && b.anchor.type === "confirmed") {
      return -1;
    } else {
      return 1;
    }
  });

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
  modal.toggle(ModalAnchorProject, () => {}, {
    projects,
    orgAddress,
    gnosisSafeAddress,
  });
};
