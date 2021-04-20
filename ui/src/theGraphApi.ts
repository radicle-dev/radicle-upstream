import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core";

const gnosisSubgraphClient = new ApolloClient({
  uri:
    "https://api.thegraph.com/subgraphs/name/radicle-dev/gnosis-safe-ropsten",
  cache: new InMemoryCache(),
});

const orgsSubgraphClient = new ApolloClient({
  uri:
    "https://api.thegraph.com/subgraphs/name/radicle-dev/radicle-orgs-ropsten",
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
    })
  ).data.orgs;

  return orgs;
};
