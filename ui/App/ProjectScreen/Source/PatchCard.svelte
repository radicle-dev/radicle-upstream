<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Patch } from "ui/src/project/patch";

  import { Icon } from "ui/DesignSystem";

  import CompareBranches from "./CompareBranches.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";

  export let patch: Patch;
  export let defaultBranch: string;

  $: iconColor = patch.merged
    ? "var(--color-negative)"
    : "var(--color-positive)";

  $: peerLabel = patch.identity ? patch.identity.metadata.handle : patch.peerId;
</script>

<style>
  .patch-card {
    display: flex;
    max-height: 3.2rem;
    justify-content: space-between;
    width: 100%;
  }

  .left {
    display: flex;
  }

  .info-column {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
    margin-right: 1.5rem;
  }

  .title-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
  }

  .title-row p {
    color: var(--color-foreground-level-6);
  }

  .desc-row {
    color: var(--color-foreground-level-5);
    display: flex;
    margin-top: 0.125rem;
    width: -webkit-fill-available;
  }
</style>

<div class="patch-card" data-cy={`patch-card-${patch.id}`}>
  <div class="left">
    <Icon.Revision style={`margin-right: 0.5rem; fill: ${iconColor};`} />
    <div>
      <div class="info-column">
        <div class="title-row">
          <p class="typo-text-bold typo-overflow-ellipsis" title={patch.id}>
            {#if patch.title}{patch.title}{:else}{patch.id}{/if}
          </p>
        </div>
        <div class="desc-row">
          <p style="margin-right: 0.5rem;">Opened by</p>
          {#if patch.identity}
            <UserIdentity
              modalStyle="top: 0.5rem; left: 3rem;"
              urn={patch.identity.urn}
              handle={patch.identity.metadata.handle} />
          {:else}
            <p style="margin-left: 0.5rem;">{patch.peerId}</p>
          {/if}
        </div>
      </div>
    </div>
  </div>
  <CompareBranches
    baseBranch={defaultBranch}
    compareBranch={`${peerLabel}/${patch.id}`} />
</div>
