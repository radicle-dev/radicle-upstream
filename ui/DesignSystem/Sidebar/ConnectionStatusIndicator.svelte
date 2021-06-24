<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { SvelteComponent } from "svelte";
  import { activeRouteStore, push } from "ui/src/router";
  import { status } from "ui/src/localPeer";
  import { indicatorState } from "ui/src/network";

  import Tooltip from "ui/DesignSystem/Tooltip.svelte";
  import SidebarItem from "ui/DesignSystem/Sidebar/SidebarItem.svelte";

  let statusText: string = "";
  let statusCy: string = "";
  let statusIcon: typeof SvelteComponent | undefined = undefined;

  $: {
    const state = indicatorState($status);
    if (state) {
      statusText = state.text;
      statusCy = state.cy;
      statusIcon = state.icon;
    }
  }
</script>

<div>
  <Tooltip value={statusText}>
    <SidebarItem
      dataCy={statusCy}
      indicator
      active={$activeRouteStore.type === "network"}
      onClick={() => push({ type: "network" })}>
      <svelte:component this={statusIcon} />
    </SidebarItem>
  </Tooltip>
</div>
