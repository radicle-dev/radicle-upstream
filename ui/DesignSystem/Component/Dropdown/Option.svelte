<script>
  import { createEventDispatcher } from "svelte";
  import { Avatar, Text } from "../../Primitive";

  const dispatch = createEventDispatcher();

  export let textProps = null;
  export let avatarProps = null;
  export let selected = false;

  export let value = null;
  export let style = null;
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
    height: 38px;
    align-items: center;
    white-space: nowrap;
    overflow: hidden;
  }

  .option.selected,
  .option:hover {
    background-color: var(--color-foreground-level-2);
  }
</style>

<div class="option" on:click={clickHandler} class:selected {style}>
  {#if variant === 'avatar'}
    <Avatar
      size="small"
      style="overflow:hidden; text-overflow: ellipsis; margin: 0 42px 0 8px;
      --title-color: var(--color-foreground-level-6);"
      {...avatarProps}
      {disabled} />
  {:else}
    <Text
      style={`overflow:hidden; text-overflow: ellipsis; margin: 0 42px  0 12px; color: ${disabledColor}`}>
      {textProps.title}
    </Text>
  {/if}
</div>
