<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts" context="module">
  import { writable } from "svelte/store";

  const current = writable<HTMLDivElement | undefined>(undefined);

  const open = (component: HTMLDivElement): void => {
    current.set(component);
  };

  const close = (): void => {
    current.set(undefined);
  };
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let expanded: boolean;
  export let style: string | undefined = undefined;

  let container: HTMLDivElement;

  const dispatch = createEventDispatcher();

  const handleClick = (ev: MouseEvent) => {
    const component = $current;
    const inside = component && component.contains(ev.target as HTMLDivElement);
    if (!inside) {
      close();
    }
  };

  $: if (expanded) {
    open(container);
  }
  $: if ($current !== container) {
    dispatch("hide");
  }
</script>

<svelte:window on:click={handleClick} />

<div bind:this={container} {style}>
  <slot />
</div>
