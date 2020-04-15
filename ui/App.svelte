<script>
  import Router, { push } from "svelte-spa-router";

  import { initializeHotkeys } from "./lib/hotkeys.js";
  import ApolloClient, { gql } from "apollo-boost";
  import { query } from "svelte-apollo";
  import { InMemoryCache } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import * as path from "./lib/path.js";
  import {
    identityAvatarUrlStore,
    identityAvatarFallbackStore,
    identityHandleStore,
    identityShareableEntityIdentifierStore
  } from "./store/identity.js";
  import { showNotification } from "./store/notification.js";

  import CreateProject from "./Screen/CreateProject.svelte";
  import Blank from "./Screen/Blank.svelte";
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
  import TransactionDetails from "./Screen/TransactionDetails.svelte";
  import CreateIdentity from "./Screen/IdentityCreation.svelte";

  initializeHotkeys();

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql",
    cache: new InMemoryCache()
  });

  setClient(client);

  const GET_IDENTITY = gql`
    query Query {
      session {
        identity {
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
    }
  `;

  const getIdentity = async () => {
    try {
      const response = query(client, {
        query: GET_IDENTITY
      });
      const result = await response.result();
      if (result.data.session.identity) {
        identityAvatarUrlStore.set(
          result.data.session.identity.metadata.avatarUrl
        );
        identityAvatarFallbackStore.set(
          result.data.session.identity.avatarFallback
        );
        identityHandleStore.set(result.data.session.identity.metadata.handle);
        identityShareableEntityIdentifierStore.set(
          result.data.session.identity.shareableEntityIdentifier
        );
        push(path.profile());
      } else {
        push(path.createIdentity());
      }
    } catch (error) {
      console.log(error);
      showNotification({
        text: "Identity could not be created",
        level: "error"
      });
    }
  };

  getIdentity();

  const routes = {
    "/": Blank,
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
    "/transactions/:id": TransactionDetails,
    "*": NotFound
  };
</script>

<Router {routes} />
