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
  export let variant = "vanilla"; // vanilla | handle | project
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
    height: 40px;
    line-height: 48px;
    padding: 0 12px;
    background-color: var(--color-background);
  }

  input::placeholder {
    color: var(--color-foreground-level-6);
  }

  input.handle {
    padding: 0 40px 0 38px;
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 0 1px
      var(--focus-outline-color, var(--color-foreground-level-3));
    border: 1px solid
      var(--focus-outline-color, var(--color-foreground-level-3));
    background-color: var(--color-foreground-level-1);
  }

  input.invalid:focus,
  input.invalid {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-negative);
    border: 1px solid var(--color-negative);
    background: var(--color-background);
    background-position: right 14px top 55%;
    padding-right: 38px;
  }

  input.invalid:focus {
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
  <input
    data-cy={dataCy}
    class:invalid={!valid && !validationPending}
    class:handle={variant === 'handle'}
    {placeholder}
    bind:value
    {disabled}
    on:change
    on:input />
  {#if variant === 'handle'}
    <Avatar
      {avatarFallback}
      {imageUrl}
      variant="user"
      size="small"
      style="width: 24px; height: 40px; justify-content: flex-start; position:
      absolute; top: 0px; left: 8px" />
  {/if}

  {#if variant === 'handle' || variant === 'project'}
    {#if validationPending}
      <Icon.Spinner
        style="justify-content: flex-start; position: absolute; top: 8px; right:
        8px;" />
    {:else if value && valid}
      <Icon.CheckCircle
        style="fill: var(--color-positive); justify-content: flex-start;
        position: absolute; top: 8px; right: 8px;" />
    {/if}
  {/if}

  {#if !validationPending && !valid && validationMessage}
    <Icon.Important
      style="fill: var(--color-negative); justify-content: flex-start; position:
      absolute; top: 8px; right: 8px;" />
    <div class="validation-row">
      <Text style="color: var(--color-negative); text-align: left;">
        {validationMessage}
      </Text>
    </div>
  {/if}
</div>
