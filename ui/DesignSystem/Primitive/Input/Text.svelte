<script>
  import Text from "../Text.svelte";
  import Icon from "../Icon";
  import Avatar from "../Avatar.svelte";

  export let style = null;
  export let placeholder = null;
  export let value = null;
  export let dataCy = null;

  export let disabled = null;
  export let valid = true;
  export let validationMessage = null;
  export let variant = "plain"; // plain || register
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  input {
    border: 1px solid var(--color-lightgray);
    padding: 8px;
    border-radius: 4px;
    width: 100%;
    height: 48px;
    line-height: 48px;
    padding: 0 16px 0 16px;
    background-color: var(--color-white);
  }

  input.register {
    padding: 0 16px 0 54px;
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-purple));
    border: 1px solid var(--focus-outline-color, var(--color-purple));
  }

  input.invalid:focus,
  input.invalid {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-red);
    border: 1px solid var(--color-red);
    background: var(--color-white);
    background-position: right 14px top 55%;
    padding-right: 46px;
  }

  .validation-row {
    display: flex;
    align-items: center;
    margin-top: 16px;
  }
</style>

<div {style} class="wrapper">
  <input
    data-cy={dataCy}
    class:invalid={!valid}
    class:register={variant === 'register'}
    {placeholder}
    bind:value
    {disabled}
    on:change />
  {#if variant === 'register'}
    <Avatar
      variant="user"
      size="small"
      style="width: 32px; height: 48px; justify-content: flex-start; position:
      absolute; top: 0px; left: 8px" />
  {/if}

  {#if !valid && validationMessage}
    <Icon.Important
      style="fill: var(--color-red); justify-content: flex-start; position:
      absolute; top: 13px; right: 15px;" />
    <div class="validation-row">
      <Icon.Important style="fill: var(--color-red); margin-right: 8px;" />
      <Text variant="small" style="color: var(--color-red);">
        {validationMessage}
      </Text>
    </div>
  {/if}
</div>
