<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { Avatar, CopyableIdentifier } from "ui/DesignSystem";
  import type * as ensResolver from "ui/src/org/ensResolver";

  export let orgAddress: string;
  export let registration: ensResolver.Registration | undefined = undefined;
</script>

<style>
  .org-domain {
    color: var(--color-foreground-level-6);
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
</style>

<div style="display: flex;">
  <Avatar
    style="margin-right: 0.5rem;"
    size="small"
    kind={registration?.avatar
      ? { type: "orgImage", url: registration.avatar }
      : { type: "orgEmoji", uniqueIdentifier: orgAddress }} />
  {#if registration?.domain}
    <span class="typo-text-bold org-domain">
      {registration?.domain}
    </span>
  {:else}
    <CopyableIdentifier kind="ethAddress" value={orgAddress} showIcon={false} />
  {/if}
</div>
