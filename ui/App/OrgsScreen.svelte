<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import lodash from "lodash";

  import { push } from "ui/src/router";
  import { unreachable } from "ui/src/unreachable";
  import * as Org from "ui/src/org";
  import { store } from "ui/src/wallet";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as theGraphApi from "ui/src/org/theGraphApi";

  import {
    Avatar,
    Badge,
    CopyableIdentifier,
    Loading,
    Icon,
  } from "ui/DesignSystem";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";

  let resolvedOrgs: ResolvedOrg[] = [];

  let state: "loading" | "loaded" | "error" = "loading";

  interface ResolvedOrg {
    org: Org.Org;
    owner: Org.Owner;
  }

  async function loadScreen(): Promise<void> {
    state = "loading";
    try {
      resolvedOrgs = await fetchOrgs();
      state = "loaded";
    } catch (err: unknown) {
      error.show(
        new error.Error({ message: "Failed to fetch orgs", source: err })
      );
      state = "error";
    }
  }

  async function fetchOrgs(): Promise<ResolvedOrg[]> {
    const unresolvedOrgs = await theGraphApi.getAllOrgs();
    await Promise.all(
      unresolvedOrgs.map(async org => {
        org.registration = await ensResolver.getCachedRegistrationByAddress(
          org.id
        );
      })
    );

    await theGraphApi.resolveProjectCounts(unresolvedOrgs);

    const resolvedOrgs = await Promise.all(
      unresolvedOrgs.map(async org => {
        const owner = await Org.getOwner(org.id);
        return { owner, org };
      })
    );
    return lodash.orderBy(resolvedOrgs, data => data.org.registration?.domain);
  }

  $: wallet = $store;
  $: walletAddress = wallet.getAddress();

  function role(owner: Org.Owner): "member" | "owner" | undefined {
    if (owner.type === "gnosis-safe") {
      if (
        owner.metadata.members.some(
          memberAddress => memberAddress === walletAddress
        )
      ) {
        return "member";
      }
    } else if (owner.type === "wallet") {
      if (owner.address === walletAddress) {
        return "owner";
      }
    }
  }

  loadScreen();
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    margin: 0 auto;
    padding: 2rem var(--content-padding);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .box {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    cursor: pointer;
  }

  .box:hover {
    background-color: var(--color-foreground-level-1);
  }

  .box:active {
    transition: transform 0.1s ease-in-out;
    transform: scale(0.99);
  }

  .title-meta {
    display: flex;
    flex-direction: column;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .row {
    display: flex;
    align-items: center;
  }

  .row:not(:first-child) {
    margin-top: 0.5rem;
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 95vh;
  }
</style>

<ScreenLayout>
  <div class="container">
    <h1 style="padding: 0 0.75rem;">Explore orgs</h1>
    {#if state === "loading"}
      <div class="loading">
        <Loading />
      </div>
    {:else if state === "loaded"}
      <div class="grid">
        {#each resolvedOrgs as { org, owner }}
          <div
            class="box"
            on:click={() =>
              push({
                type: "org",
                params: { address: org.id, view: "projects" },
              })}>
            <header class="row">
              <Avatar
                style="margin-right: 1rem;"
                size="large"
                kind={org.registration?.avatar
                  ? { type: "orgImage", url: org.registration.avatar }
                  : { type: "orgEmoji", uniqueIdentifier: org.id }} />
              <div class="title-meta">
                <h3 class="typo-overflow-ellipsis">
                  {org.registration?.domain || org.id}
                </h3>
                {#if role(owner)}
                  <Badge
                    style="align-self: flex-start; margin-top: .5rem;"
                    caption={role(owner)} />
                {/if}
              </div>
            </header>
            <ul class="metadata">
              <li class="row">
                {#if owner.type === "gnosis-safe"}
                  <Icon.Gnosis />
                {:else}
                  <Icon.Ethereum />
                {/if}
                <CopyableIdentifier
                  style="margin-left: 0.5rem;"
                  value={owner.address}
                  kind="ethAddress"
                  name="owner address"
                  showIcon={false} />
              </li>
              {#if owner.type === "gnosis-safe"}
                <li class="row">
                  <Icon.User />
                  <p style="margin-left: .5rem;">
                    {owner.metadata.members.length}
                    {owner.metadata.members.length === 1 ? "member" : "members"}
                  </p>
                </li>
              {/if}
              {#if org.projectCount}
                <li class="row">
                  <Icon.Anchor />
                  <p style="margin-left: .5rem;">
                    {org.projectCount} anchored {org.projectCount === 1
                      ? "project"
                      : "projects"}
                  </p>
                </li>
              {/if}
            </ul>
          </div>
        {/each}
      </div>
    {:else if state === "error"}
      <EmptyState
        emoji="😬"
        text="Failed to load the orgs."
        primaryActionText="Try again"
        on:primaryAction={loadScreen} />
    {:else}
      {unreachable(state)}
    {/if}
  </div>
</ScreenLayout>
