<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onDestroy } from "svelte";

  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as error from "ui/src/error";
  import type { Project } from "ui/src/project";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import {
    inputStore,
    projectRequest as request,
    projectSearch as store,
    reset,
    requestProject,
    searchProject,
  } from "ui/src/search";
  import { ValidationStatus } from "ui/src/validation";
  import { urnValidationStore } from "ui/src/urn";

  import {
    FollowToggle,
    Icon,
    Identifier,
    Remote,
    TextInput,
  } from "ui/DesignSystem";

  let value: string;
  $: value = $inputStore.trim();
  $: storeValue = $store;

  const urnValidation = urnValidationStore();

  onDestroy(() => {
    $inputStore = "";
  });

  const navigateToProject = (project: Project) => {
    reset();
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
    });
    modal.hide();
  };
  const onKeydown = (event: KeyboardEvent) => {
    switch (event.code) {
      case "Enter":
        if (storeValue.status === remote.Status.Success) {
          navigateToProject(storeValue.data);
        } else if (storeValue.status === remote.Status.Error) {
          follow();
        }
        break;
      case "Escape":
        reset();
        modal.hide();
        break;
    }
  };
  const follow = () => {
    if ($urnValidation.status === ValidationStatus.Success) {
      requestProject(value);
    }
  };

  // Validate input entered, at the moment valid RadUrns are the only acceptable input.
  $: if (value && value.length > 0) {
    urnValidation.validate(value);
  } else {
    urnValidation.reset();
  }
  // To support quick pasting, request the urn once valid to get tracking information.
  $: if ($urnValidation.status === ValidationStatus.Success) {
    searchProject(value);
  }
  // Reset searches if the input became invalid.
  $: if ($urnValidation.status !== ValidationStatus.Success) {
    reset();
  }
  // Fire notification when a request has been created.
  $: if ($request.status === remote.Status.Success) {
    reset();
    router.push({ type: "profile", activeTab: "following" });
    notification.info({
      message: "You’ll be notified when this project has been found.",
    });
    modal.hide();
  }

  $: if ($request.status === remote.Status.Error) {
    error.show($request.error);
  }

  $: tracked = $store.status === remote.Status.Success;
  $: untracked = $store.status === remote.Status.Error;
</script>

<style>
  .container {
    width: 26.5rem;
  }

  .search-bar {
    margin-bottom: 1rem;
    position: relative;
    border-radius: 0.5rem;
  }

  .result {
    background: var(--color-background);
    border-radius: 0.5rem;
    height: 0;
    overflow: hidden;
    transition: height 0.3s linear;
    box-shadow: var(--color-shadows);
  }

  .tracked {
    height: 5rem;
  }

  .untracked {
    height: 11rem;
  }

  .header {
    align-items: center;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .id {
    color: var(--color-foreground-level-6);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

<div class="container" data-cy="search-modal">
  <div class="search-bar">
    <TextInput
      autofocus
      bind:value={$inputStore}
      dataCy="search-input"
      inputStyle="height: 3rem; color: var(--color-foreground-level-6); border-radius: 0.5rem; border: 0; box-shadow: var(--color-shadows);"
      on:keydown={onKeydown}
      placeholder="Enter a project’s Radicle ID here…"
      showLeftItem
      validation={$urnValidation}>
      <div slot="left">
        <Icon.MagnifyingGlass />
      </div>
    </TextInput>
  </div>

  <div class="result" class:tracked class:untracked>
    <Remote {store} let:data={project}>
      <div style="padding: 1.5rem;">
        <div
          data-cy="project-name"
          class="header typo-header-3"
          on:click={_ev => navigateToProject(project)}>
          <span class="id">{project.metadata.name}</span>
        </div>
      </div>

      <div slot="error" style="padding: 1.5rem;">
        <div class="header typo-header-3">
          <Identifier {value} kind="radicleId" showIcon={false} />
          <FollowToggle on:follow={follow} style="margin-left: 1rem;" />
        </div>

        <p style="color: var(--color-foreground-level-6);">
          You’re not following this project yet, so there’s nothing to show
          here. Follow it and you’ll be notified as soon as it’s available.
        </p>
      </div>
    </Remote>
  </div>
</div>
