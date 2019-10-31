<script>
  import Button from "./Button.svelte";
  export let disabled;
</script>

<Button variant="transparent" {disabled}>
  <slot />
</Button>
