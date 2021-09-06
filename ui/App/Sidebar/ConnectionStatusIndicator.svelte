<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as svelteStore from "svelte/store";

  import { activeRouteStore, push } from "ui/src/router";
  import { status } from "ui/src/localPeer";
  import { indicatorState } from "ui/src/network";

  import { Tooltip } from "ui/DesignSystem";
  import SidebarItem from "./SidebarItem.svelte";

  const indicatorStatus = svelteStore.derived(status, indicatorState);
</script>

<div data-cy="network">
  <Tooltip value={$indicatorStatus.text}>
    <SidebarItem
      dataCy={$indicatorStatus.cy}
      indicator
      active={$activeRouteStore.type === "network"}
      onClick={() => push({ type: "network" })}>
      <svelte:component this={$indicatorStatus.icon} />
    </SidebarItem>
  </Tooltip>
</div>
