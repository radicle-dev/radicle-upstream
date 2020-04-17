<script>
  import { getContext } from "svelte";

  import {
    objectPathStore,
    objectTypeStore,
    revisionStore
  } from "../../store/sourceBrowser.js";

  import { BLOB } from "../../../native/types.js";
  import FileList from "./SourceBrowser/FileList.svelte";
  import FileSource from "./SourceBrowser/FileSource.svelte";
  import Stat from "./Stat.svelte";
  import Folder from "./SourceBrowser/Folder.svelte";
  import RevisionSelector from "./SourceBrowser/RevisionSelector.svelte";

  import { Icon } from "../../DesignSystem/Primitive";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const projectId = getContext("projectId");

  const GET_PROJECT = gql`
    query Query($id: ID!) {
      project(id: $projectId) {
        metadata {
          name
        }
        stats {
          commits
          branches
          contributors
        }
      }
    }
  `;

  export let style = null;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { projectId: getContext("projectId") }
  });
</script>

<style>
  .container {
    display: flex;
    width: inherit;
  }
  .column-left {
    display: flex;
    flex-direction: column;
    width: 286px;
    padding: 0 0.75rem;
  }

  .column-right {
    display: flex;
    flex-direction: column;
    padding-left: 0.75rem;
    width: 960px;
  }

  .source-tree {
    overflow-x: scroll;
  }

  .revision-selector {
    margin: 0.75rem 0;
    position: relative;
    width: 100%;
  }

  .repo-stats {
    height: 4rem;
    display: flex;
    justify-content: space-evenly;
    padding: 1.25rem 1rem;
  }
  .repo-stats > * {
    flex: 1;
    color: var(--color-foreground-level-6);
  }
</style>

{#await $project then result}
  <div class="container" {style}>
    <div class="column-left">
      <div class="revision-selector">
        <RevisionSelector style="height: 100%;" />
      </div>
      <div class="source-tree" data-cy="source-tree">
        <Folder name={result.data.project.metadata.name} />
      </div>
    </div>

    <div class="column-right">
      <div class="repo-stats">
        <div>
          <Stat icon={Icon.Commit} count={result.data.project.stats.commits}>
            &nbsp;Commits
          </Stat>
        </div>
        <div>
          <Stat icon={Icon.Branch} count={result.data.project.stats.branches}>
            &nbsp;Branches
          </Stat>
        </div>
        <div>
          <Stat
            icon={Icon.Member}
            count={result.data.project.stats.contributors}>
            &nbsp;Contributors
          </Stat>
        </div>
      </div>
      {#if $objectTypeStore === BLOB}
        <FileSource
          {projectId}
          path={$objectPathStore}
          revision={$revisionStore} />
      {:else}
        <FileList
          {projectId}
          prefix={$objectPathStore}
          revision={$revisionStore} />
      {/if}
    </div>
  </div>
{/await}
