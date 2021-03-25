<script lang="typescript">
  import * as projectSource from "../../../src/screen/project/source";
  import { Remote, Error } from "../../../DesignSystem/Component";
  import MergeRequestList from "./MergeRequestList.svelte";
</script>

<Remote store={projectSource.store} let:data={{ mergeRequests, project }}>
  <MergeRequestList {mergeRequests} {project} />
  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
