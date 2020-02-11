<script>
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { format } from "timeago.js";

  import { TREE } from "../../lib/types.js";
  import * as path from "../../lib/path.js";

  import { Icon, Text } from "../Primitives";
  import CommitTeaser from "./CommitTeaser.svelte";
  import { link } from "svelte-spa-router";

  export let projectId = null;
  export let revision = null;
  export let prefix = null;

  const QUERY = gql`
    query Query($projectId: ID!, $revision: String!, $prefix: String!) {
      tree(id: $projectId, revision: $revision, prefix: $prefix) {
        info {
          lastCommit {
            author {
              name
              avatar
            }
            committerTime
            sha1
            summary
          }
        }
        entries {
          path
          info {
            objectType
            name
            lastCommit {
              author {
                name
              }
              summary
              committerTime
            }
          }
        }
      }
    }
  `;

  $: sourceTree = query(getClient(), {
    query: QUERY,
    variables: {
      projectId: projectId,
      revision: revision,
      prefix: prefix
    }
  });
</script>

<style>
  table {
    min-width: var(--content-min-width);
    border-collapse: collapse;
    width: 100%;
  }

  thead tr {
    height: 36px;
    background-color: var(--color-almostwhite);
    color: var(--color-darkgray);
    border: 1px solid var(--color-lightgray);
    border-radius: 2px;
  }

  tbody tr {
    margin: 0 8px 0 8px;
  }

  tbody tr:hover {
    background-color: var(--color-almostwhite);
  }

  a {
    display: flex;
  }

  td {
    vertical-align: middle;
    height: 40px;
  }

  .file-header {
    padding-left: 24px;
  }

  .last-update-header {
    text-align: right;
    padding-right: 24px;
  }

  .file-column {
    display: flex;
    align-items: center;
    padding-left: 8px;
  }

  .commit-message-column {
    color: var(--color-darkgray);
  }

  .last-update-column {
    color: var(--color-darkgray);
    text-align: right;
    padding-right: 12px;
  }
</style>

{#await $sourceTree then result}
  <CommitTeaser
    user={{ username: result.data.tree.info.lastCommit.author.name, avatar: result.data.tree.info.lastCommit.author.avatar }}
    commitMessage={result.data.tree.info.lastCommit.summary}
    commitSha={result.data.tree.info.lastCommit.sha1.substring(0, 7)}
    timestamp={format(result.data.tree.info.lastCommit.committerTime * 1000)}
    style="margin-bottom: 24px" />

  <table data-cy="file-list">
    <thead>
      <tr>
        <td class="file-header">
          <Text variant="caption">Name</Text>
        </td>
        <td>
          <Text variant="caption">Commit Message</Text>
        </td>
        <td class="last-update-header">
          <Text variant="caption">Last Update</Text>
        </td>
      </tr>
    </thead>

    <tbody>
      {#each result.data.tree.entries as entry}
        <tr>
          <td class="file-column">
            <a
              href={path.projectSource(projectId, revision, entry.info.objectType, entry.info.objectType === TREE ? entry.path + '/' : entry.path)}
              data-cy={`open-${entry.info.name}`}
              use:link>
              {#if entry.info.objectType === TREE}
                <Icon.Folder />
              {:else}
                <Icon.File />
              {/if}
              <Text style="margin-left: 4px">{entry.info.name}</Text>
            </a>
          </td>
          <td class="commit-message-column">
            <a href="/commit" use:link>
              <Text>{entry.info.lastCommit.summary}</Text>
            </a>
          </td>
          <td class="last-update-column">
            <Text>{format(entry.info.lastCommit.committerTime * 1000)}</Text>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
{/await}
