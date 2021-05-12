import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core";

// TODO(rudolfs): make this respect the network selector in settings
const gnosisSubgraphClient = new ApolloClient({
  uri: "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe-rinkeby",
  cache: new InMemoryCache(),
});

// TODO(rudolfs): make this respect the network selector in settings
const orgsSubgraphClient = new ApolloClient({
  uri: "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-rinkeby",
  cache: new InMemoryCache(),
});

type GnosisSafeWallet = {
  id: string;
  owners: [string];
};

export type Org = {
  id: string;
  owner: string;
  creator: string;
};

export type Member = {
  id: string;
};

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

export const getGnosisSafeMembers = async (
  walletAddress: string
): Promise<[Member]> => {
  const members: [Member] = (
    await gnosisSubgraphClient.query({
      query: gql`
        query GetGnosisSafeWallets($id: String!) {
          wallets(where: { id: $id }) {
            owners
          }
        }
      `,
      variables: { id: walletAddress },
      fetchPolicy: "no-cache",
    })
  ).data.wallets[0].owners;
  return members;
};
