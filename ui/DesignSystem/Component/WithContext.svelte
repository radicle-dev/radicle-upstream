<script lang="ts">
  import { setContext } from "svelte";

  export let data: any;
  export let name: string;

  setContext(name, data);
</script>

<slot />
