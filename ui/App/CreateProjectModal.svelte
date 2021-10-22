<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { onDestroy } from "svelte";

  import * as notification from "ui/src/notification";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import {
    UPSTREAM_DEFAULT_BRANCH,
    clearLocalState,
    defaultBranch,
    defaultBranchForNewRepository,
    descriptionValidationStore,
    extractName,
    formatNameInput,
    localState,
    nameValidationStore,
    repositoryPathValidationStore,
  } from "ui/src/project";
  import * as proxy from "ui/src/proxy";
  import { ValidationStatus } from "ui/src/validation";
  import * as screen from "ui/src/screen";

  import {
    Button,
    Dropdown,
    DirectoryInput,
    RadioOption,
    TextInput,
    Tooltip,
  } from "ui/DesignSystem";

  import RemoteHelperHint from "ui/App/SharedComponents/RemoteHelperHint.svelte";
  import Modal from "ui/App/ModalLayout/Modal.svelte";

  type RepoType = "new" | "existing";

  let currentSelection: RepoType;
  let nameInput: TextInput;

  let startValidations = false;

  $: isNew = currentSelection === "new";
  $: isExisting = currentSelection === "existing";

  let name = "";
  let description = "";
  let newRepositoryPath = "";
  let existingRepositoryPath = "";

  let nameValidation = nameValidationStore();
  let descriptionValidation = descriptionValidationStore();

  let loading = false;

  const setCurrentSelection = (type: RepoType) => {
    currentSelection = type;
    // Reset validations on selection switch
    nameValidation = nameValidationStore();
    descriptionValidation = descriptionValidationStore();
  };

  const createProject = async () => {
    screen.withLock(async () => {
      try {
        loading = true;

        const response = await proxy.client.project.create({
          description,
          defaultBranch: isNew
            ? await defaultBranchForNewRepository()
            : $defaultBranch,
          repo: isNew
            ? { type: "new", name, path: newRepositoryPath }
            : { type: "existing", path: existingRepositoryPath },
        });

        router.push({
          type: "project",
          params: {
            urn: response.urn,
            activeView: { type: "files" },
          },
        });
        notification.info({
          message: `Project ${response.metadata.name} was created!`,
        });
      } catch (err: unknown) {
        router.push({ type: "profile" });
        let message;
        if (err instanceof proxy.ResponseError) {
          message = `Could not create project: ${err.message}`;
        } else {
          message = `Could not create project`;
        }
        error.show(
          new error.Error({
            code: error.Code.ProjectCreationFailure,
            message,
            source: err,
          })
        );
      } finally {
        modal.hide();
        loading = false;
      }
    });
  };

  // We unlock the screen already after the request, this is just a fail-safe
  // to make sure the screen gets unlocked in any case when the component gets
  // destroyed.
  onDestroy(() => {
    clearLocalState();
  });

  $: pathValidation = repositoryPathValidationStore(isNew);

  $: repositoryPath = isNew ? newRepositoryPath : existingRepositoryPath;

  $: if (repositoryPath.length > 0 || (currentSelection && name.length > 0)) {
    startValidations = true;
  }

  $: name = formatNameInput(name);

  $: if (startValidations) {
    nameValidation.validate(name);
    descriptionValidation.validate(description);
    pathValidation.validate(repositoryPath);
  }

  // Use the directory name for existing projects as the project name.
  $: name = extractName(existingRepositoryPath);

  // Reset the project name when switching between new and existing repo.
  $: isExisting && (name = "");

  // The presence check is outside the validations since we don't want to show the validation error message for it.
  $: validName =
    name.length > 0 && $nameValidation.status === ValidationStatus.Success;

  $: validDescription =
    description.length === 0 ||
    (description.length > 0 &&
      $descriptionValidation.status === ValidationStatus.Success);

  $: disableSubmit =
    !validName ||
    !validDescription ||
    $pathValidation.status !== ValidationStatus.Success ||
    loading;

  $: localStateStore = $localState;
  $: localBranches =
    localStateStore.status === remote.Status.Success
      ? localStateStore.data.branches
      : [];
</script>

<style>
  .create-project {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    width: 100%;
  }

  .radio-selector {
    margin-bottom: 2rem;
  }

  .default-branch-row {
    display: flex;
    align-items: center;
    margin-top: 1rem;
  }
</style>

<Modal dataCy="create-project-modal" emoji="🌠" title="Start a new project">
  <div class="create-project">
    <div class="radio-selector">
      <RadioOption
        title="Create a new repository"
        active={isNew}
        on:click={ev => {
          ev.stopPropagation();
          setCurrentSelection("new");
        }}
        dataCy="new-project">
        <div slot="option-body">
          <DirectoryInput
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
          setCurrentSelection("existing");
        }}
        dataCy="existing-project">
        <div slot="option-body">
          <DirectoryInput
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
                dataCy="default-branch"
                style="max-width: 22.9rem;"
                options={localBranches.map(branch => ({
                  variant: "text",
                  value: branch,
                  title: branch,
                }))}
                bind:value={$defaultBranch} />
            {:else}
              <Dropdown
                dataCy="default-branch"
                style="max-width: 22.9rem;"
                placeholder={UPSTREAM_DEFAULT_BRANCH}
                options={[]}
                disabled />
            {/if}
          </div>
          <p
            style="margin-top: 1rem; color: var(--color-foreground-level-6);
            text-align: left;">
            This will publish your repository to the Radicle network.
          </p>
        </div>
      </RadioOption>
      <RemoteHelperHint />
    </div>

    <Tooltip
      value={isExisting
        ? "The project’s name is taken from the chosen repository"
        : ""}
      position="top">
      <TextInput
        placeholder="Project name*"
        dataCy="name"
        bind:value={name}
        bind:this={nameInput}
        validation={$nameValidation}
        disabled={isExisting} />
    </Tooltip>

    <TextInput
      dataCy="description"
      style="margin-top: 1rem; margin-bottom: 1rem;"
      placeholder="Project description"
      validation={$descriptionValidation}
      bind:value={description} />
  </div>

  <svelte:fragment slot="buttons">
    <Button
      dataCy="cancel-button"
      variant="transparent"
      on:click={() => modal.hide()}>
      Cancel
    </Button>
    <Button
      dataCy="create-project-button"
      disabled={disableSubmit}
      variant="primary"
      on:click={createProject}>
      Create project
    </Button>
  </svelte:fragment>
</Modal>
