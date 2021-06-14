<script lang="typescript">
  import { fade } from "svelte/transition";

  import type * as project from "ui/src/project";

  import * as org from "ui/src/org";
  import * as radicleAvatar from "radicle-avatar";
  import * as router from "ui/src/router";
  import * as style from "ui/src/style";

  import { Avatar, Icon } from "ui/DesignSystem/Primitive";
  import { Hoverable } from "ui/DesignSystem/Component";

  export let anchor: project.Anchor;
  export let orgAddress: string | undefined;
  export let replicated: boolean = false;

  let hover: boolean;

  const openCommit = () => {
    router.push({
      type: "project",
      activeView: { type: "commit", commitHash: anchor.commitHash },
      urn: anchor.projectId,
    });
  };
</script>

<style>
  .container {
    position: absolute;
  }
  .modal {
    position: absolute;
    width: 354px;
    border-radius: 0.5rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    top: -1rem;
    left: 2rem;
  }
  .header {
    border-bottom: 1px solid var(--color-foreground-level-2);
    height: 3.5rem;
    align-items: center;
    display: flex;
    padding: 0 46px 0 1rem;
    color: var(--color-primary);
  }
  .meta {
    display: flex;
    color: var(--color-foreground-level-6);
    margin: 1rem;
    align-items: center;
  }
  .org {
    color: var(--color-foreground-level-6);
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
</style>

<Hoverable bind:hovering={hover} style="display: inline-flex;">
  <Icon.AnchorSmall style="fill: var(--color-primary);" />

  <div class="container" on:click|stopPropagation>
    {#if hover}
      <div class="modal" out:fade|local={{ duration: 100, delay: 250 }}>
        <div class="header">
          <Icon.Anchor
            style="fill: var(--color-primary); margin-left: 0.5rem;" />

          {#if orgAddress}
            <p class="typo-text-bold">Anchored in</p>
            <Avatar
              size="small"
              style="margin: 0 0.5rem 0 0.5rem;"
              variant="square"
              avatarFallback={radicleAvatar.generate(
                orgAddress,
                radicleAvatar.Usage.Any
              )} />
            <p
              class="typo-text-bold org"
              style="color: var(--color-foreground-level-6);overflow: ellipsed">
              {orgAddress}
            </p>
          {/if}
        </div>
        <div class="meta">
          <Icon.Commit style="margin-right: 0.5rem;" />
          <p class="typo-text-small-bold" style="margin-right: 0.5rem;">
            Commit hash
          </p>
          {#if replicated}
            <p class="typo-text-small typo-link" on:click={openCommit}>
              {anchor.commitHash.slice(0, 7)}↗
            </p>
          {:else}
            <p class="typo-text-small">{anchor.commitHash.slice(0, 7)}</p>
          {/if}
        </div>
        <div class="meta">
          <Icon.Ethereum style="margin-right: 0.5rem;" />
          <p class="typo-text-small-bold" style="margin-right: 0.5rem;">
            Transaction hash
          </p>
          <p
            class="typo-text-small typo-link"
            on:click={() => {
              org.openTxOnEtherscan(anchor.transactionId);
            }}>
            {style.ellipsed(anchor.transactionId, 6)}↗
          </p>
        </div>
      </div>
    {/if}
  </div>
</Hoverable>
