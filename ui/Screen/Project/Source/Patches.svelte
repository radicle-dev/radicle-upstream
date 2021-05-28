<script lang="typescript">
  import * as projectSource from "ui/src/screen/project/source";
  import { Remote, Error } from "ui/DesignSystem/Component";
  import PatchList from "./PatchList.svelte";
</script>

<Remote store={projectSource.store} let:data={{ patches, project }}>
  <PatchList {patches} {project} />
  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
