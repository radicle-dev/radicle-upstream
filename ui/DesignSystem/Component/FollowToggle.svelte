<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { ButtonVariant } from "../../src/style";

  import { Button, Icon } from "../Primitive";

  export let disabled: boolean = false;
  export let following: boolean = false;
  export let style: string | undefined = undefined;

  let hovering: boolean = false;
  let variant: ButtonVariant;
  let title: string;

  const dispatch = createEventDispatcher();
  const click = () => {
    if (disabled) return;

    following = !following;
    dispatch(following ? "follow" : "unfollow");
  };

  const mouseenter = () => {
    hovering = true;
  };

  const mouseleave = () => {
    hovering = false;
  };

  $: {
    if (following) {
      variant = "outline";
      title = hovering ? "Unfollow" : "Following";
    } else {
      variant = "primary";
      title = "Follow";
    }
  }
</script>

<Button
  dataCy="follow-toggle"
  {disabled}
  {style}
  {variant}
  icon={Icon.Network}
  on:click={click}
  on:mouseenter={mouseenter}
  on:mouseleave={mouseleave}>
  {#if !following}
    {title}
  {/if}
</Button>
