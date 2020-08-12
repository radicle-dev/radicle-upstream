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

  let searchBar, value;

  const navigateToProject = () => {
    if ($validation.status !== ValidationStatus.Success) return;

    dispatch("hide");
    push(path.projectUntracked(value));
  };

  const onKeydown = ev => {
    switch (ev.key) {
      case "Escape":
        dispatch("hide");
        break;
      case "Enter":
        navigateToProject();
        break;
    }
  };

  const dispatch = createEventDispatcher();

  const clickOutside = ev => {
    if (ev.target !== searchBar && !searchBar.contains(ev.target)) {
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
    display: flex;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    z-index: 10000;
    cursor: pointer;

    align-items: center;
    justify-content: center;
  }

  .content {
    z-index: 1000;
    width: 26.25rem;
  }

  .search-bar {
    margin-bottom: 1rem;
  }

  .tracking-info {
    background: var(--color-background);

    /* TODO(brandonhaslegs): the text input does not have this dramatic of a border radius
      do we want the component to have a variable border radius or 
      is this something that will be changing app-wide? 
    */
    border-radius: 0.5rem;

    height: 0;
    overflow: hidden;
    transition: height 0.25s ease;
  }

  .showTrackingInfo {
    height: 13.5rem;
    padding: 2rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;

    margin-bottom: 0.25rem;
  }
</style>

<svelte:window on:click={clickOutside} on:keydown={onKeydown} />

<div class="search-modal">
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

    <div class="tracking-info" class:showTrackingInfo>
      <div class="header">
        <h3 style="color: var(--color-foreground-level-6);">my-new-project</h3>
        <TrackToggle />
      </div>

      <div style="display: flex; margin-bottom: 1rem;">
        <Urn
          urn="bshw82ienbytkx8173ndja0sjen833j88113jcb"
          notificationText="The project ID was copied to your clipboard"
          showOnHover />
      </div>

      <p style="color: var(--color-foreground-level-6);">
        You’re not tracking this project yet, so there’s nothing to show here.
        Track it and you’ll be notified as soon as it’s available.
      </p>
    </div>
  </div>
</div>
