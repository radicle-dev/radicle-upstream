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

  input:focus {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-purple);
    border: 1px solid var(--color-purple);
  }
</style>

<input {placeholder} {style} />
