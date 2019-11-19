<script>
  import ApolloClient from "apollo-boost";
  import * as path from "../../path.js";
  import { Header, Title, Text, Select } from "../../DesignSystem";
  import FileSource from "../../components/FileSource.svelte";

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

  $: filePath = `app/${params.wild || ""}`;

  $: pageData = query(client, {
    query: PAGE_DATA,
    variables: {
      projectId: params.id,
      head: $head,
      path: filePath
    }
  });
</script>

{#await $pageData then result}
  <Select
    style="margin-bottom: 16px"
    items={[...result.data.tags, ...result.data.branches]}
    bind:value={$head} />

  <FileSource path={filePath} code={result.data.cat} />
{/await}
