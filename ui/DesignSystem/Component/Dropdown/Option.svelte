<script>
  import { createEventDispatcher } from "svelte";
  import { Avatar, Text } from "../../Primitive";

  const dispatch = createEventDispatcher();

  export let textProps = null;
  export let avatarProps = null;

  export let value = null;
  export let disabled = false;
  export let variant = "text"; // text | avatar

  const disabledColor = disabled
    ? "var(--color-foreground-level-4)"
    : "var(--color-foreground-level-6)";

  const clickHandler = () => {
    dispatch("selected", { value: value });
  };
</script>

<style>
  .option {
    display: flex;
    padding: 0 8px 0 8px;
    height: 46px;
    align-items: center;
    white-space: nowrap;
  }

  .option:hover {
    background-color: var(--color-foreground-level-2);
  }
</style>

<div class="option" on:click={clickHandler}>
  {#if variant === 'avatar'}
    <Avatar {...avatarProps} {disabled} style="margin-right: 12px;" />
  {:else}
    <Text style={`color: ${disabledColor}`}>{textProps.title}</Text>
  {/if}
</div>
