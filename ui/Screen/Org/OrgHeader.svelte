<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import { Avatar, Icon } from "ui/DesignSystem";
  import type { Registration } from "ui/src/org/ensResolver";

  import * as style from "ui/src/style";

  export let orgAddress: string;
  export let ownerAddress: string;
  export let threshold: number | undefined = undefined;
  export let registration: Registration | undefined = undefined;

  $: websiteUrl = registration?.url;
  $: githubUrl =
    registration?.github && `https://github.com/${registration.github}`;
  $: twitterUrl =
    registration?.twitter &&
    `https://twitter.com/${registration.twitter.replace("@", "")}`;
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
  .name {
    margin-bottom: 0.5rem;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .name-subdomain {
    color: var(--color-foreground-level-4);
  }
</style>

<div style="display: flex">
  <Avatar
    style="margin-right: 2rem;"
    size="huge"
    variant="square"
    imageUrl={registration?.avatar || undefined}
    avatarFallback={radicleAvatar.generate(
      orgAddress,
      radicleAvatar.Usage.Any
    )} />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
      {#if registration?.name}
        {registration?.name.replace(".radicle.eth", "")}<span
          class="name-subdomain">.radicle.eth</span>
      {:else}
        {style.ellipsed(orgAddress)}
      {/if}
    </h1>
    <div style="display: flex; gap: 1rem;">
      {#if websiteUrl || githubUrl || twitterUrl}
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
      {/if}
      <div>
        <div class="row">
          {#if threshold}
            <Icon.Gnosis />
          {:else}
            <Icon.Ethereum />
          {/if}
          {style.ellipsed(ownerAddress)}
        </div>
        {#if threshold}
          <div class="row">
            <Icon.Orgs />
            {threshold}
            {threshold === 1 ? "signature" : "signatures"} required for quorum
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
