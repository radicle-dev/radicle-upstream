<script>
  import { createEventDispatcher } from "svelte";
  import { push } from "svelte-spa-router";

  import { Icon, Input } from "../Primitive";

  import TrackToggle from "./TrackToggle.svelte";
  import Urn from "./Urn.svelte";

  import * as path from "../../src/path";
  import { Status } from "../../src/remote";
  import { updateUrn, validation } from "../../src/search";
  import { ValidationStatus } from "../../src/validation";

  // Trying a `hide` variable here because animating the transition might be
  // easier this way
  let searchBar,
    hide = false,
    value;

  const navigateToProject = () => {
    if ($validation.status !== ValidationStatus.Success) return;

    hide = true;
    dispatch("hide");
    push(path.projectUntracked(value));
  };

  const onKeydown = ev => {
    switch (ev.key) {
      case "Escape":
        hide = true;
        dispatch("hide");
        break;
      case "Enter":
        navigateToProject();
        break;
    }
  };

  const dispatch = createEventDispatcher();

  const clickOutside = ev => {
    if (!hide && ev.target !== searchBar && !searchBar.contains(ev.target)) {
      hide = true;
      showTrackingInfo = false;
    }
  };

  $: if (value && value.length > 0) {
    updateUrn({ urn: value });
  }

  // TODO(sos): animate & show/hide based on actual remote response
  $: showTrackingInfo =
    value && value.length > 0 && $validation.status === Status.Success;
</script>

<style>
  .overlay {
    /* TODO(sos): ask brandonhaslegs for color & opacity for both light & dark modes */
    background-color: black;
    opacity: 0.7;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    z-index: 900;
  }

  .search-modal {
    /* TODO(sos): cute animation */
    display: flex;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    z-index: 10000;

    align-items: center;
    justify-content: center;
    text-align: center;
  }
  .hide {
    display: none;
  }

  .content {
    z-index: 1000;
    width: 420px;
  }

  .search-bar {
    margin-bottom: 16px;
  }

  .tracking-info {
    background: var(--color-background);
    border-radius: 4px;
    padding: 32px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;

    margin-bottom: 48px;
  }
</style>

<svelte:window on:click={clickOutside} on:keydown={onKeydown} />

<div class="search-modal" class:hide>
  <div class="overlay" />
  <div class="content" on:click={navigateToProject}>
    <div class="search-bar" bind:this={searchBar}>
      <Input.Text
        autofocus
        bind:value
        placeholder="Have a Radicle project ID? Paste it here..."
        showLeftItem
        validation={$validation}>
        <div slot="left" style="display: flex;">
          <Icon.Search />
        </div>
      </Input.Text>
    </div>

    {#if showTrackingInfo}
      <div class="tracking-info">
        <div class="header">
          <Urn
            urn="bshw82ienbytkx8173ndja0sjen833j88113jcb"
            notificationText="The project ID was copied to your clipboard" />
          <TrackToggle />
        </div>
        <p>
          You’re not tracking this project yet, so there’s nothing to show here.
          Track it and you’ll be notified as soon as it’s available.
        </p>
      </div>
    {/if}
  </div>
</div>
