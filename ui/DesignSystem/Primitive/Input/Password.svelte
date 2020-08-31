<script>
  import Icon from "../Icon";

  import { ValidationStatus } from "../../../src/validation.ts";

  export let style = null;
  export let inputStyle = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;

  export let disabled = null;
  export let validation = null;
  export let spellcheck = false;
  export let autofocus = false;

  let input;

  // Can't use normal `autofocus` attribute on the `input`:
  // "Autofocus processing was blocked because a document's URL has a fragment"
  $: if (autofocus) input && input.focus();
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
  }

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

  input.invalid:focus,
  input.invalid {
    outline: none;
    border: 1px solid var(--color-negative);
    background: var(--color-background);
    background-position: right 14px top 55%;
    padding-right: 38px;
  }

  input.invalid:focus {
    background: var(--color-foreground-level-1);
  }

  .validation-row {
    display: flex;
    align-items: center;
    margin-top: 12px;
    margin-left: 12px;
  }

  .validation-row p {
    color: var(--color-negative);
    text-align: left;
  }
</style>

<div {style} class="wrapper">
  <input
    type="password"
    data-cy={dataCy}
    class:invalid={validation && validation.status === ValidationStatus.Error}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input
    bind:this={input}
    {spellcheck}
    style={inputStyle} />

  {#if validation && validation.status === ValidationStatus.Error}
    <Icon.ExclamationCircle
      style="fill: var(--color-negative); justify-content: flex-start; position:
      absolute; top: 8px; right: 10px;" />
    <div class="validation-row">
      <p>{validation.message}</p>
    </div>
  {/if}
</div>
