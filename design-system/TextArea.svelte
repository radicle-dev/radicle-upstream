<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { TextInputValidationState } from "./TextInput";

  export let resizable: boolean = false;

  export let caption: string | undefined = undefined;
  export let textareaStyle: string | undefined = undefined;

  export let value: string | number | undefined = undefined;
  export let placeholder: string | undefined = undefined;

  export let validationState: TextInputValidationState = {
    type: "unvalidated",
  };

  let textareaElement: HTMLTextAreaElement | undefined = undefined;

  // We either auto-grow the text area, or allow the user to resize it. These
  // options are mutually exclusive because a user resized textarea would
  // automatically shrink upon text input otherwise.
  $: if (textareaElement && !resizable) {
    // React to changes to the textarea content.
    value;

    // Reset height to 0px on every value change so that the textarea
    // immediately shrinks when all text is deleted.
    textareaElement.style.height = `0px`;

    textareaElement.style.height = `${textareaElement.scrollHeight}px`;
  }
</script>

<style>
  .container {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  textarea {
    background-color: var(--color-background);
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
    height: 2.5rem;
    padding: 0.5rem 0.75rem;
    width: 100%;
    min-height: 2.5rem;
    resize: none;
  }

  .resizable {
    resize: vertical;
  }

  textarea::-webkit-scrollbar {
    display: initial;
  }

  textarea::-webkit-scrollbar-corner {
    background-color: transparent;
  }

  textarea::-webkit-resizer {
    background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAMAAAAolt3jAAAAAXNSR0IB2cksfwAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAD9QTFRFAAAAZWZmZmZmZmVmZWVmwsLBwsLCZ2ZmwsPCZmdlZWZnwcLBZmZkYGJjw8LDwsPBZmZnZWZkZ2ZkwMDBWFtcNbXb2AAAABV0Uk5TAP///////////////////////1H/YDRrSAAAAFBJREFUeJxVjUESgCAMA2mqAoqK6P/f6kzjIXIos5NumpI8g5LbpJnNQvDl52mWUYTquqnXwstshpHaTi+o+hHXccoKmHVW9yvIxv218ntivmOYAWpLfqaRAAAAAElFTkSuQmCC);
    background-size: 7px;
    background-repeat: no-repeat;
    background-position: bottom 1px right 1px;
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
</style>

<div class="container">
  <textarea
    style={textareaStyle}
    bind:this={textareaElement}
    bind:value
    class:invalid={validationState.type === "invalid"}
    class="wrap"
    class:resizable
    {placeholder}
    on:change
    on:click
    on:input
    on:keydown
    on:keypress />

  {#if caption && validationState.type !== "invalid"}
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
