// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Registration } from "./ensResolver";
import type * as ensResolver from "ui/src/org/ensResolver";
import type * as project from "ui/src/project";

import lodash from "lodash";

import * as apolloCore from "@apollo/client/core";
import * as svelteStore from "svelte/store";
import * as ethers from "ethers";

import { memoizeLru } from "ui/src/memoizeLru";

import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as wallet from "ui/src/wallet";
import * as contract from "./contract";

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

function orgsSubgraphClient(): apolloCore.ApolloClient<unknown> {
  const walletStore = svelteStore.get(wallet.store);
  let uri;
  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      throw new error.Error({
        message:
          "orgsSubgraphClient() is not implemented for ethereum.Environment.Local",
      });
    case ethereum.Environment.Rinkeby:
      uri =
        "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-rinkeby";
      break;
    case ethereum.Environment.Mainnet:
      uri =
        "https://gateway.thegraph.com/api/1758a78ae257ad4906f9c638e4a68c19/subgraphs/id/0x2f0963e77ca6ac0c2dad1bf4147b6b40e0dd8728-0";
      break;
  }
  return createApolloClient(uri);
}

export interface Org {
  id: string;
  owner: string;
  registration?: Registration;
  creator: string;
  timestamp: number;
  projectCount?: number;
}

export interface Safe {
  id: string;
  owners: string[];
  threshold: number;
}

export async function getSafesByOwner(owner: string): Promise<Safe[]> {
  const safesResponse = await orgsSubgraphClient().query<{
    safes: Array<{
      // Safe address.
      id: string;
      // Safe owner addresses.
      owners: string[];
      // Number of signatures required for quorum.
      threshold: string;
    }>;
  }>({
    query: apolloCore.gql`
      query GetSafesByOwners($owners: [String!]!) {
        safes(where: { owners_contains: $owners }) {
          id
          owners
          threshold
        }
      }
      `,
    variables: { owners: [owner.toLowerCase()] },
  });

  return safesResponse.data.safes.map(safe => ({
    ...safe,
    threshold: Number(safe.threshold),
  }));
}

export const getSafeMetadata = memoizeLru(
  async (address: string): Promise<Safe> => {
    const safesResponse = await orgsSubgraphClient().query<{
      safe: {
        // Safe address.
        id: string;
        // Safe owner addresses.
        owners: string[];
        // Number of signatures required for quorum.
        threshold: string;
      };
    }>({
      query: apolloCore.gql`
      query GetSafe($address: ID!) {
        safe(id: $address) {
          id
          owners
          threshold
        }
      }
      `,
      variables: { address: address.toLowerCase() },
    });

    return {
      ...safesResponse.data.safe,
      threshold: Number(safesResponse.data.safe.threshold),
    };
  },
  safeAddress => safeAddress,
  { max: 1000, ttl: 15 * 60 * 1000 } // 15 minutes
);

export async function getOwnedOrgs(owners: string[]): Promise<Org[]> {
  const orgsResponse = await orgsSubgraphClient().query<{
    orgs: Array<{
      // Org address.
      id: string;
      // Owner address.
      owner: string;
      // Creator address.
      creator: string;
      // This is a UNIX seconds timestamp formatted as a string.
      timestamp: string;
    }>;
  }>({
    query: apolloCore.gql`
        query GetOrgs($owners: [String!]!) {
          orgs(where: { owner_in: $owners }, orderBy: timestamp, orderDirection: asc) {
            id
            owner
            creator
            timestamp
          }
        }
      `,
    variables: { owners },
  });

  return orgsResponse.data.orgs.map(org => ({
    ...org,
    timestamp: Number.parseInt(org.timestamp),
  }));
}

export async function getAllOrgs(): Promise<Org[]> {
  const orgsResponse = await orgsSubgraphClient().query<{
    orgs: Array<{
      // Org address.
      id: string;
      // Owner address.
      owner: string;
      // Creator address.
      creator: string;
      // This is a UNIX seconds timestamp formatted as a string.
      timestamp: string;
    }>;
  }>({
    // TODO: add pagination. By default the Graph limits queries to 100
    // results, we increase this limit to 1000 for now.
    query: apolloCore.gql`
    query GetOrgs {
      orgs(orderBy: timestamp, orderDirection: desc, first: 1000) {
        id
        owner
        creator
        timestamp
      }
    }
    `,
  });

  return orgsResponse.data.orgs.map(org => ({
    ...org,
    timestamp: Number.parseInt(org.timestamp),
  }));
}

export async function getOrgProjectAnchors(
  orgAddress: string,
  registration?: ensResolver.Registration
): Promise<project.Anchor[]> {
  const response = (
    await orgsSubgraphClient().query<{
      projects: Array<{
        anchor: {
          // Transaction ID.
          id: string;
          // Project ID.
          objectId: string;
          // Commit hash encoded as multihash.
          multihash: string;
          // This is a UNIX seconds timestamp formatted as a string.
          timestamp: string;
        };
      }>;
    }>({
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

  return response.map(project => {
    const anchor: project.Anchor = {
      type: "confirmed",
      orgAddress,
      transactionId: project.anchor.id,
      projectId: contract.decodeUrn(project.anchor.objectId),
      commitHash: contract.decodeSha1(project.anchor.multihash),
      timestamp: parseInt(project.anchor.timestamp),
      registration,
    };

    return anchor;
  });
}

export async function getProjectAnchors(
  projectId: string
): Promise<project.ConfirmedAnchor[]> {
  const response = (
    await orgsSubgraphClient().query<{
      anchors: Array<{
        // Transaction ID.
        id: string;
        org: {
          // Org address.
          id: string;
        };
        // Project ID.
        objectId: string;
        // Anchor object type. "0" stands for project commit hash.
        tag: string;
        // Anchor object encoded as multihash.
        multihash: string;
        // This is a UNIX seconds timestamp formatted as a string.
        timestamp: string;
      }>;
    }>({
      query: apolloCore.gql`
        query GetProjectAnchors($projectId: String!) {
          anchors(where: {objectId: $projectId}) {
            id
            org {
              id
            }
            objectId
            tag
            multihash
            timestamp
          }
        }
      `,
      variables: {
        projectId: ethers.utils.hexlify(contract.encodeUrn(projectId)),
      },
    })
  ).data.anchors;

  return response
    .filter(anchor => {
      return anchor.tag === "0";
    })
    .map(anchor => {
      return {
        type: "confirmed",
        transactionId: anchor.id,
        orgAddress: anchor.org.id,
        projectId: contract.decodeUrn(anchor.objectId),
        commitHash: contract.decodeSha1(anchor.multihash),
        timestamp: parseInt(anchor.timestamp),
      };
    });
}

// Returns `true` if `err` is a 502 or 503 HTTP response error thrown
// by requests to the Graph.
export function isUnavailableError(err: unknown): boolean {
  return (
    err instanceof apolloCore.ApolloError &&
    err.networkError !== null &&
    "statusCode" in err.networkError &&
    (err.networkError.statusCode === 502 || err.networkError.statusCode === 503)
  );
}

export async function getProjectCounts(
  orgIds: string[]
): Promise<Record<string, number | undefined>> {
  const response = (
    await orgsSubgraphClient().query<{
      projects: Array<{
        org: {
          // Org address.
          id: string;
        };
      }>;
    }>({
      query: apolloCore.gql`
        query GetProjectsOfOrgs($orgAddresses: [String!]!) {
          projects(where: {org_in: $orgAddresses}) {
            org {
              id
            }
          }
        }
      `,
      variables: {
        orgAddresses: orgIds.map(org => org.toLowerCase()),
      },
    })
  ).data.projects;

  return lodash.countBy(response, data => data.org.id);
}
