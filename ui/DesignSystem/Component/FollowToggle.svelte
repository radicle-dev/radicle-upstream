<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { ButtonVariant } from "../../src/style";

  import { Button, Icon } from "../Primitive";

  export let style: string | undefined = undefined;
  export let disabled: boolean = false;
  export let following: boolean = false;

  // Set this to true if you don't want the button to toggle.
  //
  // Useful for when the button triggers an action that removes the component
  // containing this button -- it prevents a visual flicker of the button just
  // before it disappears.
  export let actAsButton: boolean = false;

  const dispatch = createEventDispatcher();

  let hovering: boolean = false;
  let variant: ButtonVariant;
  let title: string;

  const click = () => {
    if (disabled) return;

    dispatch(!following ? "follow" : "unfollow");

    if (!actAsButton) {
      following = !following;
    }
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
  dataCy="follow-toggle"
  {disabled}
  {style}
  {variant}
  icon={Icon.Network}
  on:click={click}
  on:mouseenter={mouseenter}
  on:mouseleave={mouseleave}>
  {title}
</Button>
