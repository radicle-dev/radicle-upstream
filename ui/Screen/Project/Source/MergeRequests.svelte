<script lang="typescript">
  import {
    selectMergeRequest,
    store,
  } from "../../../src/screen/project/source";
  import type { MergeRequest } from "../../../src/source";

  import { Error, Remote } from "../../../DesignSystem/Component";
  import MergeRequestList from "./MergeRequestList.svelte";

  const onSelect = ({ detail: mergeRequest }: { detail: MergeRequest }) => {
    selectMergeRequest(mergeRequest);
  };
</script>

<Remote {store} let:data={{ mergeRequests, project }}>
  <MergeRequestList
    {mergeRequests}
    defaultBranch={project.metadata.defaultBranch}
    on:select={onSelect} />
  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
