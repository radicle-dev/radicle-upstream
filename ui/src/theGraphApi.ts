import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core";

// TODO(rudolfs): make this respect the network selector in settings
const gnosisSubgraphClient = new ApolloClient({
  uri: "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe-ropsten",
  cache: new InMemoryCache(),
});

// TODO(rudolfs): make this respect the network selector in settings
const orgsSubgraphClient = new ApolloClient({
  uri: "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-ropsten",
  cache: new InMemoryCache(),
});

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
  return await gnosisSubgraphClient.query({
    query: gql`
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
    await orgsSubgraphClient.query({
      query: gql`
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
    await gnosisSubgraphClient.query({
      query: gql`
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
