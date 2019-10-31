<script>
  import Button from "./Button.svelte";
  export let disabled;
</script>

<Button variant="outline" {disabled}>
  <slot />
</Button>
