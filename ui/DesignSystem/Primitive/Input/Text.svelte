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
  export let variant = "vanilla"; // vanilla | avatar
  export let imageUrl = null;
  export let avatarFallback = null;
  export let validationPending = false;
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
    padding: 0 46px 0 54px;
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
</style>

<div {style} class="wrapper">
  <input
    data-cy={dataCy}
    class:invalid={!valid && !validationPending}
    class:avatar={variant === 'avatar'}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input />
  {#if variant === 'avatar'}
    <Avatar
      {avatarFallback}
      {imageUrl}
      variant="user"
      size="regular"
      style="width: 34px; height: 48px; justify-content: flex-start; position:
      absolute; top: 0px; left: 10px" />
  {/if}

  {#if variant === 'avatar'}
    {#if validationPending}
      <Icon.Spinner
        style="justify-content: flex-start; position: absolute; top: 12px;
        right: 10px;" />
    {:else if value && valid}
      <Icon.CheckCircle
        style="fill: var(--color-positive); justify-content: flex-start;
        position: absolute; top: 12px; right: 10px;" />
    {/if}
  {/if}

  {#if !validationPending && !valid && validationMessage}
    <Icon.Important
      style="fill: var(--color-negative); justify-content: flex-start; position:
      absolute; top: 12px; right: 10px;" />
    <div class="validation-row">
      <Text style="color: var(--color-negative); text-align: left;">
        {validationMessage}
      </Text>
    </div>
  {/if}
</div>
