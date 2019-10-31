<script>
  import Button from "./Button.svelte";
  export let disabled;
</script>

<Button variant="secondary" {disabled}>
  <slot />
</Button>
