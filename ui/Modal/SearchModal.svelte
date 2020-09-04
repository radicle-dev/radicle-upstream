<script>
  import { push } from "svelte-spa-router";
  import { createEventDispatcher } from "svelte";

  import { Icon, Input } from "../DesignSystem/Primitive";

  import { TrackToggle, Urn } from "../DesignSystem/Component";

  import * as path from "../src/path";
  import { Status } from "../src/remote";
  import { updateUrn, validation } from "../src/search";
  import { ValidationStatus } from "../src/validation";

  export let content;

  let searchBar, value, hasExpanded, showTrackingInfo;

  const dispatch = createEventDispatcher();

  const navigateToProject = () => {
    if ($validation.status !== ValidationStatus.Success) return;

    dispatch("hide");
    push(path.projectUntracked(value));
  };

  const onKeydown = ev => {
    switch (ev.code) {
      case "Enter":
        navigateToProject();
        break;
      case "Escape":
        dispatch("hide");
    }
  };

  $: if (value && value.length > 0) {
    updateUrn({ urn: value });
  }

  // TODO(sos): animate & show/hide based on actual remote response
  $: {
    showTrackingInfo = hasExpanded
      ? true
      : value && value.length > 0 && $validation.status === Status.Success;
    if (showTrackingInfo && !hasExpanded) hasExpanded = true;
  }
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
    transition: height 0.5s linear;
  }

  .showTrackingInfo {
    height: 13.5rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.25rem;
  }
</style>

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
        <Icon.MagnifyingGlass />
      </div>
    </Input.Text>
  </div>

  <!-- TODO(sos): Once we determine how searching works, make sure this looks right 
    if user changes urn
  -->
  <div
    class="tracking-info"
    class:showTrackingInfo
    on:click={navigateToProject}>
    <div style="padding: 2rem;">
      <div class="header">
        <h3 style="color: var(--color-foreground-level-6);">my-new-project</h3>
        <TrackToggle variant="expanded" />
      </div>

      <div style="display: flex; margin-bottom: 1rem;">
        <Urn
          urn={value || ''}
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
