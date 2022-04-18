<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  type Kind =
    | "projectId"
    | "peerId"
    | "seedAddress"
    | "ethAddress"
    | "commitHash";

  import type { Position } from "design-system/Tooltip";
  import type { SvelteComponent } from "svelte";

  import * as format from "design-system/lib/format";

  import AtIcon from "design-system/icons/At.svelte";
  import ComputerIcon from "design-system/icons/Computer.svelte";
  import EthereumIcon from "design-system/icons/Ethereum.svelte";
  import ServerIcon from "design-system/icons/Server.svelte";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  export let style: string | undefined = undefined;
  export let value: string;
  export let name: string | undefined = undefined;
  export let kind: Kind;
  export let showIcon: boolean = true;
  export let tooltipPosition: Position = "top";

  function kindToName(kind: Kind): string {
    switch (kind) {
      case "projectId":
        return "Project ID";
      case "peerId":
        return "Peer ID";
      case "seedAddress":
        return "Seed address";
      case "ethAddress":
        return "Ethereum address";
      case "commitHash":
        return "commit hash";
    }
  }

  function kindToIcon(kind: Kind): typeof SvelteComponent | undefined {
    switch (kind) {
      case "projectId":
        return AtIcon;
      case "peerId":
        return ComputerIcon;
      case "seedAddress":
        return ServerIcon;
      case "ethAddress":
        return EthereumIcon;
      case "commitHash":
        return undefined;
    }
  }

  function kindToShortenedValue(kind: Kind): string {
    switch (kind) {
      case "projectId":
        return format.shortProjectId(value);
      case "peerId":
        return format.shortPeerId(value);
      case "seedAddress":
        return format.shortSeedAddress(value);
      case "ethAddress":
        return format.shortEthAddress(value);
      case "commitHash":
        return format.shortCommitHash(value);
    }
  }

  $: copyableName = name || kindToName(kind);
  $: shortenedValue = kindToShortenedValue(kind);
</script>

<style>
  .id {
    display: flex;
  }

  .icon {
    margin-right: 0.5rem;
    display: flex;
  }
</style>

<div class="id" {style} data-cy={kind} data={value}>
  <Copyable
    name={copyableName}
    tooltipStyle="display: flex;"
    {tooltipPosition}
    clipboardContent={value}
    style="color: var(--color-foreground-level-6)">
    {#if showIcon && kindToIcon(kind)}
      <div class="icon">
        <svelte:component this={kindToIcon(kind)} />
      </div>
    {/if}
    {shortenedValue}
  </Copyable>
</div>
