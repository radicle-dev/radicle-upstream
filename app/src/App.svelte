<script>
  import ApolloClient from "apollo-boost";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";

  import Home from "./pages/Home.svelte";
  import NotFound from "./pages/NotFound.svelte";
  import Project from "./pages/Project.svelte";

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql"
  });
  setClient(client);

  const routes = {
    "/": Home,
    "/projects/:id": Project,
    "*": NotFound
  };
</script>

<Router {routes} />
