<script>
  import ApolloClient from "apollo-boost";
  import { setClient } from "svelte-apollo";
  import Router from "svelte-spa-router";
  import { routes } from "./routes.js";

  const client = new ApolloClient({
    uri: "http://127.0.0.1:8080/graphql"
  });

  setClient(client);
</script>

<Router {routes} />
