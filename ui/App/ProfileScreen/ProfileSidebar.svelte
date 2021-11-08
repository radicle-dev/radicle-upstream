<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { push } from "ui/src/router";
  import type { Org } from "ui/src/org";
  import type { Registration } from "ui/src/org/ensResolver";

  import * as format from "design-system/lib/format";

  import BlockyIcon from "design-system/icons/Blocky.svelte";
  import GithubIcon from "design-system/icons/Github.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import EthIcon from "design-system/icons/Ethereum.svelte";
  import OrgsIcon from "design-system/icons/Orgs.svelte";
  import TwitterIcon from "design-system/icons/Twitter.svelte";
  import Avatar from "design-system/Avatar.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let urn: string;
  export let registration: Registration | undefined;
  export let ownedOrgs: Org[] = [];
</script>

<style>
  aside {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
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

  .org {
    display: flex;
    cursor: pointer;
  }
  .org:not(:last-of-type) {
    margin-bottom: 0.5rem;
  }
</style>

<aside>
  {#if registration?.address}
    <div class="row">
      <div class="title">
        <EthIcon />
        <span class="typo-text-bold">Attested address</span>
      </div>
      <CopyableIdentifier
        value={registration.address}
        kind="ethAddress"
        name="owner address"
        showIcon={false} />
    </div>
  {/if}
  {#if ownedOrgs.length > 0}
    <div class="row">
      <div class="title">
        <OrgsIcon />
        <span class="typo-text-bold">Orgs</span>
      </div>
      {#each ownedOrgs as org (org.id)}
        <div
          class="org"
          on:click={() =>
            push({
              type: "org",
              params: { address: org.id, view: "projects" },
            })}>
          <Avatar
            style="margin-right: 0.5rem;"
            size="small"
            kind={org.registration?.avatar
              ? { type: "orgImage", url: org.registration.avatar }
              : { type: "orgEmoji", uniqueIdentifier: org.id }} />
          <p class="typo-text-bold">
            {org.registration?.domain || format.shortEthAddress(org.id)}
          </p>
        </div>
      {/each}
    </div>
  {/if}
  <div class="row">
    <div class="title">
      <BlockyIcon />
      <span class="typo-text-bold">Blocky</span>
    </div>
    <Avatar
      style="justify-content: flex-start"
      size="large"
      kind={{ type: "userBlocky", uniqueIdentifier: urn }} />
  </div>
  {#if registration?.url}
    <div class="row">
      <div class="title">
        <GlobeIcon />
        <span class="typo-text-bold">Website</span>
      </div>
      <a class="typo-link" href={registration.url}
        >{registration.url.replace(/https?:\/\//, "")}</a>
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
