<script>
  import Button from "./Button.svelte";
  export let disabled;
</script>

<Button variant="vanilla" {disabled}>
  <slot />
</Button>
