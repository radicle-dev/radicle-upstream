<script>
  import { createEventDispatcher } from "svelte";

  import Icon from "../Icon";
  import Spinner from "../../Component/Spinner.svelte";

  import { ValidationStatus } from "../../../src/validation.ts";

  export let style = null;
  export let inputStyle = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;
  export let inputElement = null;
  export let hint = null;

  export let disabled = null;
  export let validation = null;
  export let showLeftItem = false;
  export let showSuccessCheck = false;
  export let spellcheck = false;
  export let autofocus = false;

  const dispatch = createEventDispatcher();

  // Can't use normal `autofocus` attribute on the `inputElement`:
  // "Autofocus processing was blocked because a document's URL has a fragment".
  // preventScroll is necessary for onboarding animations to work.
  $: if (autofocus) inputElement && inputElement.focus({ preventScroll: true });

  const onKeydown = event => {
    if (event.key === "Enter") {
      dispatch("enter");
    }
  };

  $: showHint = hint !== null && (value === "" || value === null);
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
  }

  input[disabled] {
    cursor: not-allowed;
    color: var(--color-foreground-level-4);
    background-color: var(--color-foreground-level-1);
  }

  input[disabled]::placeholder {
    color: var(--color-foreground-level-4);
  }

  input[disabled]:hover {
    background-color: var(--color-foreground-level-1);
  }

  input::placeholder {
    color: var(--color-foreground-level-5);
  }

  input.left-item {
    padding-left: 2.5rem;
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

  .left-item-wrapper {
    align-items: center;
    display: flex;
    height: 100%;
    justify-content: center;
    left: 0px;
    padding-left: 8px;
    position: absolute;
    top: 0px;
  }

  .hint {
    justify-content: flex-start;
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-5);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
  }
</style>

<div {style} class="wrapper">
  <input
    data-cy={dataCy}
    class:invalid={validation && validation.status !== ValidationStatus.NotStarted}
    class:left-item={showLeftItem}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input
    on:keydown={onKeydown}
    bind:this={inputElement}
    {spellcheck}
    style={inputStyle} />

  {#if showLeftItem}
    <div class="left-item-wrapper">
      <slot name="left" />
    </div>
  {/if}

  {#if showHint && !validation}
    <p class="typo-text hint">{hint}</p>
  {/if}

  {#if validation}
    {#if validation.status === ValidationStatus.Loading}
      <Spinner
        style="justify-content: flex-start; position: absolute; height: 100%;
        right: 10px;" />
    {:else if validation.status === ValidationStatus.Success && showSuccessCheck}
      <Icon.CheckCircle
        style="fill: var(--color-positive); justify-content: flex-start;
        position: absolute; top: 8px; right: 10px;" />
    {:else if validation.status === ValidationStatus.Error}
      <Icon.ExclamationCircle
        style="fill: var(--color-negative); justify-content: flex-start;
        position: absolute; top: 8px; right: 10px;" />
      <div class="validation-row">
        <p>{validation.message}</p>
      </div>
    {:else if showHint}
      <p class="typo-text hint">{hint}</p>
    {/if}
  {/if}
</div>
