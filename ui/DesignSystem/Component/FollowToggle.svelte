<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { ButtonVariant } from "../../src/style";

  import { Button, Icon } from "../Primitive";

  export let style: string | undefined = undefined;
  export let disabled: boolean = false;
  export let following: boolean = false;

  const dispatch = createEventDispatcher();

  let hovering: boolean = false;
  let variant: ButtonVariant;
  let title: string;

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
      title = hovering ? "Unfollow" : "Following";
      variant = "outline";
    } else {
      title = "Follow";
      variant = "primary";
    }
  }
</script>

<Button
  {disabled}
  {style}
  {variant}
  icon={Icon.Network}
  on:click={click}
  on:mouseenter={mouseenter}
  on:mouseleave={mouseleave}>
  {title}
</Button>
