<script>
  import { objectType } from "../../stores.js";
  import FileSource from "../../components/FileSource.svelte";
  import FileList from "../../components/FileList.svelte";
  import RevisionSelector from "../../components/RevisionSelector.svelte";
</script>

<RevisionSelector />

{#if $objectType === 'blob'}
  <FileSource />
{:else}
  <FileList />
{/if}
