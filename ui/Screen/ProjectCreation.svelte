<script>
  import { onMount } from "svelte";
  import { onDestroy } from "svelte";
  import { pop, push } from "svelte-spa-router";
  import validatejs from "validate.js";

  import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "../src/config.ts";
  import { Variant as IllustrationVariant } from "../src/illustration.ts";
  import * as notification from "../src/notification.ts";
  import * as path from "../src/path.ts";
  import { create, RepoType } from "../src/project.ts";
  import { getLocalState } from "../src/source.ts";
  import { getValidationState } from "../src/validation.ts";
  import * as screen from "../src/screen.ts";
  import {
    dismissRemoteHelperHint,
    fetch as fetchSession,
    settings,
  } from "../src/session.ts";

  import { Button, Flex, Icon, Input } from "../DesignSystem/Primitive";
  import {
    Dropdown,
    Illustration,
    RadioOption,
    RemoteHelperHint,
  } from "../DesignSystem/Component";

  let currentSelection;
  export let content;

  const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

  $: isNew = currentSelection === RepoType.New;
  $: isExisting = currentSelection === RepoType.Existing;

  let name;
  let description = "";
  let defaultBranch = DEFAULT_BRANCH_FOR_NEW_PROJECTS;
  let newRepositoryPath = "";
  let existingRepositoryPath = "";

  let validations = false;
  let beginValidation = false;

  let loading = false;

  validatejs.options = {
    fullMessages: false,
  };

  validatejs.validators.optional = (value, options) => {
    return !validatejs.isEmpty(value)
      ? validatejs.single(value, options)
      : null;
  };

  validatejs.validators.validateNewRepositoryPath = (
    value,
    _options,
    _key,
    _attributes
  ) => {
    if (isExisting) {
      return;
    }

    if (validatejs.isEmpty(value)) {
      return "Pick a directory for the new project";
    }

    if (!localStateError.match("could not find repository")) {
      return "The directory should be empty";
    }
  };

  validatejs.validators.validateExistingRepositoryPath = (
    value,
    _options,
    _key,
    _attributes
  ) => {
    if (isNew) {
      return;
    }

    if (validatejs.isEmpty(value)) {
      return "Pick a directory with an existing repository";
    }

    if (localStateError.match("could not find repository")) {
      return "The directory should contain a git repository";
    }
  };

  const constraints = {
    name: {
      presence: {
        message: "Project name is required",
        allowEmpty: false,
      },
      format: {
        pattern: new RegExp(projectNameMatch, "i"),
        message: `Project name should match ${projectNameMatch}`,
      },
    },
    currentSelection: {
      presence: {
        message:
          "Select whether to start a new repository or use an existing one",
      },
    },
    newRepositoryPath: {
      validateNewRepositoryPath: true,
    },
    existingRepositoryPath: {
      validateExistingRepositoryPath: true,
    },
  };

  const validate = () => {
    if (!beginValidation) {
      return;
    }

    validations = validatejs(
      {
        name: name,
        currentSelection: currentSelection,
        newRepositoryPath: newRepositoryPath,
        existingRepositoryPath: existingRepositoryPath,
      },
      constraints
    );
  };

  // Note: the arguments are actually not passed to the function, they are
  // only needed to make the function reactive to when they're changed.
  $: validate(
    name,
    currentSelection,
    newRepositoryPath,
    existingRepositoryPath
  );

  const createProject = async () => {
    beginValidation = true;
    validate();

    if (!validatejs.isEmpty(validations)) {
      return;
    }

    let response;

    try {
      loading = true;
      screen.lock();

      response = await create({
        description,
        defaultBranch,
        repo: isNew
          ? { type: RepoType.New, name, path: newRepositoryPath }
          : { type: RepoType.Existing, path: existingRepositoryPath },
      });

      // Re-fetch session so we have the right permissions to enable the
      // project registration button rithout a page-reload.
      await fetchSession();

      push(path.projectSource(response.id));
      notification.info(
        `Project ${response.metadata.name} successfully created`
      );
    } catch (error) {
      push(path.profileProjects());
      notification.error(
        `Could not create project: ${shortenUrn(error.message)}`
      );
    } finally {
      loading = false;
      screen.unlock();
    }
  };

  // We unlock the screen already after the request, this is just a fail-safe
  // to make sure the screen gets unlocked in any case when the component gets
  // destroyed.
  onDestroy(() => {
    screen.unlock();
  });

  const shortenUrn = string => {
    return string.replace(/(rad:git:[\w]{3})[\w]{53}([\w]{3})/, "$1…$2");
  };

  let localState;
  let localStateError;

  const fetchBranches = async path => {
    // Revert to defaults whenever the path changes in case this query fails
    // or the user clicks cancel in the directory selection dialog.
    localState = "";
    localStateError = "";
    defaultBranch = DEFAULT_BRANCH_FOR_NEW_PROJECTS;

    // This function gets executed even for the first path change on page load
    // which sets the path variable to an empty string. We shouldn't query the
    // backend when the path is not given.
    if (path === "") {
      return;
    }

    // Start validating all the form fields when the user chooses a path.
    beginValidation = true;

    try {
      localState = await getLocalState(path);
      if (!localState.branches.includes(defaultBranch)) {
        defaultBranch = localState.branches[0];
      }
    } catch (error) {
      localStateError = error.message;
    }

    // Now that we have a response with potential branches or an error from the
    // backend, we can perform path validation.
    validate();
  };

  // Re-fetch branches whenever the user selects a new path.
  $: fetchBranches(isNew ? newRepositoryPath : existingRepositoryPath);

  $: nameValidation = getValidationState("name", validations);
  $: newRepositoryPathValidation = getValidationState(
    "newRepositoryPath",
    validations
  );
  $: existingRepositoryPathValidation = getValidationState(
    "existingRepositoryPath",
    validations
  );

  onMount(() => {
    content.focus();
  });

  // Use the directory name for existing projects as the project name.
  $: name = existingRepositoryPath.split("/").slice(-1)[0];

  // Reset the project name when switching between new and existing repo.
  $: isExisting && (name = "");
</script>

<style>
  .container {
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
    padding: 2rem;
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

  .validation-row {
    display: flex;
    align-items: center;
  }
</style>

<div class="container" bind:this={content} data-cy="page">
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
          currentSelection = RepoType.New;
        }}
        dataCy="new-project">
        <div slot="option-body">
          <Input.Directory
            placeholder="Where to create the repository"
            validation={newRepositoryPathValidation}
            bind:path={newRepositoryPath} />
          <p
            style="margin-top: 1rem; color: var(--color-foreground-level-6);
            text-align: center">
            A new repository will be created inside this directory
            <br />
            and named after the project name.
          </p>
        </div>
      </RadioOption>

      <RadioOption
        title="Continue with an existing repository"
        active={isExisting}
        on:click={ev => {
          ev.stopPropagation();
          currentSelection = RepoType.Existing;
        }}
        dataCy="existing-project">
        <div slot="option-body">
          <Input.Directory
            placeholder="Choose an existing repository"
            validation={existingRepositoryPathValidation}
            bind:path={existingRepositoryPath} />
          <div class="default-branch-row">
            <p
              style="margin-right: 1rem; color: var(--color-foreground-level-6)">
              Default branch
            </p>
            {#if localState.branches && localState.branches.length > 0}
              <Dropdown
                style="max-width: 22.9rem;"
                options={localState.branches.map(branch => {
                  return { variant: 'text', value: branch, textProps: { title: branch } };
                })}
                bind:value={defaultBranch} />
            {:else}
              <Dropdown
                style="max-width: 22.9rem;"
                placeholder={[DEFAULT_BRANCH_FOR_NEW_PROJECTS]}
                options={[]}
                disabled />
            {/if}
          </div>
        </div>
      </RadioOption>
      {#if $settings.appearance.hints.showRemoteHelper}
        <RemoteHelperHint on:hide={dismissRemoteHelperHint} />
      {/if}
    </div>

    <Input.Text
      placeholder="Project name*"
      dataCy="name"
      bind:value={name}
      validation={nameValidation}
      disabled={isExisting} />

    <Input.Text
      dataCy="description"
      style="margin-top: 1rem; margin-bottom: 1rem;"
      placeholder="Project description"
      bind:value={description} />

    {#if validations && validations.currentSelection}
      <div class="validation-row">
        <Icon.ExclamationCircle
          style="margin-right: 0.5rem; fill: var(--color-negative)" />
        <p class="typo-text-bold" style="color: var(--color-negative)">
          {validations.currentSelection[0]}
        </p>
      </div>
    {/if}

    <Flex style="margin-top: 2rem">
      <div slot="right">
        <div class="double-button">
          <Button dataCy="cancel-button" variant="transparent" on:click={pop}>
            Cancel
          </Button>
          <Button
            dataCy="create-project-button"
            disabled={!(name && currentSelection) || loading}
            variant="primary"
            on:click={createProject}>
            Create project
          </Button>
        </div>
      </div>
    </Flex>
  </div>
</div>
