<script>
  import ApolloClient, { gql } from "apollo-boost";
  import { query } from "svelte-apollo";
  import {
    InMemoryCache,
    defaultDataIdFromObject
  } from "apollo-cache-inmemory";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import { hash } from "./lib/hash.js";
  import { initializeHotkeys } from "./lib/hotkeys.js";
  import {
    identityHandleStore,
    identityAvatarUrlStore
  } from "./store/identity.js";

  import CreateProject from "./Screen/CreateProject.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Help from "./Screen/Help.svelte";
  import Network from "./Screen/Network.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Profile from "./Screen/Profile.svelte";
  import Project from "./Screen/Project.svelte";
  import Projects from "./Screen/Projects.svelte";
  import RegisterProject from "./Screen/RegisterProject.svelte";
  import Search from "./Screen/Search.svelte";

  initializeHotkeys();

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql",
    cache: new InMemoryCache({
      dataIdFromObject: object => {
        switch (object.__typename) {
          case "Project":
            return hash(JSON.stringify(object));
          default:
            return defaultDataIdFromObject(object);
        }
      }
    })
  });

  setClient(client);

  const GET_IDENTITY = gql`
    query Query($id: ID!) {
      identity(id: $id) {
        id
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
        identityHandleStore.set(result.data.identity.metadata.handle);
        identityAvatarUrlStore.set(result.data.identity.metadata.avatarUrl);
      }
    } catch (error) {
      console.log(error);
    }
  };

  getIdentity();

  const routes = {
    "/search": Search,
    "/network": Network,
    "/profile": Profile,
    "/profile/*": Profile,
    "/projects": Projects,
    "/projects/new": CreateProject,
    "/projects/:id/register": RegisterProject,
    "/projects/:id/*": Project,
    "/design-system-guide": DesignSystemGuide,
    "/help": Help,
    "*": NotFound
  };
</script>

<Router {routes} />
