<script>
  import { push } from "svelte-spa-router";

  import { Icon, Input } from "../Primitive";

  import TrackToggle from "./TrackToggle.svelte";
  import Urn from "./Urn.svelte";

  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { Status } from "../../src/remote";
  import { updateUrn, validation } from "../../src/search";
  import { ValidationStatus } from "../../src/validation";

  let searchBar, value;

  export let content;

  const navigateToProject = () => {
    if ($validation.status !== ValidationStatus.Success) return;

    modal.hide();
    push(path.projectUntracked(value));
  };

  const onKeydown = ev => {
    switch (ev.code) {
      case "Enter":
        navigateToProject();
        break;
      case "Escape":
        modal.hide();
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
  .container {
    width: 26.25rem;
  }

  .search-bar {
    margin-bottom: 1rem;
  }

  .tracking-info {
    background: var(--color-background);
    cursor: pointer;
    border-radius: 0.5rem;
    height: 0;
    overflow: hidden;
    /* TODO(sos): replace with svelte transitions */
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

<!-- TODO(sos) -->
<svelte:window on:keydown={onKeydown} />

<div class="container" bind:this={content}>
  <div class="search-bar" bind:this={searchBar}>
    <!-- TODO(sos): fix autofocus / hotkey conflict -->
    <Input.Text
      autofocus
      bind:value
      placeholder="Have a Radicle project ID? Paste it here..."
      showLeftItem
      style="height: 3rem;"
      inputStyle="border: none; border-radius: 0.5rem; height: 3rem;"
      validation={$validation}>
      <div slot="left" style="display: flex;">
        <Icon.Search />
      </div>
    </Input.Text>
  </div>

  <div
    class="tracking-info"
    class:showTrackingInfo
    on:click={navigateToProject}>
    <div class="header">
      <h3 style="color: var(--color-foreground-level-6);">my-new-project</h3>
      <TrackToggle variant="expanded" />
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
