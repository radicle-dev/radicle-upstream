<script>
  import { createEventDispatcher } from "svelte";
  import { Avatar, Text, Title } from "../../Primitive";

  const dispatch = createEventDispatcher();

  export let text = null;
  export let value = null;
  export let disabled = false;
  export let identity = null;
  export let org = null;
  export let variant = "text"; // text | org | identity

  const disabledColor = () => {
    return disabled
      ? "var(--color-foreground-level-4)"
      : "var(--color-foreground-level-6)";
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

<div
  class="option"
  on:click={() => {
    dispatch('selected', { value: value });
  }}>
  {#if variant === 'identity'}
    <Avatar {...identity} style="margin-right: 12px;" />
    <Title style={`color: ${disabledColor()}`}>
      {identity.metadata.handle}
    </Title>
  {:else if variant === 'org'}
    <Avatar {...org} variant="project" style="margin-right: 12px;" />
    <Title style={`color: ${disabledColor()}`}>{org.metadata.name}</Title>
  {:else}
    <Text style={`color: ${disabledColor()}`}>{text}</Text>
  {/if}
</div>
