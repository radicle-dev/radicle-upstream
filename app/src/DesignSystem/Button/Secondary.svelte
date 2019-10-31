<script>
  import Button from "./Button.svelte";
  export let disabled = null;
</script>

<Button variant="secondary" {disabled}>
  <slot />
</Button>
