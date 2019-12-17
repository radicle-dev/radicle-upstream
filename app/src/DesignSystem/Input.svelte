<script>
  export let style = null;
  export let placeholder = null;
  export let value = null;

  export let disabled = null;
</script>

<style>
  input {
    border: 1px solid var(--color-lightgray);
    padding: 8px;
    border-radius: 4px;
    width: 100%;
    height: 48px;
    line-height: 48px;
    padding: 0 16px 0 16px;
    background-color: var(--color-white);
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-purple);
    border: 1px solid var(--color-purple);
  }
</style>

<input {placeholder} {style} bind:value {disabled} />
