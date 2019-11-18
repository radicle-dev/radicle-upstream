<script>
  import ApolloClient from "apollo-boost";
  import * as path from "../../path.js";
  import { Header, Title, Text } from "../../DesignSystem";
  import Layout from "./Layout.svelte";

  import { head } from "../../stores.js";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  export let params = null;

  const PAGE_DATA = gql`
    query($projectId: String!, $head: String!, $path: String!) {
      tags(projectId: $projectId)
      branches(projectId: $projectId)
      cat(projectId: $projectId, head: $head, path: $path)
    }
  `;

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  $: pageData = query(client, {
    query: PAGE_DATA,
    variables: {
      projectId: params.id,
      head: $head,
      path: `app/${params.wild || ""}`
    }
  });
</script>

<Layout>
  <Header>
    <div slot="left">
      <Title.Big>Source</Title.Big>
    </div>
  </Header>

  <pre>{JSON.stringify(params, null, 2)}</pre>

  {#await $pageData}
    <Text.Regular>Loading...</Text.Regular>
  {:then result}
    <select bind:value={$head}>
      {#each result.data.tags as availableHead}
        <option value={availableHead}>{availableHead}</option>
      {/each}

      {#each result.data.branches as availableHead}
        <option value={availableHead}>{availableHead}</option>
      {/each}
    </select>

    <pre>{result.data.cat}</pre>
  {:catch error}
    <p>ERROR: {error}</p>
  {/await}
</Layout>
