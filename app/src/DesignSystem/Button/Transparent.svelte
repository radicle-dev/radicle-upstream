<script>
  import Button from "./Button.svelte";
  export let disabled = null;
</script>

<Button variant="transparent" {disabled}>
  <slot />
</Button>
