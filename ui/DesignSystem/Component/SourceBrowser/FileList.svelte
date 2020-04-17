<script>
  import { format } from "timeago.js";
  import { link } from "svelte-spa-router";

  import { TREE } from "../../../../native/types.js";
  import * as path from "../../../lib/path.js";

  import { Caption, Icon, Text } from "../../Primitive";
  import CommitTeaser from "./CommitTeaser.svelte";

  export let projectId = null;
  export let revision = null;

  export let tree = null;
</script>

<style>
  table {
    min-width: var(--content-min-width);
    border-collapse: collapse;
    width: 100%;
  }

  thead tr {
    height: 36px;
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-6);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 2px;
  }

  tbody tr {
    margin: 0 8px 0 8px;
  }

  tbody tr:hover {
    background-color: var(--color-foreground-level-1);
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
    color: var(--color-foreground-level-6);
  }

  .last-update-column {
    color: var(--color-foreground-level-6);
    text-align: right;
    padding-right: 12px;
  }
</style>

<CommitTeaser
  {projectId}
  user={{ username: tree.info.lastCommit.author.name, avatar: tree.info.lastCommit.author.avatar }}
  commitMessage={tree.info.lastCommit.summary}
  commitSha={tree.info.lastCommit.sha1}
  timestamp={format(tree.info.lastCommit.committerTime * 1000)}
  style="margin-bottom: 24px" />

<table data-cy="file-list">
  <thead>
    <tr>
      <td class="file-header">
        <Caption>Name</Caption>
      </td>
      <td>
        <Caption>Commit Message</Caption>
      </td>
      <td class="last-update-header">
        <Caption>Last Update</Caption>
      </td>
    </tr>
  </thead>

  <tbody>
    {#each tree.entries as entry}
      <tr>
        <td class="file-column">
          <a
            href={path.projectSource(projectId, revision, entry.info.objectType, entry.path)}
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
