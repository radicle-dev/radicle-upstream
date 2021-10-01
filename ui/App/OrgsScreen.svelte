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
  import * as ipc from "ui/src/ipc";

  import { Avatar, Badge, CopyableIdentifier, Icon } from "ui/DesignSystem";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  let resolvedOrgs: ResolvedOrg[] = [];

  interface ResolvedOrg {
    org: Org.Org;
    owner: Org.Owner;
  }

  async function fetchOrgs() {
    const unresolvedOrgs = await theGraphApi.getAllOrgs();

    await Promise.all(
      unresolvedOrgs.map(async org => {
        org.registration = await ensResolver.getCachedRegistrationByAddress(
          org.id
        );
      })
    );

    resolvedOrgs = await Promise.all(
      unresolvedOrgs.map(async org => {
        const owner = await Org.getOwner(org.id);
        return { owner, org };
      })
    );
  }

  fetchOrgs();
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 4rem auto;
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
</style>

<ScreenLayout dataCy="network-page">
  <div class="container">
    <h1>Orgs</h1>
    <div class="grid">
      {#each resolvedOrgs as { org, owner }}
        <div class="box">
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
                  ? "single signer org"
                  : "multisig org"} />
            </div>
          </header>
          <ul class="metadata">
            <li class="row">
              <CopyableIdentifier
                value={org.id}
                kind="ethAddress"
                name="org address" />
            </li>
            {#if org.registration?.url}
              <li class="row">
                <Icon.Globe />
                <div style="cursor: pointer; margin-left: .5rem;">
                  <span
                    on:click={() => {
                      org.registration?.url &&
                        ipc.openUrl(org.registration?.url);
                    }}>{org.registration?.url}</span>
                </div>
              </li>
            {/if}
          </ul>
        </div>
      {/each}
    </div>
  </div>
</ScreenLayout>
