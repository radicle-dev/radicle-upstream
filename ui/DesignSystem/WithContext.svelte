<script lang="typescript">
  import { setContext } from "svelte";

  export let data: unknown;
  export let name: string;

  setContext(name, data);
</script>

<slot />
