<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { FollowToggle, CopyableIdentifier } from "ui/DesignSystem";

  export let urn: string;
</script>

<style>
  .request-card {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    padding: 2rem;
    height: 15rem;
    display: flex;
    justify-content: space-between;
    flex-direction: column;
    cursor: pointer;
  }
</style>

<li class="request-card" on:click>
  <CopyableIdentifier kind="radicleId" value={urn} />
  <FollowToggle style="align-self: flex-start;" following on:unfollow />
</li>
