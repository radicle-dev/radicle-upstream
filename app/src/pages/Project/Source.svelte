<script>
  import { getContext } from "svelte";

  import { objectPath, objectType, revision } from "../../stores.js";
  import { BLOB } from "../../types.js";
  import FileSource from "../../components/FileSource.svelte";
  import FileList from "../../components/FileList.svelte";
  import RevisionSelector from "../../components/RevisionSelector.svelte";

  let projectId = getContext("projectId");
</script>

<RevisionSelector />

{#if $objectType === BLOB}
  <FileSource {projectId} path={$objectPath} revision={$revision} />
{:else}
  <FileList {projectId} prefix={$objectPath} revision={$revision} />
{/if}
