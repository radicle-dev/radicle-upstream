<script>
  import Text from "../Text.svelte";
  import Icon from "../Icon";

  import { ValidationStatus } from "../../../src/validation.ts";

  export let style = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;

  export let disabled = null;
  export let validation = null;
  export let showSuccessCheck = false;

  function resize({ target }) {
    target.style.height = "40px";
    target.style.height = `${+target.scrollHeight + 2}px`;
  }

  export function textAreaResize(el) {
    resize({ target: el });
    el.style.overflow = "hidden";
    el.addEventListener("input", resize);

    return {
      destroy: () => el.removeEventListener("input", resize),
    };
  }
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  textarea {
    border: 1px solid var(--color-foreground-level-3);
    padding: 8px 12px;
    border-radius: 4px;
    width: 100%;
    background-color: var(--color-background);
    resize: none;
    max-height: 216px;
  }

  textarea::placeholder {
    color: var(--color-foreground-level-5);
  }

  textarea:focus {
    min-height: 98px;
    outline: none;
    box-shadow: 0 0 0 1px
      var(--focus-outline-color, var(--color-foreground-level-3));
    border: 1px solid
      var(--focus-outline-color, var(--color-foreground-level-3));
    background-color: var(--color-foreground-level-1);
  }

  textarea.invalid:focus,
  textarea.invalid {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-negative);
    border: 1px solid var(--color-negative);
    background: var(--color-background);
    background-position: right 14px top 55%;
    padding-right: 38px;
  }

  textarea.invalid:focus {
    background: var(--color-foreground-level-1);
  }

  .validation-row {
    display: flex;
    align-items: center;
    margin-top: 12px;
    margin-left: 12px;
  }
</style>

<div {style} class="wrapper">
  <textarea
    data-cy={dataCy}
    class:invalid={validation && validation.status === ValidationStatus.Error}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:textarea
    use:textAreaResize />

  {#if validation}
    {#if validation.status === ValidationStatus.Loading}
      <Icon.Spinner
        style="justify-content: flex-start; position: absolute; top: 8px; right:
        10px;" />
    {:else if validation.status === ValidationStatus.Success && showSuccessCheck}
      <Icon.CheckCircle
        style="fill: var(--color-positive); justify-content: flex-start;
        position: absolute; top: 8px; right: 10px;" />
    {:else if validation.status === ValidationStatus.Error}
      <Icon.Important
        style="fill: var(--color-negative); justify-content: flex-start;
        position: absolute; top: 8px; right: 10px;" />
      <div class="validation-row">
        <Text style="color: var(--color-negative); text-align: left;">
          {validation.message}
        </Text>
      </div>
    {/if}
  {/if}
</div>
