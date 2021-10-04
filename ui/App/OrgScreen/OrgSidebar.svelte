<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as org from "ui/src/org";
  import type * as ensResolver from "ui/src/org/ensResolver";

  import * as ipc from "ui/src/ipc";

  import { CopyableIdentifier, Icon } from "ui/DesignSystem";

  export let ownerAddress: string;
  export let threshold: number | undefined = undefined;
  export let members: org.Member[] | undefined = undefined;
  export let registration: ensResolver.Registration | undefined = undefined;

  $: websiteUrl = registration?.url?.replace("https://", "");
  $: githubUrl = registration?.github && `github.com/${registration.github}`;
  $: twitterUrl =
    registration?.twitter &&
    `twitter.com/${registration.twitter.replace("@", "")}`;
  $: seedId = registration?.seedId;
  $: seedHost = registration?.seedHost;
</script>

<style>
  aside {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    margin-top: 1.5rem;
  }
  .row {
    padding: 1.5rem;
    color: var(--color-foreground-level-6);
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .row:last-child {
    border-bottom: none;
  }

  .row-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .url {
    cursor: pointer;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }
</style>

<aside>
  {#if websiteUrl || githubUrl || twitterUrl || (seedId && seedHost) || threshold}
    <div class="row">
      {#if threshold}
        <div class="row-title">
          <Icon.Gnosis />
          <p class="typo-text-bold">Gnosis safe</p>
        </div>
      {:else}
        <div class="row-title">
          <Icon.Ethereum />
          <p class="typo-text-bold">Owner address</p>
        </div>
      {/if}
      <CopyableIdentifier
        value={ownerAddress}
        kind="ethAddress"
        name="owner address"
        showIcon={false} />
    </div>
    {#if members && threshold}
      <div class="row">
        <div class="row-title">
          <Icon.Orgs />
          <p class="typo-text-bold">Quorum</p>
        </div>
        {threshold} of {members.length}
        {threshold === 1 ? "signature" : "signatures"} required
      </div>
    {/if}
    {#if seedId && seedHost}
      <div class="row">
        <div class="row-title">
          <Icon.Server />
          <p class="typo-text-bold">Seed address</p>
        </div>
        <CopyableIdentifier
          value={`${registration?.seedId}@${registration?.seedHost}:8776`}
          kind="seedAddress"
          showIcon={false} />
      </div>
    {/if}
    {#if githubUrl}
      <div class="row">
        <div class="row-title">
          <Icon.Github />
          <p class="typo-text-bold">Github</p>
        </div>
        <div class="url">
          <span
            on:click={() => {
              githubUrl && ipc.openUrl(githubUrl);
            }}>{githubUrl}</span>
        </div>
      </div>
    {/if}
    {#if websiteUrl}
      <div class="row">
        <div class="row-title">
          <Icon.Globe />
          <p class="typo-text-bold">Website</p>
        </div>
        <div class="url">
          <span
            on:click={() => {
              websiteUrl && ipc.openUrl(websiteUrl);
            }}>{websiteUrl}</span>
        </div>
      </div>
    {/if}
    {#if twitterUrl}
      <div class="row">
        <div class="row-title">
          <Icon.Twitter />
          <p class="typo-text-bold">Twitter</p>
        </div>
        <div class="url">
          <span
            on:click={() => {
              twitterUrl && ipc.openUrl(twitterUrl);
            }}>{twitterUrl}</span>
        </div>
      </div>
    {/if}
  {/if}
</aside>
