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
  import * as notification from "ui/src/notification";
  import Tooltip from "ui/DesignSystem/Tooltip.svelte";

  import { CopyableIdentifier, Icon } from "ui/DesignSystem";

  export let ownerAddress: string;
  export let threshold: number | undefined = undefined;
  export let members: org.Member[] | undefined = undefined;
  export let registration: ensResolver.Registration | undefined = undefined;

  export function copy(): void {
    const content = `${registration?.seedId}@${registration?.seedHost}:8776`;
    ipc.copyToClipboard(content.trim());
    notification.info({
      message: "Org node address copied to your clipboard",
    });
  }

  $: seedId = registration?.seedId;
  $: seedHost = registration?.seedHost;
</script>

<style>
  aside {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    margin-top: 1.5rem;
    height: fit-content;
  }

  .row {
    padding: 1.5rem;
    color: var(--color-foreground-level-6);
    border-bottom: 1px solid var(--color-foreground-level-2);
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
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
</style>

<aside>
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
        <Icon.Proposals />
        <p class="typo-text-bold">Quorum</p>
      </div>
      {threshold} of {members.length}
      {members.length === 1 ? "signature" : "signatures"} required
    </div>
  {/if}
  {#if seedId && seedHost}
    <div class="row">
      <div class="row-title">
        <Icon.Server />
        <p class="typo-text-bold">Org node</p>
      </div>
      <Tooltip value="Copy org node address to clipboard" position="top">
        <p
          class="typo-overflow-ellipsis"
          style="cursor: pointer;"
          on:click|stopPropagation={copy}>
          {registration?.seedHost}
        </p>
      </Tooltip>
    </div>
  {/if}
  {#if registration?.github}
    <div class="row">
      <div class="row-title">
        <Icon.Github />
        <p class="typo-text-bold">Github</p>
      </div>
      <a class="typo-link url" href={`http://github.com/${registration.github}`}
        >github.com/{registration.github}</a>
    </div>
  {/if}
  {#if registration?.url}
    <div class="row">
      <div class="row-title">
        <Icon.Globe />
        <p class="typo-text-bold">Website</p>
      </div>
      <a class="typo-link url" href={registration.url}
        >{registration.url.replace(/https?:\/\//, "")}</a>
    </div>
  {/if}
  {#if registration?.twitter}
    <div class="row">
      <div class="row-title">
        <Icon.Twitter />
        <p class="typo-text-bold">Twitter</p>
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
