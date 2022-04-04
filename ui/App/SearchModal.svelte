<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as project from "ui/src/project";
  import type * as proxyProject from "proxy-client/project";

  import { fade } from "svelte/transition";

  import { VALID_PEER_MATCH } from "ui/src/screen/project";

  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import * as urn from "ui/src/urn";

  import MagnifyingGlassIcon from "design-system/icons/MagnifyingGlass.svelte";

  import FollowToggle from "design-system/FollowToggle.svelte";
  import TextInput from "design-system/TextInput.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import ProjectStats from "ui/App/SharedComponents/ProjectStats.svelte";

  export let searchQuery: string = "";

  const projectRequestStore = remote.createStore<proxyProject.Request>();
  const projectSearchStore = remote.createStore<proxyProject.Project>();

  function navigateToProject(project: project.Project): void {
    reset();
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });
    modal.hide();
  }

  function onKeydown(event: KeyboardEvent): void {
    switch (event.code) {
      case "Enter":
        if ($projectSearchStore.status === remote.Status.Success) {
          navigateToProject($projectSearchStore.data);
        } else if ($projectSearchStore.status === remote.Status.Error) {
          follow();
        }
        break;
      case "Escape":
        reset();
        modal.hide();
        break;
    }
  }

  function reset(): void {
    projectRequestStore.reset();
    projectSearchStore.reset();
  }

  function follow(): void {
    if (validationState.type === "valid") {
      remote.fetch(
        projectRequestStore,
        proxy.client.project.requestSubmit(sanitizedSearchQuery)
      );
    }
  }

  let validationState:
    | { type: "initial" }
    | { type: "valid" }
    | { type: "invalid"; message: string } = { type: "initial" };

  $: sanitizedSearchQuery = searchQuery.trim();

  // Validate input entered, at the moment valid RadUrns are the only
  // acceptable input.
  $: if (sanitizedSearchQuery.length > 0) {
    const result = urn.extractSha1FromUrn(sanitizedSearchQuery);

    if (result.isUrnValid) {
      validationState = { type: "valid" };
      // Load and show project metadata.
      remote.fetch(
        projectSearchStore,
        proxy.client.project.get(sanitizedSearchQuery)
      );
    } else if (VALID_PEER_MATCH.test(sanitizedSearchQuery)) {
      validationState = {
        type: "invalid",
        message:
          "You’ve entered a Peer ID instead of a Project URN. To collaborate with someone, add their Peer ID as a remote directly to a project.",
      };
    } else {
      validationState = {
        type: "invalid",
        message: "That’s not a valid Project URN.",
      };
    }
  } else {
    validationState = { type: "initial" };
  }

  // Reset searches if the input became invalid.
  $: if (validationState.type !== "valid") {
    reset();
  }

  $: if ($projectRequestStore.status === remote.Status.Success) {
    reset();
    router.push({ type: "profile" });
    notification.show({
      type: "info",
      message: "You’ll be notified when this project has been found.",
    });
    modal.hide();
  }

  $: if ($projectRequestStore.status === remote.Status.Error) {
    notification.showException($projectRequestStore.error);
  }
</script>

<style>
  .container {
    width: 30rem;
    /* Fixed height prevents changing the position of the element when
      `.result` is shown. */
    height: 3rem;
  }

  .search-bar {
    align-items: center;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    box-shadow: var(--color-shadows);
    color: var(--color-foreground-level-6);
    display: flex;
    height: 3rem;
    margin-bottom: 1rem;
    position: relative;
  }

  .result {
    background: var(--color-background);
    border-radius: 0.5rem;
    box-shadow: var(--color-shadows);
    color: var(--color-foreground-level-6);
    padding: 1.5rem;
  }

  .header {
    align-items: center;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .project-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

<div class="container" data-cy="search-modal">
  <div class="search-bar">
    <MagnifyingGlassIcon style="margin-left: 0.5rem;" />
    <TextInput
      dataCy="search-input"
      style="flex: 1;"
      inputStyle="border: 0; background: transparent;"
      autofocus
      bind:value={searchQuery}
      on:keydown={onKeydown}
      placeholder="Enter a Project URN here…"
      hint={validationState.type === "valid" ? "↵" : ""} />
  </div>

  {#if $projectSearchStore.status === remote.Status.Success}
    <div class="result" out:fade|local={{ duration: 100 }}>
      <h3
        class="header"
        data-cy="project-name"
        on:click={navigateToProject.bind(null, $projectSearchStore.data)}>
        <span class="project-name"
          >{$projectSearchStore.data.metadata.name}</span>
        <FollowToggle disabled following={true} />
      </h3>

      <p style="margin-bottom: 1rem;">
        {$projectSearchStore.data.metadata.description}
      </p>

      <ProjectStats
        branches={$projectSearchStore.data.stats.branches}
        commits={$projectSearchStore.data.stats.commits}
        contributors={$projectSearchStore.data.stats.contributors} />
    </div>
  {:else if $projectSearchStore.status === remote.Status.Error}
    <div class="result" out:fade|local={{ duration: 100 }}>
      <h3 class="header">
        <CopyableIdentifier
          value={sanitizedSearchQuery}
          kind="projectUrn"
          showIcon={false} />
        <FollowToggle on:follow={follow} style="margin-left: 1rem;" />
      </h3>

      <p>
        You’re not following this project yet, so there’s nothing to show here.
        Follow it and you’ll be notified as soon as it’s available.
      </p>
    </div>
  {:else if validationState.type === "invalid"}
    <div class="result" out:fade|local={{ duration: 100 }}>
      <p>{validationState.message}</p>
    </div>
  {/if}
</div>
