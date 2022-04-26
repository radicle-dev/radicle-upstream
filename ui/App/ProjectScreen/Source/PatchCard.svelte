<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Patch from "ui/src/project/patch";
  import { store } from "ui/src/screen/project";

  import Button from "design-system/Button.svelte";
  import Hoverable from "design-system/Hoverable.svelte";
  import LinkIcon from "design-system/icons/Link.svelte";
  import PatchIcon from "./PatchIcon.svelte";

  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";
  import { Status } from "ui/src/remote";
  import { User } from "ui/src/project";

  export let patch: Patch.Patch;
  export let projectId: string;

  function getUserForPeerId(peerId: string): User | undefined {
    if ($store.status !== Status.Success) {
      return undefined;
    }

    return $store.data.peerSelection.find(p => p.peerId === peerId);
  }

  $: lastUpdateBy =
    patch.status.byPeerId && getUserForPeerId(patch.status.byPeerId);
</script>

<style>
  .patch-card {
    display: flex;
    max-height: 3.2rem;
    justify-content: space-between;
    width: 100%;
    align-items: center;
  }

  .left {
    display: flex;
  }

  .icon {
    margin-right: 8px;
  }

  .info-column {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
    margin-right: 1.5rem;
  }

  .title-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
  }

  .title-row p {
    color: var(--color-foreground-level-6);
  }

  .desc-row {
    color: var(--color-foreground-level-5);
    display: flex;
    margin-top: 0.125rem;
    width: -webkit-fill-available;
    align-items: center;
    gap: 0.5rem;
  }

  .item {
    display: flex;
  }
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }
</style>

<Hoverable let:hovering={hover} style="display: flex; flex: 1;">
  <div class="list-item">
    <div class="patch-card" data-cy={`patch-card-${patch.id}`}>
      <div class="left">
        <div class="icon">
          <PatchIcon status={patch.status.current} />
        </div>
        <div>
          <div class="info-column">
            <div class="title-row" data-cy={`patch-card-title-${patch.id}`}>
              <p class="typo-text-bold typo-overflow-ellipsis" title={patch.id}>
                {#if patch.title}{patch.title}{:else}{patch.id}{/if}
              </p>
            </div>
            <div class="desc-row">
              {#if patch.status.current === "open" || patch.status.current === "merged"}
                <div class="item">
                  <p style="margin-right: 0.5rem;">Opened by</p>
                  {#if patch.identity}
                    <UserIdentity
                      modalStyle="top: 0.5rem; left: 3rem;"
                      urn={patch.identity.urn}
                      handle={patch.identity.metadata.handle} />
                  {:else}
                    <p style="margin-left: 0.5rem;">{patch.peerId}</p>
                  {/if}
                </div>
              {:else if patch.status.current === "closed"}
                {#if patch.identity}
                  <div class="item">
                    <p style="margin-right: 0.5rem;">Opened by</p>
                    <UserIdentity
                      modalStyle="top: 0.5rem; left: 3rem;"
                      urn={patch.identity.urn}
                      handle={patch.identity.metadata.handle} />
                  </div>
                {:else}
                  <p style="margin-left: 0.5rem;">{patch.peerId}</p>
                {/if}
                {#if lastUpdateBy}
                  <div class="item">
                    <p style="margin-right: 0.5rem;">• Closed by</p>
                    <UserIdentity
                      modalStyle="top: 0.5rem; left: 3rem;"
                      urn={lastUpdateBy.identity.urn}
                      handle={lastUpdateBy.identity.metadata.handle} />
                  </div>
                {/if}
              {/if}
            </div>
          </div>
        </div>
      </div>
      {#if hover}
        <Button
          dataCy="copy-url"
          variant="vanilla"
          icon={LinkIcon}
          on:click={() => {
            Patch.copyPatchUrlToClipboard(projectId, patch);
          }}>Copy link</Button>
      {/if}
    </div>
  </div></Hoverable>
