<script lang="typescript">
  import { push } from "svelte-spa-router";
  import { createEventDispatcher } from "svelte";

  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import type { Project } from "../src/project";
  import * as remote from "../src/remote";
  import {
    projectRequest as request,
    projectSearch as store,
    reset,
    requestProject,
    searchProject,
    urnValidationStore,
  } from "../src/search";
  import { ValidationStatus } from "../src/validation";

  import { Icon, Input } from "../DesignSystem/Primitive";
  import { FollowToggle, Remote } from "../DesignSystem/Component";

  let id: string;
  let input: string = "";

  let value: string;
  $: value = input.trim();

  const dispatch = createEventDispatcher();
  const urnValidation = urnValidationStore();

  const navigateToProject = (project: Project) => {
    reset();
    push(path.project(project.urn));
    dispatch("hide");
  };
  const onKeydown = (event: KeyboardEvent) => {
    switch (event.code) {
      case "Enter":
        // Navigate to project directly if present.
        if ($store.status === remote.Status.Success) {
          // FIXME(xla): Once remote/Remote offer stronger type guarantees this needs to go.
          navigateToProject(
            ($store as { status: remote.Status.Success; data: Project }).data
          );
        }
        break;
      case "Escape":
        reset();
        dispatch("hide");
        break;
    }
  };
  const follow = () => {
    requestProject(value);
  };

  // Validate input entered, at the moment valid RadUrns are the only acceptable input.
  $: if (value && value.length > 0) {
    urnValidation.validate(value);
    id = value.replace("rad:git:", "");
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
    push(path.profileFollowing());
    notification.info({
      message: "You’ll be notified when this project has been found.",
    });
    dispatch("hide");
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
    <Input.Text
      autofocus
      bind:value={input}
      dataCy="search-input"
      inputStyle="height: 3rem; color: var(--color-foreground-level-6); border-radius: 0.5rem; border: 0; box-shadow: var(--color-shadows);"
      on:keydown={onKeydown}
      placeholder="Enter a project's Radicle ID here…"
      showLeftItem
      validation={$urnValidation}>
      <div slot="left">
        <Icon.MagnifyingGlass />
      </div>
    </Input.Text>
  </div>

  <div class="result" class:tracked class:untracked>
    <Remote {store} let:data={project}>
      <div style="padding: 1.5rem;">
        <div
          class="header typo-header-3"
          on:click={_ev => navigateToProject(project)}>
          <span class="id">{project.metadata.name}</span>
        </div>
      </div>

      <div slot="error" style="padding: 1.5rem;">
        <div class="header typo-header-3">
          <span class="id">{id}</span>
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
