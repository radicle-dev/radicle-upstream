<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Avatar, Icon, Identifier } from "ui/DesignSystem";

  import * as ensResolver from "ui/src/org/ensResolver";
  import * as format from "ui/src/format";

  export let orgAddress: string;
  export let ownerAddress: string;
  export let threshold: number | undefined = undefined;
  export let registration: ensResolver.Registration | undefined = undefined;

  $: name = registration?.domain.replace(`.${ensResolver.DOMAIN}`, "");
  $: websiteUrl = registration?.url;
  $: githubUrl =
    registration?.github && `https://github.com/${registration.github}`;
  $: twitterUrl =
    registration?.twitter &&
    `https://twitter.com/${registration.twitter.replace("@", "")}`;
  $: seedId = registration?.seedId;
  $: seedApi = registration?.seedApi;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
    white-space: nowrap;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .name {
    margin-bottom: 0.5rem;
  }
  .domain {
    color: var(--color-foreground-level-4);
  }
</style>

<div style="display: flex">
  <Avatar
    style="margin-right: 2rem;"
    size="huge"
    kind={registration?.avatar
      ? { type: "orgImage", url: registration.avatar }
      : { type: "orgEmoji", uniqueIdentifier: orgAddress }} />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
      {#if name}
        {name}.<span class="domain">{ensResolver.DOMAIN}</span>
      {:else}
        {format.shortEthAddress(orgAddress)}
      {/if}
    </h1>
    <div style="display: flex; gap: 1rem;">
      <div>
        <div class="row">
          {#if threshold}
            <Icon.Gnosis />
          {:else}
            <Icon.Ethereum />
          {/if}
          <Identifier
            value={ownerAddress}
            kind="ethAddress"
            name="org owner address"
            showIcon={false} />
        </div>
        {#if threshold}
          <div class="row">
            <Icon.Orgs />
            {threshold}
            {threshold === 1 ? "signature" : "signatures"} required for quorum
          </div>
        {/if}
      </div>
      {#if websiteUrl || githubUrl || twitterUrl || seedId || seedApi}
        <div>
          {#if websiteUrl}
            <div class="row">
              <Icon.Globe />
              <a href={websiteUrl}>{websiteUrl}</a>
            </div>
          {/if}
          {#if githubUrl}
            <div class="row">
              <Icon.Github />
              <a href={githubUrl}>{githubUrl}</a>
            </div>
          {/if}
          {#if twitterUrl}
            <div class="row">
              <Icon.Twitter />
              <a href={twitterUrl}>{twitterUrl}</a>
            </div>
          {/if}
        </div>
        <div>
          {#if seedId}
            <div class="row">
              <Identifier value={seedId} kind="seedAddress" />
            </div>
          {/if}
          {#if seedApi}
            <div class="row">
              <Icon.Globe />
              <a href={seedApi}>{seedApi}</a>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>
