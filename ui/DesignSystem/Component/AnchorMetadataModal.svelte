<script lang="typescript">
  import type * as theGraphApi from "ui/src/theGraphApi";

  import * as radicleAvatar from "radicle-avatar";
  import * as style from "ui/src/style";

  import { Avatar, Icon } from "ui/DesignSystem/Primitive";
  import { Hoverable } from "ui/DesignSystem/Component";

  export let anchor: theGraphApi.ProjectAnchor;
  export let orgAddress: string | undefined;

  let hover: boolean;
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
    <div hidden={!hover} class="modal">
      <div class="header">
        <Icon.Anchor style="fill: var(--color-primary); margin-left: 0.5rem;" />

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
        <p class="typo-text-small-mono">{anchor.commitSha.slice(0, 7)}↗</p>
      </div>
      <div class="meta">
        <Icon.Ethereum style="margin-right: 0.5rem;" />
        <p class="typo-text-small-bold" style="margin-right: 0.5rem;">
          Transaction hash
        </p>
        <p class="typo-text-small">{style.ellipsed(anchor.id, 6)}↗</p>
      </div>
    </div>
  </div>
</Hoverable>
