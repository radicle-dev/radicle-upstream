<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "../Icon";
  import Spinner from "../../Component/Spinner.svelte";
  import KeyHint from "../../Component/KeyHint.svelte";

  import type { ValidationState } from "../../../src/validation";
  import { ValidationStatus as Status } from "../../../src/validation";

  export let style = "";
  export let inputStyle = "";
  export let placeholder = "";
  export let value: any;
  export let dataCy = "";
  export let disabled: boolean = false;
  export let inputElement: HTMLInputElement | undefined = undefined;

  export let validation: ValidationState | undefined = undefined;
  export let hint = "";
  export let showLeftItem: boolean = false;
  export let showSuccessCheck: boolean = false;
  export let spellcheck: boolean = false;
  export let autofocus: boolean = false;

  const dispatch = createEventDispatcher();

  // Can't use normal `autofocus` attribute on the `inputElement`:
  // "Autofocus processing was blocked because a document's URL has a fragment".
  // preventScroll is necessary for onboarding animations to work.
  $: if (autofocus) inputElement && inputElement.focus({ preventScroll: true });

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      dispatch("enter");
    }
  };

  $: showHint = hint.length > 0 && value.length === 0;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  input {
    border: 1px solid var(--color-foreground-level-3);
    padding: 0.5rem;
    border-radius: 0.25rem;
    width: 100%;
    height: 2.5rem;
    line-height: 3rem;
    padding: 0 0.75rem;
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
    background-position: right 0.875rem top 55%;
  }

  input.padding {
    padding-right: 2.375rem;
  }

  input.invalid:focus {
    background: var(--color-foreground-level-1);
  }

  .validation-row {
    display: flex;
    align-items: center;
    margin-top: 0.75rem;
    margin-left: 0.75rem;
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
    padding-left: 0.5rem;
    position: absolute;
    top: 0px;
  }

  .hint {
    justify-content: flex-start;
    position: absolute;
    right: 0.75rem;
    top: 50%;
    transform: translateY(-50%);
  }
</style>

<div {style} class="wrapper">
  <input
    data-cy={dataCy}
    class:invalid={validation && validation.status === Status.Error}
    class:padding={validation && validation.status !== Status.NotStarted}
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
    <div class="hint">
      <KeyHint {hint} />
    </div>
  {/if}

  {#if validation}
    {#if validation && validation.status === Status.Loading}
      <Spinner
        style="justify-content: flex-start; position: absolute; height: 100%;
        right: 10px;" />
    {:else if validation && validation.status === Status.Success && showSuccessCheck}
      <Icon.CheckCircle
        style="fill: var(--color-positive); justify-content: flex-start;
        position: absolute; top: 8px; right: 10px;" />
    {:else if validation && validation.status === Status.Error}
      <Icon.ExclamationCircle
        style="fill: var(--color-negative); justify-content: flex-start;
        position: absolute; top: 8px; right: 10px;" />
      <div class="validation-row">
        <p>{validation.message}</p>
      </div>
    {:else if showHint}
      <div class="hint">
        <KeyHint {hint} />
      </div>
    {/if}
  {/if}
</div>
