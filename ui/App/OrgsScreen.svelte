<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import lodash from "lodash";

  import { push } from "ui/src/router";
  import { store } from "ui/src/wallet";
  import { unreachable } from "ui/src/unreachable";
  import * as Org from "ui/src/org";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as notification from "ui/src/notification";
  import * as theGraphApi from "ui/src/org/theGraphApi";

  import AnchorIcon from "design-system/icons/Anchor.svelte";
  import EthereumIcon from "design-system/icons/Ethereum.svelte";
  import GnosisIcon from "design-system/icons/Gnosis.svelte";
  import UserIcon from "design-system/icons/User.svelte";

  import Avatar from "design-system/Avatar.svelte";
  import Badge from "design-system/Badge.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";

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
      notification.showException(
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

    const projectCounts = await theGraphApi.getProjectCounts(
      unresolvedOrgs.map(org => org.id)
    );

    unresolvedOrgs.forEach(org => {
      org.projectCount = projectCounts[org.id];
    });

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
</style>

<ScreenLayout>
  <h1 style="padding: 2rem 0.75rem 0;">Explore orgs</h1>
  {#if state === "loading"}
    <Loading style="height: 80vh;" />
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
              <Badge
                style="align-self: flex-start; margin-top: .5rem;"
                text={role(owner)} />
            </div>
          </header>
          <ul class="metadata">
            <li class="row">
              {#if owner.type === "gnosis-safe"}
                <GnosisIcon />
              {:else}
                <EthereumIcon />
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
                <UserIcon />
                <p style="margin-left: .5rem;">
                  {owner.metadata.members.length}
                  {owner.metadata.members.length === 1 ? "member" : "members"}
                </p>
              </li>
            {/if}
            {#if org.projectCount}
              <li class="row">
                <AnchorIcon />
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
      emoji="🌴"
      text="Orgs aren't loading at the moment. Take a break!"
      primaryActionText="Try again"
      on:primaryAction={loadScreen} />
  {:else}
    {unreachable(state)}
  {/if}
</ScreenLayout>
