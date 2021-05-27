import * as apolloCore from "@apollo/client/core";
import * as ethers from "ethers";
import * as svelteStore from "svelte/store";

import * as wallet from "ui/src/wallet";
import * as ethereum from "ui/src/ethereum";
import * as error from "ui/src/error";
import * as urn from "ui/src/urn";

const gnosisClientEndpoint = (): string => {
  const walletStore = svelteStore.get(wallet.store);
  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      error.show(
        new error.Error({
          code: error.Code.FeatureNotAvailableForGivenNetwork,
          message: "Orgs not available on the Local testnet.",
        })
      );
      return "";
    case ethereum.Environment.Ropsten:
      return "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe-ropsten";
    case ethereum.Environment.Rinkeby:
      return "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe-rinkeby";
  }
};

const gnosisSubgraphClient = () => {
  return new apolloCore.ApolloClient({
    uri: gnosisClientEndpoint(),
    cache: new apolloCore.InMemoryCache(),
  });
};

const orgClientEndpoint = (): string => {
  const walletStore = svelteStore.get(wallet.store);
  switch (walletStore.environment) {
    case ethereum.Environment.Local:
      error.show(
        new error.Error({
          code: error.Code.FeatureNotAvailableForGivenNetwork,
          message: "Orgs not available on the Local testnet.",
        })
      );
      return "";
    case ethereum.Environment.Ropsten:
      return "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-ropsten";
    case ethereum.Environment.Rinkeby:
      return "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-rinkeby";
  }
};

const orgsSubgraphClient = () => {
  return new apolloCore.ApolloClient({
    uri: orgClientEndpoint(),
    cache: new apolloCore.InMemoryCache(),
  });
};

interface GnosisSafeWallet {
  id: string;
  owners: [string];
}

export interface Org {
  id: string;
  owner: string;
  creator: string;
}

const getGnosisSafeWallets = async (walletOwnerAddress: string) => {
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
    fetchPolicy: "no-cache",
  });
};

export const getOrgs = async (walletOwnerAddress: string): Promise<[Org]> => {
  const gnosisSafeWallets: [GnosisSafeWallet] = (
    await getGnosisSafeWallets(walletOwnerAddress)
  ).data.wallets;

  const orgs: [Org] = (
    await orgsSubgraphClient().query({
      query: apolloCore.gql`
        query GetOrgs($owners: [String!]!) {
          orgs(where: { owner_in: $owners }) {
            id
            owner
            creator
          }
        }
      `,
      variables: { owners: gnosisSafeWallets.map(owner => owner.id) },
      fetchPolicy: "no-cache",
    })
  ).data.orgs;

  return orgs;
};

interface Member {
  id: string;
}

export interface MemberResponse {
  threshold: number;
  members: Member[];
}

export const getGnosisSafeMembers = async (
  walletAddress: string
): Promise<MemberResponse> => {
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
      variables: { id: walletAddress },
      fetchPolicy: "no-cache",
    })
  ).data.wallets[0];

  return { members: response.owners, threshold: response.threshold };
};

export interface ProjectAnchor {
  id: string;
  projectId: string;
  commitSha: string;
}

export const getOrgProjectAnchors = async (
  orgAddress: string
): Promise<ProjectAnchor[]> => {
  const response = (
    await orgsSubgraphClient().query({
      query: apolloCore.gql`
        query GetOrgAnchors($orgAddress: String!) {
          anchors(where: {org: $orgAddress, stateType: 0, stateHashFormat: 0 }) {
            id
            objectId
            stateHash
          }
        }
      `,
      variables: { orgAddress },
      fetchPolicy: "no-cache",
    })
  ).data.anchors;

  return response.map(
    (anchor: { id: string; stateHash: string; objectId: string }) => {
      return {
        id: anchor.id,
        projectId: urn.identitySha1Urn(
          ethers.utils.arrayify(`0x${anchor.objectId.slice(26)}`)
        ),
        commitSha: anchor.stateHash.slice(26, 66),
      };
    }
  );
};
