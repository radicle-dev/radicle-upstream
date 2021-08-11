<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import Button from "./Button.svelte";
  import Icon from "./Icon";

  export let disabled: boolean = false;
  export let following: boolean = false;
  export let style: string | undefined = undefined;

  let hovering: boolean = false;
  let variant: "primary" | "outline";
  let title: string;

  const dispatch = createEventDispatcher();
  const click = () => {
    if (disabled) {
      return;
    }

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
  {title}
</Button>
