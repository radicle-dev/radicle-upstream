<script lang="typescript">
  import type { Root } from "../../../../src/screen/project/source";

  import EmptyState from "../../EmptyState.svelte";
  import Readme from "../Readme.svelte";

  export let view: Root;
</script>

{#if view.readme}
  <Readme content={view.readme.content} path={view.readme.path} />
{:else}
  <EmptyState
    text="This project doesn't have a README yet."
    emoji="👀"
    style="height: 320px;" />
{/if}
