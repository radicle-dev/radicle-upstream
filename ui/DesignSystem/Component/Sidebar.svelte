<script>
  import { createEventDispatcher } from "svelte";
  import { location, link } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Avatar, Icon, Title } from "../Primitive";

  import AddOrgButton from "./Sidebar/AddOrgButton.svelte";

  const dispatch = createEventDispatcher();

  export let identity = null;
  export let orgs = null;

  let tooltip = { className: "hidden" };

  const hideTooltip = () => {
    tooltip.className = "hidden";
  };
  const showTooltip = (title, e) => {
    const rect = e.target.closest("[data-tooltip]").getBoundingClientRect();
    tooltip = { positionY: rect.top, title: title, className: "visible" };
  };
</script>

<style>
  .wrapper {
    width: var(--sidebar-width);
    height: 100%;
    background-color: var(--color-foreground-level-2);
    position: fixed;
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
  }

  .top {
    overflow-y: scroll;
    padding-bottom: 36px;
    padding-top: 16px;
  }

  .top::-webkit-scrollbar {
    display: none;
  }

  .bottom {
    position: relative;
    padding-top: 16px;
  }

  .bottom:before {
    position: absolute;
    content: " ";
    height: 36px;
    width: 68px;
    top: -36px;
    left: 0;
    background: linear-gradient(
      0deg,
      rgba(235, 239, 243, 1) 0%,
      rgba(235, 239, 243, 0) 100%
    );
  }

  .item {
    width: 68px;
    height: 36px;
    margin-bottom: 12px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .indicator:hover:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 36px;
    background-color: var(--color-foreground-level-5);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator.active:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 36px;
    background-color: var(--color-secondary);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .wrapper :global(li:hover svg) {
    fill: var(--color-secondary);
  }

  .indicator.active :global(svg) {
    fill: var(--color-secondary);
  }

  .item:hover .tooltip {
    transition: 0.3s;
  }

  .tooltip {
    white-space: nowrap;
    user-select: none;
    background-color: var(--color-foreground);
    color: var(--color-background);
    text-align: center;
    border-radius: 4px;
    padding: 4px 8px 6px 8px;
    position: fixed;

    left: 60px;
    pointer-events: none;
    z-index: 100;
    margin-top: 2px;
    margin-bottom: 2px;
  }
  .tooltip.visible {
    visibility: visible;
  }
  .tooltip.hidden {
    visibility: hidden;
  }

  .tooltip:before {
    content: "";
    display: block;
    width: 0;
    height: 0;
    position: absolute;

    border-top: 6px solid transparent;
    border-bottom: 6px solid transparent;
    border-right: 6px solid var(--color-foreground);
    left: -6px;
    border-top-left-radius: 30%;

    top: 10px;
  }

  a {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }
</style>

<div
  style={`top: ${tooltip.positionY}px`}
  class={`tooltip ${tooltip.className}`}>
  <Title>{tooltip.title}</Title>
</div>

<div class="wrapper" data-cy="sidebar">
  <ul class="top">
    <li
      class="item indicator"
      data-cy="profile"
      class:active={path.active(path.profile(), $location, true)}>
      <a
        data-tooltip
        on:mouseover={(e) => showTooltip(identity.metadata.handle, e)}
        on:mouseout={hideTooltip}
        href={path.profileProjects()}
        use:link>
        <Avatar
          size="medium"
          avatarFallback={identity.avatarFallback}
          imageUrl={identity.metadata.avatarUrl}
          variant="circle" />
      </a>
    </li>

    {#each orgs as org}
      <li
        class="item indicator"
        data-cy={`org-${org.id}`}
        class:active={path.active(path.orgs(org.id), $location, true)}>
        <a
          data-tooltip
          on:mouseover={(e) => showTooltip(org.id, e)}
          on:mouseout={hideTooltip}
          href={path.orgProjects(org.id)}
          use:link>
          <Avatar
            avatarFallback={org.avatarFallback}
            variant="square"
            size="medium" />
        </a>
      </li>
    {/each}

    <li class="item" data-cy="add-org-button">
      <div
        data-tooltip
        on:mouseover={(event) => showTooltip('Add org', event)}
        on:mouseout={hideTooltip}>
        <AddOrgButton on:click={() => dispatch('createorg')} />
      </div>
    </li>
  </ul>
  <ul class="bottom">
    <li
      class="item indicator"
      data-cy="wallet"
      data-tooltip
      on:mouseover={(e) => showTooltip('Wallet', e)}
      on:mouseout={hideTooltip}>
      <a href={path.profileWallet()} use:link>
        <Icon.Fund />
      </a>
    </li>
    <li
      class="item indicator"
      data-cy="settings"
      class:active={path.active(path.settings(), $location)}
      data-tooltip
      on:mouseover={(event) => showTooltip('Settings', event)}
      on:mouseout={hideTooltip}>
      <a href={path.settings()} use:link>
        <Icon.Settings />
      </a>
    </li>
  </ul>
</div>
