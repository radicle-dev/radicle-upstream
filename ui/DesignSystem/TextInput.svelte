<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { ValidationState } from "ui/src/validation";

  import { ValidationStatus as Status } from "ui/src/validation";

  import Icon from "./Icon";
  import KeyHint from "./KeyHint.svelte";
  import Spinner from "./Spinner.svelte";

  export let autofocus: boolean = false;
  export let disabled: boolean = false;
  export let showSuccessCheck: boolean = false;
  export let concealed: boolean = false;

  export let dataCy: string | undefined = undefined;
  export let hint: string | undefined = undefined;
  export let inputStyle: string | undefined = undefined;
  export let placeholder: string | undefined = undefined;
  export let style: string | undefined = undefined;
  export let suffix: string | undefined = undefined;
  export let validation: ValidationState | undefined = undefined;
  export let value: string | undefined = undefined;

  export const focus = (): void => {
    inputElement && inputElement.focus();
  };

  let inputElement: HTMLInputElement | undefined = undefined;

  // Can't use normal `autofocus` attribute on the `inputElement`: "Autofocus
  // processing was blocked because a document's URL has a fragment".
  // preventScroll is necessary for onboarding animations to work.
  $: if (autofocus) {
    inputElement && inputElement.focus({ preventScroll: true });
  }

  // We do it this way to work around the svelte-check error: 'type' attribute
  // cannot be dynamic if input uses two-way binding (svelte).
  $: if (inputElement) {
    inputElement.type = concealed ? "password" : "text";
  }

  let rightContainerWidth: number;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  input {
    background-color: var(--color-background);
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
    height: 2.5rem;
    padding: 0.5rem 0.75rem;
    width: 100%;
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

  .right-container {
    height: 2.5rem;
    position: absolute;
    top: 0;
    right: 0;
    display: flex;
    align-items: center;
  }

  .concealed {
    color: var(--color-foreground-level-6);
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
    background-position: right 0.875rem top 55%;
  }

  input.invalid:focus {
    background: var(--color-foreground-level-1);
  }

  .validation-row {
    align-items: center;
    color: var(--color-negative);
    display: flex;
    margin-top: 0.75rem;
    margin-left: 0.75rem;
    text-align: left;
  }
</style>

<div {style} class="wrapper">
  <div on:click>
    <input
      style={`${inputStyle}; padding-right: ${
        rightContainerWidth ? `${rightContainerWidth}px` : "auto"
      };`}
      class:invalid={validation && validation.status === Status.Error}
      class:concealed
      data-cy={dataCy}
      {placeholder}
      {disabled}
      spellcheck={false}
      bind:value
      bind:this={inputElement}
      on:change
      on:input
      on:keydown
      on:keypress />
  </div>

  <div class="right-container" bind:clientWidth={rightContainerWidth}>
    {#if hint && (validation === undefined || validation.status === Status.Success)}
      <KeyHint style="margin: 0 0.5rem;">{hint}</KeyHint>
    {/if}

    {#if suffix}
      <span style="color: var(--color-foreground-level-5); margin: 0 0.5rem;">
        {suffix}
      </span>
    {/if}

    {#if validation && validation.status === Status.Loading}
      <Spinner style="margin: 0 0.5rem;" />
    {:else if validation && validation.status === Status.Success && showSuccessCheck}
      <Icon.CheckCircle
        style="fill: var(--color-positive); margin: 0 0.5rem;" />
    {:else if validation && validation.status === Status.Error}
      <Icon.ExclamationCircle
        dataCy="validation-error-icon"
        style="fill: var(--color-negative); margin: 0 0.5rem;" />
    {/if}
  </div>

  {#if validation && validation.status === Status.Error}
    <div class="validation-row">
      {validation.message}
    </div>
  {/if}
</div>
