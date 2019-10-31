<script>
  import Button from "./Button.svelte";
  export let disabled = null;
</script>

<Button variant="outline" {disabled}>
  <slot />
</Button>
