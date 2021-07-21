// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as apolloCore from "@apollo/client/core";
import * as ethers from "ethers";
import * as multihash from "multihashes";
import * as svelteStore from "svelte/store";

import type * as project from "ui/src/project";

import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as urn from "ui/src/urn";
import * as wallet from "ui/src/wallet";
import type { Registration } from "./ensResolver";

function createApolloClient(uri: string): apolloCore.ApolloClient<unknown> {
  return new apolloCore.ApolloClient({
    uri,
    cache: new apolloCore.InMemoryCache(),
    defaultOptions: {
      query: {
        fetchPolicy: "no-cache",
      },
    },
  });
}

function gnosisSubgraphClient(): apolloCore.ApolloClient<unknown> {
  const walletStore = svelteStore.get(wallet.store);
  let uri;
  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Orgs is not available on the Local testnet.",
      });
    case ethereum.Environment.Rinkeby:
      uri =
        "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe-rinkeby";
      break;
    case ethereum.Environment.Mainnet:
      uri = "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe";
      break;
  }

  return createApolloClient(uri);
}

function orgsSubgraphClient() {
  const walletStore = svelteStore.get(wallet.store);
  let uri;
  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Orgs is not available on the Local testnet.",
      });
    case ethereum.Environment.Rinkeby:
      uri =
        "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-rinkeby";
      break;
    case ethereum.Environment.Mainnet:
      uri = "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs";
      break;
  }
  return createApolloClient(uri);
}

interface GnosisSafeWallet {
  id: string;
  owners: string[];
}

export interface Org {
  id: string;
  owner: string;
  registration?: Registration;
  creator: string;
  timestamp: number;
}

async function getGnosisSafeWallets(walletOwnerAddress: string) {
  return await gnosisSubgraphClient().query({
    query: apolloCore.gql`
      query GetGnosisSafeWallets($owners: [String!]!) {
        wallets(where: { owners_contains: $owners }) {
          id
          owners
        }
      }
    `,
    variables: { owners: [walletOwnerAddress] },
  });
}

export async function getOrgs(walletOwnerAddress: string): Promise<Org[]> {
  const gnosisSafeWallets: [GnosisSafeWallet] = (
    await getGnosisSafeWallets(walletOwnerAddress)
  ).data.wallets;

  const multiSigOwners = gnosisSafeWallets.map(owner => owner.id);

  const orgsResponse = await orgsSubgraphClient().query<{
    orgs: Array<{
      id: string;
      owner: string;
      creator: string;
      // This is a UNIX seconds timestamp formatted as a string
      timestamp: string;
    }>;
  }>({
    query: apolloCore.gql`
        query GetOrgs($owners: [String!]!) {
          orgs(where: { owner_in: $owners }) {
            id
            owner
            creator
            timestamp
          }
        }
      `,
    variables: { owners: [walletOwnerAddress, ...multiSigOwners] },
  });

  return orgsResponse.data.orgs.map(org => ({
    ...org,
    timestamp: Number.parseInt(org.timestamp),
  }));
}

export interface MemberResponse {
  threshold: number;
  members: string[];
}

export async function getGnosisSafeMembers(
  walletAddress: string
): Promise<MemberResponse> {
  const response = (
    await gnosisSubgraphClient().query({
      query: apolloCore.gql`
        query GetGnosisSafeWallets($id: String!) {
          wallets(where: { id: $id }) {
            owners
            threshold
          }
        }
      `,
      // The Gnosis index is case-sensitive and normalised to lower-case safe
      // IDs.
      variables: { id: walletAddress.toLowerCase() },
    })
  ).data.wallets[0];

  return { members: response.owners, threshold: parseInt(response.threshold) };
}

export async function getOrgProjectAnchors(
  orgAddress: string
): Promise<project.Anchor[]> {
  const response = (
    await orgsSubgraphClient().query({
      query: apolloCore.gql`
        query GetOrgAnchoredProjects($orgAddress: String!) {
          projects(where: {org: $orgAddress}) {
            anchor {
              id
              objectId
              multihash
              timestamp
            }
          }
        }
      `,
      variables: { orgAddress },
    })
  ).data.projects;

  return response.map(
    (project: {
      anchor: {
        id: string;
        objectId: string;
        multihash: string;
        // This is a UNIX seconds timestamp formatted as a string
        timestamp: number;
      };
    }) => {
      const decodedProjectId = urn.identitySha1Urn(
        ethers.utils.arrayify(`0x${project.anchor.objectId.slice(26)}`)
      );

      const byteArray = ethers.utils.arrayify(project.anchor.multihash);
      const decodedMultihash = multihash.decode(byteArray);
      const decodedCommitHash = ethers.utils
        .hexlify(decodedMultihash.digest)
        .replace(/^0x/, "");
      const anchor: project.Anchor = {
        type: "confirmed",
        orgAddress,
        transactionId: project.anchor.id,
        projectId: decodedProjectId,
        commitHash: decodedCommitHash,
        timestamp: project.anchor.timestamp,
      };

      return anchor;
    }
  );
}

// Returns `true` if `err` is a 502 HTTP response error thrown by
// requests to the Graph.
export function is502Error(err: unknown): boolean {
  return (
    err instanceof apolloCore.ApolloError &&
    err.networkError !== null &&
    "statusCode" in err.networkError &&
    err.networkError.statusCode === 502
  );
}
