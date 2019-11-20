<script>
  import ApolloClient from "apollo-boost";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { Caption, Icon, Text } from "../DesignSystem";
  import { link } from "svelte-spa-router";

  export let style = null;

  const LS = gql`
    query Query($projectId: String!, $head: String!, $prefix: String!) {
      ls(projectId: $projectId, head: $head, prefix: $prefix) {
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

  $: sourceTree = query(client, {
    query: LS,
    variables: { projectId: "123", head: "master", prefix: "/" }
  });
</script>

<style>
  table {
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
  <table {style}>
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
      {#each result.data.ls as entry}
        <tr>
          <td class="file-column">
            <a href="/path" use:link>
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
