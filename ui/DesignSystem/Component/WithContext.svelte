<script>
  import { setContext } from "svelte";

  export let data = null;
  export let name = null;

  setContext(name, data);
</script>

<slot />
