// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as lodash from "lodash";
import { OperationType } from "@gnosis.pm/safe-core-sdk-types";

import type { Org } from "./org/theGraphApi";
import type * as identity from "ui/src/proxy/identity";
import type * as project from "ui/src/project";

import * as Safe from "./org/safe";
import * as Contract from "./org/contract";

import { claimsAddress, ClaimsContract } from "./attestation/contract";
import { identitySha1Urn } from "ui/src/urn";
import { isCypressTestEnv } from "ui/src/config";
import { memoizeLru } from "ui/src/memoizeLru";
import { sleep } from "ui/src/sleep";
import { unreachable } from "ui/src/unreachable";

import * as ensRegistrar from "./org/ensRegistrar";
import * as ensResolver from "./org/ensResolver";
import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as graph from "./org/theGraphApi";
import * as ipc from "ui/src/ipc";
import * as modal from "ui/src/modal";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as notification from "ui/src/notification";
import * as proxy from "ui/src/proxy";
import * as router from "ui/src/router";
import * as svelteStore from "ui/src/svelteStore";
import * as transaction from "./transaction";
import * as wallet from "ui/src/wallet";

import AnchorProjectModal from "ui/App/OrgScreen/AnchorProjectModal.svelte";
import ConfigureEnsModal from "ui/App/OrgScreen/ConfigureEnsModal.svelte";

export type { Org };

const orgPollExecutor = mutexExecutor.create();

// Update the org data for the sidebar store every `pollInterval` milliseconds,
// defaults to every 5 minutes.
//
// When encountering a 502 or 503 response we don’t show an error
// immediately but retry again.
const pollOrgListForever = async (
  pollInterval: number = 5 * 60 * 1000
): Promise<void> => {
  await orgPollExecutor.run(async abortSignal => {
    let showError = true;
    let remainingRetriesUnavailable = 20;

    for (;;) {
      if (abortSignal.aborted) {
        return;
      }
      const walletStore = svelteStore.get(wallet.store);

      await svelteStore.waitUntil(
        walletStore,
        w => w.status === wallet.Status.Connected
      );

      await fetchOrgs().then(
        () => {
          remainingRetriesUnavailable = 20;
          showError = true;
        },
        err => {
          if (
            graph.isUnavailableError(err) &&
            remainingRetriesUnavailable > 0
          ) {
            remainingRetriesUnavailable -= 1;
            console.warn(err);
            return;
          }
          // We only show the first error that is thrown by
          // `fetchOrgs()`. If the function keeps throwing errors we
          // don’t show them. We reset this behavior after the fetch is
          // successful.
          if (showError) {
            error.show(
              new error.Error({
                code: error.Code.OrgFetchFailed,
                message: `Failed to fetch org data`,
                source: err,
              })
            );
            showError = false;
          }
        }
      );

      await sleep(pollInterval);
    }
  });
};

// Start a background task that continously updates the org data for
// the sidebar.
export function initialize(): void {
  if (!isCypressTestEnv) {
    pollOrgListForever();
    const walletStore = svelteStore.get(wallet.store);
    walletStore.subscribe(state => {
      if (state.status === wallet.Status.Connected) {
        orgSidebarStore.set({ type: "initial" });
        fetchOrgs();
      }
    });
  }
}

export function openOnGnosisSafe(
  gnosisSafeAddress: string,
  view: "transactions" | "settings"
): void {
  ipc.openUrl(gnosisSafeWebAppUrl(gnosisSafeAddress, view));
}

export function gnosisSafeWebAppUrl(
  gnosisSafeAddress: string,
  view: "transactions" | "settings"
): string {
  return Safe.appUrl(
    svelteStore.get(wallet.store).environment,
    gnosisSafeAddress,
    view
  );
}

export const openOnEtherscan = (query: string): void => {
  ipc.openUrl(
    ethereum.etherscanUrl(svelteStore.get(wallet.store).environment, query)
  );
};

export const etherscanUrl = (query: string): string => {
  return ethereum.etherscanUrl(
    svelteStore.get(wallet.store).environment,
    query
  );
};

export async function anchorProjectWithGnosis(
  orgAddress: string,
  safeAddress: string,
  projectUrn: string,
  commitHash: string
): Promise<void> {
  const walletStore = svelteStore.get(wallet.store);
  const txData = await Contract.generateAnchorProjectTxData(
    projectUrn,
    commitHash
  );

  const confirmNotification = notification.info({
    message:
      "Waiting for you to sign the anchor transaction in your connected wallet",
    showIcon: true,
    persist: true,
  });

  try {
    await Safe.signAndProposeTransaction(walletStore, safeAddress, {
      to: orgAddress,
      value: "0",
      data: txData,
      operation: OperationType.Call,
    });
  } finally {
    confirmNotification.remove();
  }

  notification.info({
    message:
      "Your anchored project will appear once the quorum of members have confirmed the transaction",
    showIcon: true,
    persist: true,
    actions: [
      {
        label: "View on Gnosis Safe",
        handler: () => {
          openOnGnosisSafe(safeAddress, "transactions");
        },
      },
      {
        label: "Dismiss",
        handler: () => {},
      },
    ],
  });

  router.push({
    type: "org",
    params: { address: orgAddress, view: "projects" },
  });
}

export async function anchorProjectWithWallet(
  orgAddress: string,
  projectUrn: string,
  commitHash: string
): Promise<void> {
  const walletStore = svelteStore.get(wallet.store);

  const confirmNotification = notification.info({
    message:
      "Waiting for you to confirm the anchor transaction in your connected wallet",
    showIcon: true,
    persist: true,
  });

  let response: Contract.TransactionResponse;
  try {
    response = await Contract.submitSingleSigAnchor(
      projectUrn,
      commitHash,
      orgAddress,
      walletStore.signer
    );
  } finally {
    confirmNotification.remove();
  }

  transaction.add(transaction.anchorProject(response));

  notification.info({
    message:
      "Your anchored project will appear once the transaction has been included",
    showIcon: true,
    persist: true,
    actions: [
      {
        label: "View on Etherscan",
        handler: () => {
          openOnEtherscan(response.hash);
        },
      },
      {
        label: "Dismiss",
        handler: () => {},
      },
    ],
  });
}

// Holds the number of pending org creation transactions
export const pendingOrgs = svelteStore.writable<number>(0);

// Create an org contract that is controlled by `owner`.
//
// If `isMultiSig` is true a Gnosis Safe contract is created that acts
// as the orgs owner and has `owner` as a single member.
export async function createOrg(
  owner: string,
  isMultiSig: boolean
): Promise<void> {
  const walletStore = svelteStore.get(wallet.store);
  const confirmNotification = notification.info({
    message:
      "Waiting for you to confirm the org creation transaction in your connected wallet",
    showIcon: true,
    persist: true,
  });

  let response;
  try {
    response = await Contract.submitCreateOrgTx(
      walletStore.environment,
      owner,
      walletStore.signer,
      isMultiSig
    );
  } finally {
    confirmNotification.remove();
  }
  pendingOrgs.update(x => x + 1);
  // Poll org list every 15 seconds.
  pollOrgListForever(15 * 1000);

  transaction.add(transaction.createOrg(response));
  const creationNotification = notification.info({
    message: "Org creation transaction confirmed, your org will appear shortly",
    showIcon: true,
    persist: true,
  });

  const receipt = await walletStore.provider.waitForTransaction(response.hash);
  const orgAddress = Contract.parseOrgCreatedReceipt(receipt);

  await svelteStore.waitUntil(orgSidebarStore, store => {
    if (store.type === "initial") {
      return false;
    } else {
      return store.orgs.some(org => org.id === orgAddress);
    }
  });
  creationNotification.remove();
  pendingOrgs.update(x => x - 1);
  // Reset org list polling to default interval.
  pollOrgListForever();

  notification.info({
    message: `Org ${orgAddress} has been created`,
    showIcon: true,
    persist: true,
    actions: [
      {
        label: "Go to org",
        handler: () => {
          router.push({
            type: "org",
            params: {
              address: orgAddress,
              view: "projects",
            },
          });
        },
      },
    ],
  });
}

type OrgSidebarStore =
  | { type: "initial" }
  | { type: "fetched"; orgs: Org[] }
  | { type: "resolved"; orgs: Org[] };

export const orgSidebarStore = svelteStore.writable<OrgSidebarStore>({
  type: "initial",
});
const fetchOrgsExecutor = mutexExecutor.create();
const resolveOrgsExecutor = mutexExecutor.create();

export async function fetchOrgs(): Promise<void> {
  const sortedOrgs = await fetchOrgsExecutor.run(async () => {
    const walletStore = svelteStore.get(wallet.store);
    const wallet_ = svelteStore.get(walletStore);

    if (wallet_.status !== wallet.Status.Connected) {
      throw new error.Error({
        code: error.Code.OrgFetchOrgsCalledWithNoWallet,
        message: "Tried to call fetchOrgs while the wallet wasn't connected",
      });
    }

    const walletAddress = wallet_.connected.address;

    const gnosisSafeWallets = await Safe.getSafesByOwner(
      walletStore.environment,
      walletAddress
    );
    return lodash.sortBy(
      await graph.getOrgs(walletAddress, gnosisSafeWallets),
      org => org.timestamp
    );
  });

  if (sortedOrgs) {
    orgSidebarStore.set({ type: "fetched", orgs: sortedOrgs });

    const resolvedOrgs = await resolveOrgsExecutor.run(async () => {
      return await Promise.all(
        sortedOrgs.map(async org => {
          const registration = await ensResolver.getCachedRegistrationByAddress(
            org.id
          );
          if (registration) {
            org.registration = registration;
          }
          return org;
        })
      );
    });

    if (resolvedOrgs) {
      orgSidebarStore.set({ type: "resolved", orgs: resolvedOrgs });
    }
  }
}

// Owner of an org that controlls the interaction with the org
// contract. Maybe a simple wallet address that is controlled by one
// private key or a Gnosis Safe.
type Owner = { type: "wallet"; address: string } | GnosisSafeOwner;

interface GnosisSafeOwner {
  type: "gnosis-safe";
  address: string;
  members: Member[];
  threshold: number;
}

// Determines the owner of an org at the given address.
export async function getOwner(orgAddress: string): Promise<Owner> {
  const walletStore = svelteStore.get(wallet.store);
  const address = await Contract.getOwner(orgAddress, walletStore.provider);
  const ownerCode = await walletStore.provider.getCode(address);
  // We’re not really checking that the address is the Gnosis Safe
  // contract. We’re just checking if it is _a_ contract.
  const isSafe = ownerCode !== "0x";
  if (isSafe) {
    const { members, threshold } = await fetchMembers(walletStore, address);
    return { type: "gnosis-safe", address, members, threshold };
  } else {
    return { type: "wallet", address };
  }
}

interface OrgMembers {
  threshold: number;
  members: Member[];
}

export interface Member {
  ethereumAddress: string;
  identity: identity.RemoteIdentity | undefined;
}

export async function fetchMembers(
  wallet: wallet.Wallet,
  gnosisSafeAddress: string
): Promise<OrgMembers> {
  const response = await Safe.getMetadata(
    wallet.environment,
    gnosisSafeAddress
  );

  const contract = new ClaimsContract(
    wallet.signer,
    claimsAddress(wallet.environment)
  );

  const members = (
    await Promise.all(
      response.members.map(async ethereumAddress => {
        const identity = await getClaimedIdentity(contract, ethereumAddress);
        return { ethereumAddress, identity };
      })
    )
  ).sort((a, b) => {
    // Show members with claimed identities first.
    if (!a.identity && b.identity) {
      return 1;
    }
    if (a.identity && !b.identity) {
      return -1;
    }
    return 0;
  });

  return {
    threshold: response.threshold,
    members,
  };
}

// Return all anchoring transactions that are pending for the given
// Gnosis safe.
async function fetchPendingAnchors(
  orgAddress: string,
  gnosis: GnosisSafeOwner,
  registration?: ensResolver.Registration
): Promise<project.PendingAnchor[]> {
  const walletStore = svelteStore.get(wallet.store);
  const txs = await Safe.getPendingTransactions(
    walletStore.environment,
    gnosis.address
  );
  const isAnchor = (
    anchor: project.PendingAnchor | undefined
  ): anchor is project.PendingAnchor => !!anchor;

  const pendingAnchors = txs
    .map(tx => {
      if (!tx.data) {
        return;
      }

      const anchorData = Contract.parseAnchorTx(tx.data);

      if (anchorData) {
        const anchor: project.Anchor = {
          type: "pending",
          projectId: anchorData.projectId,
          commitHash: anchorData.commitHash,
          threshold: gnosis.threshold,
          orgAddress: orgAddress,
          confirmations: tx.confirmations ? tx.confirmations.length : 0,
          timestamp: Date.parse(tx.submissionDate),
          registration,
        };
        return anchor;
      }
    })
    .filter<project.PendingAnchor>(isAnchor);

  return pendingAnchors;
}

export interface OrgAnchors {
  pendingResolved: project.Project[];
  confirmedResolved: project.Project[];
  pendingUnresolved: project.Anchor[];
  confirmedUnresolved: project.Anchor[];
}

// Return project information for all anchors of an org. If the project
// of an anchor is not replicated by radicle link we include it in
// `unresolvedAnchors`.
//
// Includes anchors from transactions that have not been confirmed yet.
export async function resolveProjectAnchors(
  orgAddress: string,
  owner: Owner,
  registration?: ensResolver.Registration
): Promise<OrgAnchors> {
  let pendingAnchors: project.Anchor[];
  if (owner.type === "wallet") {
    pendingAnchors = [];
  } else if (owner.type === "gnosis-safe") {
    pendingAnchors = await fetchPendingAnchors(orgAddress, owner, registration);
  } else {
    pendingAnchors = unreachable(owner);
  }
  const confirmedAnchors = await graph.getOrgProjectAnchors(
    orgAddress,
    registration
  );
  const anchors: project.Anchor[] = [...pendingAnchors, ...confirmedAnchors];

  const anchoredProjects: project.Project[] = [];
  const unresolvedAnchors: project.Anchor[] = [];

  await Promise.all(
    anchors.map(async anchor => {
      try {
        const project = await proxy.client.project.get(anchor.projectId);
        anchoredProjects.push({ ...project, anchor });
      } catch (_error: unknown) {
        // TODO: only catch when backend can't find project, reraise other errors
        unresolvedAnchors.push(anchor);
      }
    })
  );

  return {
    pendingResolved: anchoredProjects
      .filter(p => p.anchor && p.anchor.type === "pending")
      .sort((a, b) => {
        if (a.anchor && b.anchor) {
          return a.anchor.timestamp - b.anchor.timestamp;
        } else {
          return 0;
        }
      }),
    confirmedResolved: anchoredProjects
      .filter(p => p.anchor && p.anchor.type === "confirmed")
      .sort((a, b) => {
        if (a.anchor && b.anchor) {
          return a.anchor.timestamp - b.anchor.timestamp;
        } else {
          return 0;
        }
      }),
    pendingUnresolved: unresolvedAnchors
      .filter(a => a.type === "pending")
      .sort((a, b) => {
        return a.timestamp - b.timestamp;
      }),
    confirmedUnresolved: unresolvedAnchors
      .filter(a => a.type === "confirmed")
      .sort((a, b) => {
        return a.timestamp - b.timestamp;
      }),
  };
}

export interface ProjectOption {
  title: string;
  value: string;
}

export async function openAnchorProjectModal(
  orgAddress: string,
  ownerAddress: string,
  isMultiSig: boolean
): Promise<void> {
  const [tracked, contributed] = await Promise.all([
    proxy.client.project.listTracked(),
    proxy.client.project.listContributed(),
  ]);
  const allProjects = [...tracked, ...contributed];

  const projects: ProjectOption[] = allProjects.map(project => {
    return { title: project.metadata.name, value: project.urn };
  });

  modal.toggle(AnchorProjectModal, () => {}, {
    projects,
    orgAddress,
    ownerAddress,
    isMultiSig,
  });
}

export async function getProjectCount(): Promise<number> {
  const [tracked, contributed] = await Promise.all([
    proxy.client.project.listTracked(),
    proxy.client.project.listContributed(),
  ]);

  return tracked.length + contributed.length;
}

async function getClaimedIdentity(
  contract: ClaimsContract,
  address: string
): Promise<identity.RemoteIdentity | undefined> {
  const radicleIdBytes = await contract.getClaimed(address);
  if (!radicleIdBytes) {
    return undefined;
  }
  const urn = identitySha1Urn(radicleIdBytes);
  let identity;
  try {
    identity = await proxy.client.remoteIdentityGet(urn);
  } catch (error: unknown) {
    if (error instanceof proxy.ResponseError && error.response.status === 404) {
      return undefined;
    }
    throw error;
  }
  // Assert that the identity claims the ethereum address
  const claimed = identity.metadata.ethereum?.address.toLowerCase();
  if (claimed !== address.toLowerCase()) {
    return undefined;
  }
  return identity;
}

// Returns true if a given org at the given address is owned by a Gnosis safe.
export const isMultiSig = memoizeLru(
  async (address: string): Promise<boolean> => {
    const walletStore = svelteStore.get(wallet.store);
    const code = await walletStore.provider.getCode(address);
    // We’re not really checking that the address is the Gnosis Safe
    // contract. We’re just checking if it is _a_ contract.
    return code !== "0x";
  },
  address => address,
  { max: 1000 }
);

export async function setNameSingleSig(
  name: string,
  orgAddress: string
): Promise<Contract.TransactionResponse> {
  const walletStore = svelteStore.get(wallet.store);

  const ensAddress = ethereum.ensAddress(walletStore.environment);

  return Contract.setName(walletStore.signer, orgAddress, name, ensAddress);
}

// Propose a transaction to change the wallet name to the Gnosis safe.
export async function proposeSetNameChange(
  name: string,
  orgAddress: string,
  ownerAddress: string
): Promise<void> {
  const walletStore = svelteStore.get(wallet.store);

  const ensAddress = ethereum.ensAddress(walletStore.environment);

  const data = await Contract.populateSetNameTransaction(
    orgAddress,
    name,
    ensAddress
  );

  const safeTx = {
    to: orgAddress,
    value: "0",
    data,
    operation: OperationType.Call,
  };
  await Safe.signAndProposeTransaction(walletStore, ownerAddress, safeTx);
}

export async function openEnsConfiguration(
  orgAddress: string,
  registration?: ensResolver.Registration,
  safeAddress?: string
): Promise<void> {
  const fee = await ensRegistrar.getFee();
  modal.show(ConfigureEnsModal, () => {}, {
    safeAddress,
    orgAddress,
    registration,
    fee,
  });
}
