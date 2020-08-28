<script>
  export let style = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;

  export let disabled = null;
  export let spellcheck = false;
  export let autofocus = false;

  let inputElement;

  // Can't use normal `autofocus` attribute on the `input`:
  // "Autofocus processing was blocked because a document's URL has a fragment"
  $: if (autofocus) inputElement && inputElement.focus();
</script>

<style>
  input {
    border: 1px solid var(--color-foreground-level-3);
    padding: 8px;
    border-radius: 4px;
    width: 100%;
    height: 40px;
    line-height: 48px;
    padding: 0 12px;
    background-color: var(--color-background);
    color: var(--color-foreground-level-6);
  }

  input[disabled] {
    cursor: not-allowed;
    color: var(--color-foreground-level-4);
  }

  input[disabled]::placeholder {
    color: var(--color-foreground-level-4);
  }

  input[disabled]:hover {
    background-color: var(--color-background);
  }

  input::placeholder {
    color: var(--color-foreground-level-5);
  }

  input:focus,
  input:hover {
    outline: none;
    border: 1px solid
      var(--focus-outline-color, var(--color-foreground-level-3));
    background-color: var(--color-foreground-level-1);
  }
</style>

<input
  {style}
  type="password"
  data-cy={dataCy}
  {placeholder}
  bind:value
  {disabled}
  on:change
  on:input
  bind:this={inputElement}
  {spellcheck} />
