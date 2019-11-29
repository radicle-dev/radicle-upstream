<script>
  import ApolloClient from "apollo-boost";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  import { format } from "timeago.js";

  import { getContext } from "svelte";
  import { revision, objectPath } from "../stores.js";

  import { Icon } from "../DesignSystem";
  import CommitTeaser from "./CommitTeaser.svelte";

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  const SOURCE = gql`
    query($projectId: String!, $revision: String!, $path: String!) {
      blob(projectId: $projectId, revision: $revision, path: $path) {
        content
        info {
          lastCommit {
            author {
              name
              avatar
            }
            authorDate
            subject
            sha1
          }
        }
      }
    }
  `;

  $: source = query(client, {
    query: SOURCE,
    variables: {
      projectId: getContext("projectId"),
      revision: $revision,
      path: $objectPath
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
    font-family: "GT America Mono Regular";
    font-size: 14px;
    height: 48px;
    align-items: center;
    padding-left: 13px;
    border-bottom: 1px solid var(--color-lightgray);
  }

  .line-numbers {
    font-family: "GT America Mono Regular";
    font-size: 14px;
    background-color: var(--color-almostwhite);
    color: var(--color-gray);
    text-align: center;
    flex: 0 0 49px;
    border-right: 1px solid var(--color-lightgray);
    user-select: none;
  }

  .code {
    font-family: "GT America Mono Regular";
    font-size: 14px;
    padding-left: 8px;
    overflow-x: scroll;
  }

  .container {
    display: flex;
  }
</style>

{#await $source then result}
  <CommitTeaser
    user={{ username: result.data.blob.info.lastCommit.author.name, avatar: result.data.blob.info.lastCommit.author.avatar }}
    commitMessage={result.data.blob.info.lastCommit.subject}
    commitSha={result.data.blob.info.lastCommit.sha1.substring(0, 7)}
    timestamp={format(result.data.blob.info.lastCommit.authorDate)}
    style="margin-bottom: 48px" />

  <div class="file-source">
    <header>
      <Icon.File />
      {$objectPath}
    </header>
    <div class="container">
      <pre class="line-numbers">
        {@html result.data.blob.content
          .split('\n')
          .slice(0, -1)
          .map((_, index) => {
            return `${index + 1}`;
          })
          .join('\n')}
      </pre>
      <pre class="code">{result.data.blob.content}</pre>
    </div>
  </div>
{/await}
