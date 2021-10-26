<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as ensResolver from "ui/src/org/ensResolver";

  import EthereumIcon from "design-system/icons/Ethereum.svelte";
  import GithubIcon from "design-system/icons/Github.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import GnosisIcon from "design-system/icons/Gnosis.svelte";
  import ProposalsIcon from "design-system/icons/Proposals.svelte";
  import ServerIcon from "design-system/icons/Server.svelte";
  import TwitterIcon from "design-system/icons/Twitter.svelte";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let ownerAddress: string;
  export let threshold: number | undefined = undefined;
  export let memberCount: number | undefined = undefined;
  export let registration: ensResolver.Registration | undefined = undefined;
</script>

<style>
  aside {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    margin-top: 1.75rem;
    height: fit-content;
  }

  .row {
    padding: 1.5rem;
    color: var(--color-foreground-level-6);
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .row:not(:last-of-type) {
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
</style>

<aside>
  <div class="row">
    {#if threshold}
      <div class="title">
        <GnosisIcon />
        <span class="typo-text-bold">Gnosis safe</span>
      </div>
    {:else}
      <div class="title">
        <EthereumIcon />
        <span class="typo-text-bold">Owner address</span>
      </div>
    {/if}
    <CopyableIdentifier
      value={ownerAddress}
      kind="ethAddress"
      name="owner address"
      showIcon={false} />
  </div>
  {#if memberCount && threshold}
    <div class="row">
      <div class="title">
        <ProposalsIcon />
        <span class="typo-text-bold">Quorum</span>
      </div>
      {threshold} out of {memberCount}
    </div>
  {/if}
  {#if registration?.seedId && registration?.seedHost}
    <div class="row">
      <div class="title">
        <ServerIcon />
        <span class="typo-text-bold">Org node</span>
      </div>
      <Copyable
        name="seed address"
        clipboardContent={`${registration?.seedId}@${registration?.seedHost}:8776`}>
        <span class="typo-overflow-ellipsis">
          {registration?.seedHost}
        </span>
      </Copyable>
    </div>
  {/if}
  {#if registration?.github}
    <div class="row">
      <div class="title">
        <GithubIcon />
        <span class="typo-text-bold">Github</span>
      </div>
      <a class="typo-link url" href={`http://github.com/${registration.github}`}
        >github.com/{registration.github}</a>
    </div>
  {/if}
  {#if registration?.url}
    <div class="row">
      <div class="title">
        <GlobeIcon />
        <span class="typo-text-bold">Website</span>
      </div>
      <a class="typo-link url" href={registration.url}
        >{registration.url.replace(/https?:\/\//, "")}</a>
    </div>
  {/if}
  {#if registration?.twitter}
    <div class="row">
      <div class="title">
        <TwitterIcon />
        <span class="typo-text-bold">Twitter</span>
      </div>
      <a
        class="typo-link"
        href={`https://www.twitter.com/${registration.twitter.replace(
          "@",
          ""
        )}`}>twitter.com/{registration.twitter.replace("@", "")}</a>
    </div>
  {/if}
</aside>
