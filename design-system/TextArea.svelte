<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { TextInputValidationState } from "./TextInput";

  import CheckCircleIcon from "./icons/CheckCircle.svelte";
  import ExclamationCircleIcon from "./icons/ExclamationCircle.svelte";

  import KeyHint from "./KeyHint.svelte";
  import Spinner from "./Spinner.svelte";

  export let autofocus: boolean = false;
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let showSuccessCheck: boolean = false;

  export let dataCy: string | undefined = undefined;
  export let inputStyle: string | undefined = undefined;
  export let style: string | undefined = undefined;
  export let caption: string | undefined = undefined;

  export let value: string | number | undefined = undefined;
  export let placeholder: string | undefined = undefined;

  export let hint: string | undefined = undefined;
  export let suffix: string | undefined = undefined;

  export let validationState: TextInputValidationState = {
    type: "unvalidated",
  };

  export const focus = (): void => {
    textareaElement && textareaElement.focus();
  };

  let textareaElement: HTMLTextAreaElement | undefined = undefined;

  // Can't use normal `autofocus` attribute on the `textareaElement`: "Autofocus
  // processing was blocked because a document's URL has a fragment".
  // preventScroll is necessary for onboarding animations to work.
  $: if (autofocus) {
    textareaElement && textareaElement.focus({ preventScroll: true });
  }

  let rightContainerWidth: number;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
    width: 100%;
  }

  textarea {
    background-color: var(--color-background);
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
    height: 2.5rem;
    padding: 0.5rem 0.75rem;
    width: 100%;

    resize: vertical;
    min-height: 80px;
  }

  textarea[disabled] {
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-4);
    cursor: not-allowed;
  }

  textarea[disabled]::placeholder {
    color: var(--color-foreground-level-4);
  }

  textarea[disabled]:hover {
    background-color: var(--color-foreground-level-1);
  }

  textarea[readonly]:hover {
    cursor: pointer;
  }

  textarea::placeholder {
    color: var(--color-foreground-level-5);
  }

  textarea:focus,
  textarea:hover {
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-3);
    outline: none;
  }

  textarea.invalid:focus,
  textarea.invalid {
    background-position: right 0.875rem top 55%;
    background: var(--color-background);
    border: 1px solid var(--color-negative);
    outline: none;
  }

  textarea.invalid:focus {
    background: var(--color-foreground-level-1);
  }

  .validation-message {
    align-items: center;
    color: var(--color-negative);
    display: flex;
    margin-left: 0.75rem;
    margin-top: 0.75rem;
    text-align: left;
  }

  .caption {
    align-items: center;
    color: var(--color-foreground-level-4);
    display: flex;
    margin-left: 0.75rem;
    margin-top: 0.75rem;
    text-align: left;
  }

  .right-container {
    align-items: center;
    display: flex;
    height: 2.5rem;
    position: absolute;
    right: 0;
    top: 0;
  }
</style>

<div {style} class="wrapper">
  <textarea
    style={`${inputStyle}; padding-right: ${
      rightContainerWidth ? `${rightContainerWidth}px` : "auto"
    };`}
    class:invalid={validationState.type === "invalid"}
    data-cy={dataCy}
    {placeholder}
    {disabled}
    {readonly}
    spellcheck={false}
    bind:value
    bind:this={textareaElement}
    on:change
    on:click
    on:input
    on:keydown
    on:keypress />

  <div class="right-container" bind:clientWidth={rightContainerWidth}>
    {#if hint && (validationState.type === "unvalidated" || validationState.type === "valid")}
      <KeyHint style="margin: 0 0.5rem;">{hint}</KeyHint>
    {/if}

    {#if suffix}
      <span style="color: var(--color-foreground-level-5); margin: 0 0.5rem;">
        {suffix}
      </span>
    {/if}

    {#if validationState.type === "pending"}
      <Spinner style="margin: 0 0.5rem;" />
    {:else if showSuccessCheck && validationState.type === "valid"}
      <CheckCircleIcon style="fill: var(--color-positive); margin: 0 0.5rem;" />
    {:else if validationState.type === "invalid"}
      <ExclamationCircleIcon
        dataCy="validation-error-icon"
        style="fill: var(--color-negative); margin: 0 0.5rem;" />
    {/if}
  </div>

  {#if caption}
    <div class="caption typo-text-small">
      {caption}
    </div>
  {/if}

  {#if validationState.type === "invalid"}
    <div class="validation-message">
      {validationState.message}
    </div>
  {/if}
</div>
