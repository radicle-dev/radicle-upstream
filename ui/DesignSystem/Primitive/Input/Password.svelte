<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import Icon from "../Icon";
  import KeyHint from "../../Component/KeyHint.svelte";

  import type { ValidationState } from "../../../src/validation";
  import { ValidationStatus } from "../../../src/validation";

  export let style = "";
  export let inputStyle = "";
  export let placeholder = "";
  export let value = "";
  export let dataCy = "";
  export let disabled: boolean = false;
  export let inputElement: HTMLInputElement | undefined = undefined;

  export let validation: ValidationState | undefined = undefined;
  export let hint = "";
  export let spellcheck: boolean = false;
  export let autofocus: boolean = false;

  const dispatch = createEventDispatcher();

  // Can't use normal `autofocus` attribute on the `input`:
  // "Autofocus processing was blocked because a document's URL has a fragment"
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
    border: 0.0625rem solid var(--color-foreground-level-3);
    padding: 0.5rem;
    border-radius: 0.25rem;
    width: 100%;
    height: 2.5rem;
    line-height: 3rem;
    padding: 0 0.75rem;
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
    border: 0.0625rem solid
      var(--focus-outline-color, var(--color-foreground-level-3));
    background-color: var(--color-foreground-level-1);
  }

  input.invalid:focus,
  input.invalid {
    outline: none;
    border: 0.0625rem solid var(--color-negative);
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
    type="password"
    data-cy={dataCy}
    class:invalid={validation && validation.status === ValidationStatus.Error}
    class:padding={validation && validation.status !== ValidationStatus.NotStarted}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input
    on:keydown={onKeydown}
    bind:this={inputElement}
    {spellcheck}
    style={inputStyle} />

  {#if showHint && !validation}
    <div class="hint">
      <KeyHint {hint} />
    </div>
  {/if}

  {#if validation && validation.status === ValidationStatus.Error}
    <Icon.ExclamationCircle
      style="fill: var(--color-negative); justify-content: flex-start; position:
      absolute; top: 0.5rem; right: 0.625rem;" />
    <div class="validation-row">
      <p>{validation.message}</p>
    </div>
  {:else if showHint}
    <div class="hint">
      <KeyHint {hint} />
    </div>
  {/if}
</div>
