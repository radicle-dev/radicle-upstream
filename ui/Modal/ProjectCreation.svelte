<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";
  import { push } from "svelte-spa-router";

  import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "../src/config";
  import { Variant as IllustrationVariant } from "../src/illustration";
  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import * as remote from "../src/remote";
  import * as urn from "../src/urn";
  import {
    clearLocalState,
    create,
    defaultBranch,
    localState,
    nameValidationStore,
    formatNameInput,
    extractName,
    repositoryPathValidationStore,
    RepoType,
  } from "../src/project";
  import * as testing from "../src/project";
  import { ValidationStatus } from "../src/validation";
  import * as screen from "../src/screen";
  import type { Settings } from "../src/settings";
  import {
    dismissRemoteHelperHint,
    fetch as fetchSession,
    settings,
  } from "../src/session";
  import type { LocalState } from "../src/source";

  import { Button, Flex, Input } from "../DesignSystem/Primitive";
  import {
    Dropdown,
    Illustration,
    RadioOption,
    RemoteHelperHint,
    Tooltip,
  } from "../DesignSystem/Component";
  import { CSSPosition } from "../src/style";

  let currentSelection: RepoType;
  let nameInput: HTMLInputElement;
  // export let content;

  const dispatch = createEventDispatcher();

  $: isNew = currentSelection === RepoType.New;
  $: isExisting = currentSelection === RepoType.Existing;

  let name = "";
  let description = "";
  let newRepositoryPath = "";
  let existingRepositoryPath = "";

  let nameValidation = nameValidationStore();

  let loading = false;

  const setCurrentSelection = (type: RepoType) => {
    currentSelection = type;
    // Reset the name validation on selection switch
    nameValidation = nameValidationStore();
  };

  const createProject = async () => {
    let response;

    try {
      loading = true;
      screen.lock();

      response = await create({
        description,
        defaultBranch: $defaultBranch,
        repo: isNew
          ? { type: RepoType.New, name, path: newRepositoryPath }
          : { type: RepoType.Existing, path: existingRepositoryPath },
      });

      // Re-fetch session so we have the right permissions to enable the
      // project registration button rithout a page-reload.
      fetchSession();

      push(path.projectSource(response.id));
      notification.info(
        `Project ${response.metadata.name} successfully created`
      );
    } catch (error) {
      push(path.profileProjects());
      notification.error(
        `Could not create project: ${urn.shorten(error.message)}`
      );
    } finally {
      dispatch("hide");
      loading = false;
      screen.unlock();
    }
  };

  // We unlock the screen already after the request, this is just a fail-safe
  // to make sure the screen gets unlocked in any case when the component gets
  // destroyed.
  onDestroy(() => {
    clearLocalState();
    screen.unlock();
  });

  $: pathValidation = repositoryPathValidationStore(isNew);

  $: repositoryPath = isNew ? newRepositoryPath : existingRepositoryPath;
  $: if (repositoryPath.length > 0 || (currentSelection && name.length > 0))
    pathValidation.validate(repositoryPath);

  $: {
    if (name.length > 0) {
      name = formatNameInput(name);
      nameValidation.validate(name);
    }
  }

  // Use the directory name for existing projects as the project name.
  $: name = extractName(existingRepositoryPath);

  // Reset the project name when switching between new and existing repo.
  $: isExisting && (name = "");

  // The presence check is outside the validations since we don't want to show the validation error message for it.
  $: validName =
    name.length > 0 && $nameValidation.status === ValidationStatus.Success;

  $: disableSubmit =
    !validName ||
    $pathValidation.status !== ValidationStatus.Success ||
    loading;

  $: localStateStore = $localState;
  $: localBranches =
    localStateStore.status === remote.Status.Success
      ? localStateStore.data.branches
      : [];

  $: showRemoteHelper =
    $settings && ($settings as Settings).appearance.hints.showRemoteHelper;
</script>

<style>
  .container {
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
    padding: 3rem 2rem 2rem 2rem;
  }

  .create-project {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .radio-selector {
    margin-bottom: 2rem;
  }

  .double-button {
    display: grid;
    grid-template-columns: auto auto;
    grid-column-gap: 1rem;
  }

  .default-branch-row {
    display: flex;
    align-items: center;
    margin-top: 1rem;
  }
</style>

<div class="container" data-cy="page">
  <div class="create-project" data-cy="create-project">
    <Illustration
      style="align-self: center; margin-bottom: 1rem;"
      variant={IllustrationVariant.Star} />
    <h2 style="margin-bottom: 3rem;">Start a new project</h2>

    <div class="radio-selector">
      <RadioOption
        title="Create a new repository"
        active={isNew}
        on:click={ev => {
          ev.stopPropagation();
          setCurrentSelection(RepoType.New);
        }}
        dataCy="new-project">
        <div slot="option-body">
          <Input.Directory
            placeholder="Where to create the repository"
            validation={$pathValidation}
            bind:path={newRepositoryPath}
            on:selected={() => nameInput.focus()} />
          <p
            style="margin-top: 1rem; color: var(--color-foreground-level-6);
            text-align: center">
            A new repository will be created on your machine inside a directory
            and named after the project name.
          </p>
        </div>
      </RadioOption>

      <RadioOption
        title="Continue with an existing repository"
        active={isExisting}
        on:click={ev => {
          ev.stopPropagation();
          setCurrentSelection(RepoType.Existing);
        }}
        dataCy="existing-project">
        <div slot="option-body">
          <Input.Directory
            placeholder="Choose an existing repository"
            validation={$pathValidation}
            bind:path={existingRepositoryPath} />
          <div class="default-branch-row">
            <p
              style="margin-right: 1rem; color: var(--color-foreground-level-6)">
              Default branch
            </p>
            {#if localBranches.length > 0}
              <Dropdown
                style="max-width: 22.9rem;"
                options={localBranches.map(branch => ({
                  variant: 'text',
                  value: branch,
                  title: branch,
                }))}
                bind:value={$defaultBranch} />
            {:else}
              <Dropdown
                style="max-width: 22.9rem;"
                placeholder={DEFAULT_BRANCH_FOR_NEW_PROJECTS}
                options={[]}
                disabled />
            {/if}
          </div>
          <p
            style="margin-top: 1rem; color: var(--color-foreground-level-6);
            text-align: left;">
            This will publish the chosen repository to the Radicle network.
          </p>
        </div>
      </RadioOption>
      {#if showRemoteHelper}
        <RemoteHelperHint on:hide={dismissRemoteHelperHint} />
      {/if}
    </div>

    <Tooltip
      value={isExisting ? 'The project name is taken from the the repository you selected' : ''}
      position={CSSPosition.Top}>
      <Input.Text
        placeholder="Project name*"
        dataCy="name"
        bind:value={name}
        bind:inputElement={nameInput}
        validation={$nameValidation}
        disabled={isExisting} />
    </Tooltip>

    <Input.Text
      dataCy="description"
      style="margin-top: 1rem; margin-bottom: 1rem;"
      placeholder="Project description"
      bind:value={description} />

    <Flex style="margin-top: 1rem">
      <div slot="right">
        <div class="double-button">
          <Button
            dataCy="cancel-button"
            variant="transparent"
            on:click={() => dispatch('hide')}>
            Cancel
          </Button>
          <Button
            dataCy="create-project-button"
            disabled={disableSubmit}
            variant="primary"
            on:click={createProject}>
            Create project
          </Button>
        </div>
      </div>
    </Flex>
  </div>
</div>
