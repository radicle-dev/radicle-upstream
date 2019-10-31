<script>
  export let style = null;
  export let placeholder = null;
</script>

<style>
  input {
    border: 1px solid var(--color-lightgray);
    display: block;
    padding: 8px;
    border-radius: 4px;
    width: 100%;
    height: 48px;
  }
</style>

<input {placeholder} {style} />
