<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Identity } from "../../src/identity";
  import type { Remote } from "../../src/project";
  import { RemoteType, Maintainer, Tracking } from "../../src/project";
  import { CSSPosition } from "../../src/style";

  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import { Overlay, Tooltip } from "../../DesignSystem/Component";

  export let expanded: boolean = false;
  export let peers: Remote[];
  export let selected: Identity;

  const hide = () => {
    expanded = false;
  };

  const show = () => {
    expanded = true;
  };

  const dispatch = createEventDispatcher();
  const open = (peer: Identity) => {
    hide();
    dispatch("open", peer);
  };
  const select = (peer: Identity) => {
    hide();
    dispatch("select", peer);
  };
</script>

<style>
  .peer-selector {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    padding: 0.5rem;
    display: flex;
    cursor: pointer;
    justify-content: space-between;
  }

  .peer-selector:hover {
    color: var(--color-foreground);
    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);
  }

  .peer-selector[hidden] {
    visibility: hidden;
  }

  .selector-avatar {
    display: flex;
    justify-content: space-between;
    width: 100%;
  }

  .selector-expand {
    align-self: flex-end;
  }

  .peer-dropdown-container {
    display: flex;
    position: absolute;
    right: 0;
    top: 0;
  }

  .peer-dropdown {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-medium);
    z-index: 8;
    max-width: 30rem;
    height: 100%;
    min-width: 100%;
  }

  .peer {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
    user-select: none;
    align-items: center;
    justify-content: space-between;
  }

  .open-profile {
    display: flex;
    justify-content: center;
    cursor: pointer;
  }
</style>

<Overlay
  {expanded}
  on:hide={hide}
  style="margin-right: 1rem; position: relative; user-select: none;">
  <div
    class="peer-selector"
    data-cy="peer-selector"
    on:click|stopPropagation={show}
    hidden={expanded}>
    <div class="selector-avatar typo-overflow-ellipsis">
      <Avatar
        avatarFallback={selected.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p class="typo-text-bold typo-overflow-ellipsis">
        {selected.metadata.handle || selected.shareableEntityIdentifier}
      </p>
      <p>
        <slot name="badge" peer={selected} />
      </p>
    </div>
    <div class="selector-expand">
      <Icon.ChevronUpDown
        style="vertical-align: bottom; fill: var(--color-foreground-level-4)" />
    </div>
  </div>
  <div class="peer-dropdown-container">
    <div class="peer-dropdown" hidden={!expanded}>
      {#each peers as { identity }}
        <div
          class="peer"
          class:selected={identity.peerId == selected.peerId}
          data-peer-handle={identity.metadata.handle}>
          <div style="display: flex;" on:click={() => select(identity)}>
            <Avatar
              avatarFallback={identity.avatarFallback}
              style="display: flex; justify-content: flex-start; margin-right:
            8px;"
              size="small"
              variant="circle" />
            <p class="typo-text-bold typo-overflow-ellipsis">
              {identity.metadata.handle || identity.shareableEntityIdentifier}
            </p>
            <p>
              <slot name="badge" peer={identity} />
            </p>
          </div>
          <Tooltip value="Go to profile" position={CSSPosition.Top}>
            <div
              data-cy={identity.metadata.handle}
              class="open-profile"
              on:click={() => {
                open(identity);
              }}>
              <Icon.ArrowBoxUpRight />
            </div>
          </Tooltip>
        </div>
      {/each}
    </div>
  </div>
</Overlay>
