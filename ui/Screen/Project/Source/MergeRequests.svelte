<script lang="typescript">
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../../src/path";
  import type { Project } from "../../../src/project";
  import { store } from "../../../src/screen/project/source";
  import type { MergeRequest } from "../../../src/source";

  import { EmptyState, Error, Remote } from "../../../DesignSystem/Component";
  import MergeRequestList from "./MergeRequestList.svelte";

  const project: Project = getContext("project-page").project;
  const select = ({ detail: mergeRequest }: { detail: MergeRequest }) => {
    push(path.projectSourceMergeRequest(project.urn, mergeRequest));
  };
</script>

<Remote {store} let:data={{ mergeRequests }}>
  {#if mergeRequests.length > 0}
    <MergeRequestList {mergeRequests} on:select={select} />
  {:else}
    <EmptyState
      text="There’s nothing here yet, get started by opening your first merge request."
      primaryActionText="Start a new merge request"
      on:primaryAction={() => {
        console.log('Clicked new merge request');
      }} />
  {/if}

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
