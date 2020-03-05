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
  export let validationPending = false;
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
    class:invalid={!valid && !validationPending}
    class:register={variant === 'register'}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input />
  {#if variant === 'register'}
    <Avatar
      variant="user"
      size="small"
      style="width: 32px; height: 48px; justify-content: flex-start; position:
      absolute; top: 0px; left: 8px" />
  {/if}

  {#if variant === 'register'}
    {#if validationPending}
      <Icon.Spinner
        style="justify-content: flex-start; position: absolute; top: 13px;
        right: 15px;" />
    {:else if value && valid}
      <Icon.CheckCircle
        style="fill: var(--color-green); justify-content: flex-start; position:
        absolute; top: 13px; right: 15px;" />
    {/if}
  {/if}

  {#if !validationPending && !valid && validationMessage}
    <Icon.Important
      style="fill: var(--color-red); justify-content: flex-start; position:
      absolute; top: 13px; right: 15px;" />
    <div class="validation-row">
      <Text style="color: var(--color-red);">{validationMessage}</Text>
    </div>
  {/if}
</div>
