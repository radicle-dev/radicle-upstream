<script>
  import Text from "../Text.svelte";
  import Icon from "../Icon";

  import { ValidationStatus } from "../../../src/validation.ts";

  export let style = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;

  export let disabled = null;

  // TODO(sos): replace with actual slot presence check once
  // https://github.com/sveltejs/svelte/issues/2106 is solved
  let slotFallback;

  export let validation;
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
    height: 48px;
    line-height: 48px;
    padding: 0 16px 0 16px;
    background-color: var(--color-background);
  }

  input.avatar {
    padding: 0 46px 0 46px;
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-secondary));
    border: 1px solid var(--focus-outline-color, var(--color-secondary));
  }

  input.invalid:focus,
  input.invalid {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-negative);
    border: 1px solid var(--color-negative);
    background: var(--color-background);
    background-position: right 14px top 55%;
    padding-right: 46px;
  }

  .validation-row {
    display: flex;
    align-items: center;
    margin-top: 16px;
  }

  .avatar-wrapper {
    position: absolute;
    top: 0px;
    left: 0px;
    padding-left: 8px;
    height: 48px;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>

<div {style} class="wrapper">
  <input
    data-cy={dataCy}
    class:invalid={validation.status === ValidationStatus.Error}
    class:avatar={!slotFallback}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input />

  <div class="avatar-wrapper">
    <slot name="avatar">
      <div bind:this={slotFallback} />
    </slot>
  </div>

  {#if validation.status === ValidationStatus.Loading}
    <Icon.Spinner
      style="justify-content: flex-start; position: absolute; top: 12px; right:
      10px;" />
  {:else if validation.status === ValidationStatus.Success}
    <Icon.CheckCircle
      style="fill: var(--color-positive); justify-content: flex-start; position:
      absolute; top: 12px; right: 10px;" />
  {:else if validation.status === ValidationStatus.Error}
    <Icon.Important
      style="fill: var(--color-negative); justify-content: flex-start; position:
      absolute; top: 12px; right: 10px;" />
    <div class="validation-row">
      <Text style="color: var(--color-negative); text-align: left;">
        {validation.message}
      </Text>
    </div>
  {/if}
</div>
