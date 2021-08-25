<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  type Kind = "radicleId" | "deviceId" | "seedAddress" | "ethAddress";

  import type { SvelteComponent } from "svelte";
  import type { Position } from "./Tooltip.svelte";
  import * as format from "ui/src/format";

  import Copyable from "./Copyable.svelte";
  import Icon from "./Icon";

  export let style: string | undefined = undefined;
  export let value: string;
  export let name: string | undefined = undefined;
  export let kind: Kind;
  export let showIcon: boolean = true;
  export let tooltipPosition: Position = "top";

  function kindToName(kind: Kind): string {
    switch (kind) {
      case "radicleId":
        return "Radicle ID";
      case "deviceId":
        return "Device ID";
      case "seedAddress":
        return "Seed address";
      case "ethAddress":
        return "Ethereum address";
    }
  }

  function kindToIcon(kind: Kind): typeof SvelteComponent {
    switch (kind) {
      case "radicleId":
        return Icon.At;
      case "deviceId":
        return Icon.Computer;
      case "seedAddress":
        return Icon.Server;
      case "ethAddress":
        return Icon.Ethereum;
    }
  }

  function kindToShortenedValue(kind: Kind): string {
    switch (kind) {
      case "radicleId":
        return format.shortUrn(value);
      case "deviceId":
        return format.shortDeviceId(value);
      case "seedAddress":
        return format.shortSeedAddress(value);
      case "ethAddress":
        return format.shortEthAddress(value);
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
    {#if showIcon}
      <div class="icon">
        <svelte:component this={kindToIcon(kind)} />
      </div>
    {/if}
    {shortenedValue}
  </Copyable>
</div>
