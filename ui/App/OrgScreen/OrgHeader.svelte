<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Avatar, Copyable, Icon, Identifier } from "ui/DesignSystem";

  import * as ensResolver from "ui/src/org/ensResolver";
  import * as format from "ui/src/format";
  import * as ipc from "ui/src/ipc";

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
  .url {
    cursor: pointer;
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
    <Copyable
      name="org address"
      clipboardContent={orgAddress}
      tooltipStyle="width: fit-content;">
      <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
        {#if name}
          {name}<span class="domain">.{ensResolver.DOMAIN}</span>
        {:else}
          {format.shortEthAddress(orgAddress)}
        {/if}
      </h1>
    </Copyable>
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
            name="owner address"
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
              <div class="url">
                <span
                  on:click={() => {
                    websiteUrl && ipc.openUrl(websiteUrl);
                  }}>{websiteUrl}</span>
              </div>
            </div>
          {/if}
          {#if githubUrl}
            <div class="row">
              <Icon.Github />
              <div class="url">
                <span
                  on:click={() => {
                    githubUrl && ipc.openUrl(githubUrl);
                  }}>{githubUrl}</span>
              </div>
            </div>
          {/if}
          {#if twitterUrl}
            <div class="row">
              <Icon.Twitter />
              <div class="url">
                <span
                  on:click={() => {
                    twitterUrl && ipc.openUrl(twitterUrl);
                  }}>{twitterUrl}</span>
              </div>
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
              <div class="url">
                <span
                  on:click={() => {
                    seedApi && ipc.openUrl(seedApi);
                  }}>{seedApi}</span>
              </div>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>
