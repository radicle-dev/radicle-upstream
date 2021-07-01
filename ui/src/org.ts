// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as lodash from "lodash";
import { OperationType } from "@gnosis.pm/safe-core-sdk-types";

import type * as identity from "ui/src/proxy/identity";
import type * as project from "ui/src/project";

import * as Safe from "./org/safe";
import * as Contract from "./org/contract";

import * as error from "ui/src/error";
import * as ipc from "ui/src/ipc";
import * as modal from "ui/src/modal";
import * as notification from "ui/src/notification";
import * as proxy from "ui/src/proxy";
import * as router from "ui/src/router";
import * as svelteStore from "ui/src/svelteStore";
import * as transaction from "./transaction";
import * as wallet from "ui/src/wallet";
import { claimsAddress, ClaimsContract } from "./attestation/contract";
import type { Org, MemberResponse } from "./org/theGraphApi";
import * as graph from "./org/theGraphApi";
import { identitySha1Urn } from "ui/src/urn";
import { sleep } from "ui/src/sleep";

import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";

export type { MemberResponse, Org };

const ORG_POLL_INTERVAL_MS = 2000;

// Update the org data for the sidebar store every
// `ORG_POLL_INTERVAL_MS` milliseconds.
const updateOrgsForever = async (): Promise<never> => {
  let showError = true;
  let remainingRetries502 = 0;

  for (;;) {
    const walletStore = svelteStore.get(wallet.store);

    await svelteStore.waitUntil(
      walletStore,
      w => w.status === wallet.Status.Connected
    );

    await fetchOrgs().then(
      () => {
        remainingRetries502 = 20;
        showError = true;
      },
      err => {
        if (graph.is502Error(err) && remainingRetries502 > 0) {
          remainingRetries502 -= 1;
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

    await sleep(ORG_POLL_INTERVAL_MS);
  }
};

// Start a background task that continously updates the org data for
// the sidebar.
export function initialize(): void {
  updateOrgsForever();
}

export function openOnGnosisSafe(
  gnosisSafeAddress: string,
  view: "transactions" | "settings"
): void {
  ipc.openUrl(
    Safe.appUrl(
      svelteStore.get(wallet.store).environment,
      gnosisSafeAddress,
      view
    )
  );
}

export async function anchorProject(
  orgAddress: string,
  gnosisSafeAddress: string,
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
      "Waiting for you to confirm the anchor transaction in your connected wallet",
    showIcon: true,
    persist: true,
  });

  try {
    await Safe.signAndProposeTransaction(walletStore, gnosisSafeAddress, {
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
          openOnGnosisSafe(gnosisSafeAddress, "transactions");
        },
      },
      {
        label: "Dismiss",
        handler: () => {},
      },
    ],
  });

  router.push({ type: "org", address: orgAddress, activeTab: "projects" });
}

// Holds the number of pending org creation transactions
export const pendingOrgs = svelteStore.writable<number>(0);

export async function createOrg(owner: string): Promise<void> {
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
      walletStore.signer
    );
  } finally {
    confirmNotification.remove();
  }
  pendingOrgs.update(x => x + 1);

  transaction.add(transaction.createOrg(response));
  notification.info({
    message: "Org creation transaction confirmed, your org will appear shortly",
    showIcon: true,
  });

  const receipt = await walletStore.provider.waitForTransaction(response.hash);
  const orgAddress = Contract.parseOrgCreatedReceipt(receipt);

  await svelteStore.waitUntil(orgSidebarStore, orgs => {
    return orgs.some(org => org.id === orgAddress);
  });
  pendingOrgs.update(x => x - 1);

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
}

export const orgSidebarStore = svelteStore.writable<Org[]>([]);

async function fetchOrgs(): Promise<void> {
  const walletStore = svelteStore.get(wallet.store);
  const w = svelteStore.get(walletStore);

  if (w.status !== wallet.Status.Connected) {
    throw new error.Error({
      code: error.Code.OrgFetchOrgsCalledWithNoWallet,
      message: "Tried to call fetchOrgs while the wallet wasn't connected",
    });
  }

  const orgs = await graph.getOrgs(w.connected.account.address);
  const sortedOrgs = lodash.sortBy(orgs, org => org.timestamp);
  orgSidebarStore.set(sortedOrgs);
}

// Information about an org and the safe that controls it.
interface OrgWithSafe {
  orgAddress: string;
  gnosisSafeAddress: string;
  members: Member[];
  threshold: number;
}

export async function fetchOrg(orgAddress: string): Promise<OrgWithSafe> {
  const walletStore = svelteStore.get(wallet.store);
  const gnosisSafeAddress = await Contract.getOwner(
    orgAddress,
    walletStore.provider
  );
  const { members, threshold } = await fetchMembers(
    walletStore,
    gnosisSafeAddress
  );
  return { orgAddress, gnosisSafeAddress, members, threshold };
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
  const response: MemberResponse = await graph.getGnosisSafeMembers(
    gnosisSafeAddress
  );

  const contract = new ClaimsContract(
    wallet.signer,
    claimsAddress(wallet.environment)
  );

  const members = await Promise.all(
    response.members.map(async ethereumAddress => {
      const identity = await getClaimedIdentity(contract, ethereumAddress);
      return { ethereumAddress, identity };
    })
  );

  return {
    threshold: response.threshold,
    members,
  };
}

// Return all anchors for the org where the anchoring transactions are
// still pending
async function fetchPendingAnchors(
  org: OrgWithSafe
): Promise<project.PendingAnchor[]> {
  const walletStore = svelteStore.get(wallet.store);
  const txs = await Safe.getPendingTransactions(
    walletStore.environment,
    org.gnosisSafeAddress
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
          threshold: org.threshold,
          orgAddress: org.orgAddress,
          confirmations: tx.confirmations ? tx.confirmations.length : 0,
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
  org: OrgWithSafe
): Promise<OrgAnchors> {
  const pendingAnchors = await fetchPendingAnchors(org);
  const confirmedAnchors = await graph.getOrgProjectAnchors(org.orgAddress);
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

  return {
    pendingResolved: anchoredProjects.filter(
      p => p.anchor && p.anchor.type === "pending"
    ),
    confirmedResolved: anchoredProjects.filter(
      p => p.anchor && p.anchor.type === "confirmed"
    ),
    pendingUnresolved: unresolvedAnchors.filter(a => a.type === "pending"),
    confirmedUnresolved: unresolvedAnchors.filter(a => a.type === "confirmed"),
  };
}

export interface ProjectOption {
  title: string;
  value: string;
}

export async function openAnchorProjectModal(
  orgAddress: string,
  gnosisSafeAddress: string
): Promise<void> {
  const [tracked, contributed] = await Promise.all([
    proxy.client.project.listTracked(),
    proxy.client.project.listContributed(),
  ]);
  const allProjects = [...tracked, ...contributed];

  const projects: ProjectOption[] = allProjects.map(project => {
    return { title: project.metadata.name, value: project.urn };
  });

  modal.toggle(ModalAnchorProject, () => {}, {
    projects,
    orgAddress,
    gnosisSafeAddress,
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
  } catch (error) {
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
