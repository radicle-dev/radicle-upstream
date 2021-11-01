<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Identity } from "ui/src/identity";
  import { push } from "ui/src/router";
  import * as ethereum from "ui/src/ethereum";
  import * as Wallet from "ui/src/wallet";

  import { orgSidebarStore } from "ui/src/org";

  const ethereumEnvironment = ethereum.selectedEnvironment;
  const walletStore = Wallet.store;

  import * as format from "design-system/lib/format";

  import GithubIcon from "design-system/icons/Github.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import UserIcon from "design-system/icons/User.svelte";
  import OrgsIcon from "design-system/icons/Orgs.svelte";
  import TwitterIcon from "design-system/icons/Twitter.svelte";
  import Avatar from "design-system/Avatar.svelte";
  import Spinner from "design-system/Spinner.svelte";

  export let urn: string;
  export let identity: Identity;

  $: wallet = $walletStore;
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
  {#if $wallet.status === Wallet.Status.Connected && ethereum.supportedNetwork($ethereumEnvironment) === $wallet.connected.network}
    <div class="row">
      <div class="title">
        <OrgsIcon />
        <span class="typo-text-bold">Orgs</span>
      </div>
      {#if $orgSidebarStore.type === "fetched" && $orgSidebarStore.orgs.length > 0}
        <Spinner />
      {:else if $orgSidebarStore.type === "resolved"}
        {#each $orgSidebarStore.orgs as org (org.id)}
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
      {/if}
    </div>
  {/if}
  <div class="row">
    <div class="title">
      <UserIcon />
      <span class="typo-text-bold">Emoji</span>
    </div>
    <Avatar
      style="justify-content: flex-start;"
      size="large"
      kind={{ type: "userEmoji", uniqueIdentifier: urn }} />
  </div>
  <div class="row">
    <div class="title">
      <GlobeIcon />
      <span class="typo-text-bold">Website</span>
    </div>
    <a class="typo-link" href={`https://www.juliendonck.com`}
      >juliendonck.com</a>
  </div>
  <div class="row">
    <div class="title">
      <GithubIcon />
      <span class="typo-text-bold">Github</span>
    </div>
    <a class="typo-link" href={`https://www.github.com/juliendonck`}
      >github.com/juliendonck</a>
  </div>
  <div class="row">
    <div class="title">
      <TwitterIcon />
      <span class="typo-text-bold">Twitter</span>
    </div>
    <a class="typo-link" href={`https://www.twitter.com/juliendonck`}
      >twitter.com/juliendonck</a>
  </div>
</aside>
