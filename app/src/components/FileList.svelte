<script>
  import ApolloClient from "apollo-boost";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  import { getContext } from "svelte";
  import { revision, objectPath } from "../stores.js";
  import * as path from "../path.js";

  import { Caption, Icon, Text } from "../DesignSystem";
  import CommitTeaser from "../components/CommitTeaser.svelte";
  import { link } from "svelte-spa-router";

  const TREE = gql`
    query Query($projectId: String!, $revision: String!, $prefix: String!) {
      tree(projectId: $projectId, revision: $revision, prefix: $prefix) {
        path
        info {
          isDirectory
          name
          lastCommit
        }
      }
    }
  `;

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  const projectId = getContext("projectId");

  $: sourceTree = query(client, {
    query: TREE,
    variables: {
      projectId: projectId,
      revision: $revision,
      prefix: $objectPath
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
    user={{ username: 'cloudhead', avatar: 'https://avatars2.githubusercontent.com/u/2326909?s=400&v=4' }}
    commitMessage="Remove debugging statement"
    commitSha="f4c7697"
    timestamp="13 days ago"
    style="margin-bottom: 48px" />

  <table>
    <thead>
      <tr>
        <td style="padding-left: 24px">
          <Caption>Name</Caption>
        </td>
        <td>
          <Caption>Commit Message</Caption>
        </td>
        <td style="text-align: right;padding-right: 24px">
          <Caption>Last Update</Caption>
        </td>
      </tr>
    </thead>

    <tbody>
      {#each result.data.tree as entry}
        <tr>
          <td class="file-column">
            <a
              href={path.projectSource({
                id: projectId,
                revision: $revision,
                objectType: entry.info.isDirectory ? 'tree' : 'blob',
                path: entry.info.isDirectory ? entry.path + '/' : entry.path
              })}
              use:link>
              {#if entry.info.isDirectory}
                <Icon.Folder />
              {:else}
                <Icon.File />
              {/if}
              <Text.Regular style="margin-left: 4px">
                {entry.info.name}
              </Text.Regular>
            </a>
          </td>
          <td class="commit-message-column">
            <a href="/commit" use:link>
              <Text.Regular>{entry.info.lastCommit}</Text.Regular>
            </a>
          </td>
          <td class="last-update-column">
            <Text.Regular>2 months ago</Text.Regular>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
{/await}
