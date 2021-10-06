<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as theGraphApi from "ui/src/org/theGraphApi";
  import * as Org from "ui/src/org";
  import * as ensResolver from "ui/src/org/ensResolver";
  import { push } from "ui/src/router";

  import { Avatar, Badge, CopyableIdentifier, Icon } from "ui/DesignSystem";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  let resolvedOrgs: ResolvedOrg[] = [];

  interface ResolvedOrg {
    org: Org.Org;
    owner: Org.Owner;
  }

  async function fetchOrgs() {
    const unresolvedOrgs = await theGraphApi.getAllOrgs();
    let unsortedOrgs: ResolvedOrg[] = [];
    await Promise.all(
      unresolvedOrgs.map(async org => {
        org.registration = await ensResolver.getCachedRegistrationByAddress(
          org.id
        );
      })
    );

    unsortedOrgs = await Promise.all(
      unresolvedOrgs.map(async org => {
        const owner = await Org.getOwner(org.id);
        return { owner, org };
      })
    );

    resolvedOrgs = await Promise.all(
      unsortedOrgs.sort((a, b) => {
        if (!a.org.registration?.domain && b.org.registration?.domain) {
          return 1;
        }
        if (a.org.registration?.domain && !b.org.registration?.domain) {
          return -1;
        }
        return 0;
      })
    );
  }

  fetchOrgs();
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 2rem auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .box {
    border: 1px solid var(--color-foreground-level-3);
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
    gap: 0.5rem;
  }

  .row {
    display: flex;
    align-items: center;
  }

  .row:not(:first-child) {
    margin-top: 0.5rem;
  }
</style>

<ScreenLayout dataCy="network-page">
  <div class="container">
    <h1 style="padding: 0 0.75rem;">Explore orgs</h1>
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
                {org.registration?.domain ? org.registration.domain : org.id}
              </h3>
              <Badge
                style="align-self: flex-start;"
                caption={owner.type === "wallet"
                  ? "Single signer org"
                  : "Multi signer org"} />
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
                value={org.id}
                kind="ethAddress"
                name="org address"
                showIcon={false} />
            </li>
            {#if owner.type === "gnosis-safe"}
              <li class="row">
                <Icon.Proposals />
                <p style="margin-left: .5rem;">
                  {owner.threshold} of {owner.members.length} signatures required
                </p>
              </li>
            {/if}
          </ul>
        </div>
      {/each}
    </div>
  </div>
</ScreenLayout>
