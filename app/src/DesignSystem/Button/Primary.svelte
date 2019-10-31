<script>
  import Button from "./Button.svelte";
  export let disabled = null;
</script>

<Button variant="primary" {disabled}>
  <slot />
</Button>
