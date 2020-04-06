<script>
  import ApolloClient, { gql } from "apollo-boost";
  import { query } from "svelte-apollo";
  import { InMemoryCache } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import { initializeHotkeys } from "./lib/hotkeys.js";
  import {
    identityAvatarUrlStore,
    identityAvatarFallbackStore,
    identityHandleStore,
    identityShareableEntityIdentifierStore
  } from "./store/identity.js";

  import CreateProject from "./Screen/CreateProject.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Help from "./Screen/Help.svelte";
  import Network from "./Screen/Network.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Org from "./Screen/Org.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import RegisterProject from "./Screen/RegisterProject.svelte";
  import UserRegistration from "./Screen/UserRegistration.svelte";
  import Search from "./Screen/Search.svelte";
  import CreateIdentity from "./Screen/IdentityCreation.svelte";

  initializeHotkeys();

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql",
    cache: new InMemoryCache()
  });

  setClient(client);

  const GET_IDENTITY = gql`
    query Query($id: ID!) {
      identity(id: $id) {
        id
        avatarFallback {
          background {
            b
            g
            r
          }
          emoji
        }
        metadata {
          avatarUrl
          displayName
          handle
        }
        shareableEntityIdentifier
      }
    }
  `;

  const getIdentity = async () => {
    try {
      const response = await query(client, {
        query: GET_IDENTITY,
        variables: { id: "123" }
      });
      const result = await response.result();
      if (result.data.identity) {
        identityAvatarUrlStore.set(result.data.identity.metadata.avatarUrl);
        identityAvatarFallbackStore.set(result.data.identity.avatarFallback);
        identityHandleStore.set(result.data.identity.metadata.handle);
        identityShareableEntityIdentifierStore.set(
          result.data.identity.shareableEntityIdentifier
        );
      }
    } catch (error) {
      console.log(error);
    }
  };

  getIdentity();

  const routes = {
    "/identity/new": CreateIdentity,
    "/search": Search,
    "/network": Network,
    "/profile": Profile,
    "/profile/*": Profile,
    "/orgs/:id": Org,
    "/orgs/:id/*": Org,
    "/projects/new": CreateProject,
    "/projects/:id/register": RegisterProject,
    "/projects/:id/*": Project,
    "/design-system-guide": DesignSystemGuide,
    "/help": Help,
    "/user-registration": UserRegistration,
    "*": NotFound
  };
</script>

<Router {routes} />
