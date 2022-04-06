<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Button from "./Button.svelte";
  import NetworkIcon from "./icons/Network.svelte";

  export let disabled: boolean = false;
  export let tracking: boolean = false;
  export let style: string | undefined = undefined;

  let hovering: boolean = false;
  let variant: "primary" | "outline";
  let title: string;

  const dispatch = createEventDispatcher();
  const click = () => {
    if (disabled) {
      return;
    }

    tracking = !tracking;
    dispatch(tracking ? "track" : "untrack");
  };

  const mouseenter = () => {
    hovering = true;
  };

  const mouseleave = () => {
    hovering = false;
  };

  $: {
    if (tracking) {
      variant = "outline";
      title = hovering ? "Untrack" : "Tracking";
    } else {
      variant = "primary";
      title = "Track";
    }
  }
</script>

<Button
  dataCy="track-toggle"
  {disabled}
  {style}
  {variant}
  icon={NetworkIcon}
  on:click={click}
  on:mouseenter={mouseenter}
  on:mouseleave={mouseleave}>
  {title}
</Button>
