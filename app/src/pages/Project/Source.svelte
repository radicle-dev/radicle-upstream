<script>
  import { getContext } from "svelte";

  import ApolloClient from "apollo-boost";
  import * as path from "../../path.js";
  import { Header, Title, Text, Select } from "../../DesignSystem";
  import FileSource from "../../components/FileSource.svelte";
  import FileList from "../../components/FileList.svelte";
  import CommitTeaser from "../../components/CommitTeaser.svelte";

  import { revision } from "../../stores.js";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  export let params = null;

  const PAGE_DATA = gql`
    query($projectId: String!, $revision: String!, $path: String!) {
      tags(projectId: $projectId)
      branches(projectId: $projectId)
      blob(projectId: $projectId, revision: $revision, path: $path)
    }
  `;

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  $: filePath = `app/${params.wild || ""}`;

  $: pageData = query(client, {
    query: PAGE_DATA,
    variables: {
      projectId: getContext("projectId"),
      revision: $revision,
      path: filePath
    }
  });
</script>

{#await $pageData then result}
  <Select
    style="margin-bottom: 16px"
    items={[...result.data.tags, ...result.data.branches]}
    bind:value={$revision} />

  <CommitTeaser
    user={{ username: 'cloudhead', avatar: 'https://avatars2.githubusercontent.com/u/2326909?s=400&v=4' }}
    commitMessage="Remove debugging statement"
    commitSha="f4c7697"
    timestamp="13 days ago"
    style="margin-bottom: 48px" />

  <FileSource path={filePath} code={result.data.blob} />
  <FileList style="margin-top: 48px" />
{/await}
