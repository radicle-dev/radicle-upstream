<script>
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { format } from "timeago.js";

  import { Icon, Text } from "../DesignSystem";
  import CommitTeaser from "./CommitTeaser.svelte";

  export let projectId = null;
  export let path = null;
  export let revision = null;

  const SOURCE = gql`
    query($projectId: ID!, $revision: String!, $path: String!) {
      blob(id: $projectId, revision: $revision, path: $path) {
        binary
        content
        info {
          lastCommit {
            author {
              name
              avatar
            }
            committerTime
            summary
            sha1
          }
        }
      }
    }
  `;

  $: source = query(getClient(), {
    query: SOURCE,
    variables: {
      projectId: projectId,
      path: path,
      revision: revision
    }
  });
</script>

<style>
  .file-source {
    border: 1px solid var(--color-lightgray);
    border-radius: 3px;
    min-width: var(--content-min-width);
  }

  header {
    display: flex;
    background-color: var(--color-almostwhite);
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    height: 48px;
    align-items: center;
    padding-left: 13px;
    border-bottom: 1px solid var(--color-lightgray);
  }

  .container {
    display: flex;
  }
</style>

{#await $source then result}
  <CommitTeaser
    user={{ username: result.data.blob.info.lastCommit.author.name, avatar: result.data.blob.info.lastCommit.author.avatar }}
    commitMessage={result.data.blob.info.lastCommit.summary}
    commitSha={result.data.blob.info.lastCommit.sha1.substring(0, 7)}
    timestamp={format(result.data.blob.info.lastCommit.committerTime * 1000)}
    style="margin-bottom: 24px" />

  <div class="file-source" data-cy="file-source">
    <header>
      <Icon.File />
      {path}
    </header>
    <div class="container">
      {#if result.data.blob.binary}
        ఠ ͟ಠ Binary content.
      {:else}
        <Text
          variant="code"
          style="background-color: var(--color-almostwhite); color:
          var(--color-gray); text-align: center; flex: 0 0 49px; border-right:
          1px solid var(--color-lightgray); user-select: none;">
          {@html result.data.blob.content
            .split('\n')
            .slice(0, -1)
            .map((_, index) => {
              return `${index + 1}`;
            })
            .join('\n')}
        </Text>
        <Text variant="code" style="padding-left: 8px; overflow-x: scroll">
          {result.data.blob.content}
        </Text>
      {/if}
    </div>
  </div>
{/await}
