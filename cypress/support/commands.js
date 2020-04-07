import ApolloClient from "apollo-boost";
import { gql } from "apollo-boost";

const client = new ApolloClient({ uri: "http://localhost:8080/control" });

Cypress.Commands.add("nukeCocoState", () => {
  console.log("Nuking CoCo state");
  client.mutate({
    mutation: gql`
      mutation {
        nukeCocoState
      }
    `
  });
});

Cypress.Commands.add("nukeRegistryState", () => {
  console.log("Nuking Registry state");
  client.mutate({
    mutation: gql`
      mutation {
        nukeRegistryState
      }
    `
  });
});

Cypress.Commands.add("nukeAllState", () => {
  console.log("Nuking CoCo and Registry state");
  client.mutate({
    mutation: gql`
      mutation {
        nukeCocoState
        nukeRegistryState
      }
    `
  });
});

Cypress.Commands.add(
  "createProjectWithFixture",
  (
    name = "Monadic",
    description = "Monadic is currently supporting radicle.",
    defaultBranch = "master"
  ) => {
    client.mutate({
      variables: {
        name: name,
        description: description,
        defaultBranch: defaultBranch
      },
      mutation: gql`
        mutation CreateProjectWithFixture(
          $name: String!
          $description: String!
          $defaultBranch: String!
        ) {
          createProjectWithFixture(
            metadata: {
              name: $name
              description: $description
              defaultBranch: $defaultBranch
            }
          ) {
            id
          }
        }
      `
    });
  }
);

Cypress.Commands.add("registerUser", (handle = "nope", id = "123abcd.git") => {
  client.mutate({
    variables: {
      handle: handle,
      id: id
    },
    mutation: gql`
      mutation RegisterUser($handle: ID!, $id: ID!) {
        registerUser(handle: $handle, id: $id) {
          messages {
            ... on UserRegistrationMessage {
              handle
              id
            }
          }
        }
      }
    `
  });
});
