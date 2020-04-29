<script>
  import { pop, push } from "svelte-spa-router";
  import validatejs from "validate.js";

  import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "../config.js";
  import * as path from "../lib/path.js";
  import { showNotification } from "../store/notification.js";
  import { create } from "../src/project.ts";
  import { getLocalBranches } from "../src/source.ts";
  import { getValidationState } from "../src/validation.ts";

  import { ModalLayout, RadioOption } from "../DesignSystem/Component";
  import {
    Button,
    Flex,
    Icon,
    Input,
    Text,
    Title
  } from "../DesignSystem/Primitive";

  let currentSelection;

  const NEW = "new";
  const EXISTING = "existing";

  const projectNameMatch = "^[a-z0-9][a-z0-9_-]+$";

  $: isNew = currentSelection === NEW;
  $: isExisting = currentSelection === EXISTING;

  let name;
  let description = "";
  let defaultBranch = DEFAULT_BRANCH_FOR_NEW_PROJECTS;
  let newRepositoryPath = "";
  let existingRepositoryPath = "";

  let validations = false;
  let beginValidation = false;

  validatejs.options = {
    fullMessages: false
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

    if (!localBranchesError.match("could not find repository")) {
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

    if (localBranchesError.match("could not find repository")) {
      return "The directory should contain a git repository";
    }

    if (
      localBranches.includes("rad/rad/contributor") ||
      localBranches.includes("rad/rad/project")
    ) {
      return "This repository is already managed by Radicle";
    }
  };

  const constraints = {
    name: {
      presence: {
        message: "Project name is required",
        allowEmpty: false
      },
      format: {
        pattern: new RegExp(projectNameMatch, "i"),
        message: `Project name should match ${projectNameMatch}`
      }
    },
    currentSelection: {
      presence: {
        message:
          "Select whether to start a new repository or use an existing one"
      }
    },
    newRepositoryPath: {
      validateNewRepositoryPath: true
    },
    existingRepositoryPath: {
      validateExistingRepositoryPath: true
    }
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
        existingRepositoryPath: existingRepositoryPath
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
      response = await create(
        {
          name,
          description,
          defaultBranch
        },
        isNew ? newRepositoryPath : existingRepositoryPath
      );

      push(path.projectSource(response.id));
      showNotification({
        text: `Project ${response.metadata.name} successfully created`,
        level: "info"
      });
    } catch (error) {
      push(path.profile());
      showNotification({
        text: "Could not create project",
        level: "error"
      });
    }
  };

  let localBranches;
  let localBranchesError;

  const fetchBranches = async path => {
    // Revert to defaults whenever the path changes in case this query fails
    // or the user clicks cancel in the directory selection dialog.
    localBranches = "";
    localBranchesError = "";
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
      localBranches = await getLocalBranches(path);
    } catch (error) {
      localBranchesError = error.message;
    }

    // Now that we have a response with potential branches or an error from the
    // backend, we can perform path validation.
    validate();
  };

  // Re-fetch branches whenever the user selects a new path.
  $: fetchBranches(isNew ? newRepositoryPath : existingRepositoryPath);

  $: nameValidation = getValidationState("name", validations);
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    margin: 92px 0 72px 0;
  }

  .create-project {
    text-align: center;
    flex: 1;
  }

  .double-button {
    display: flex;
    flex-direction: row;
  }

  .default-branch-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 16px;
  }

  .validation-row {
    display: flex;
    align-items: center;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper" data-cy="create-project">
    <div class="create-project">
      <Title variant="big" style="margin-bottom: 32px;">
        Create a new project
      </Title>

      <Input.Text
        placeholder="Project name*"
        dataCy="name"
        bind:value={name}
        validation={nameValidation} />

      <Input.Text
        style="margin-top: 16px; margin-bottom: 16px;"
        placeholder="Project description"
        bind:value={description} />

      <Title style="margin: 16px 0 12px 16px; text-align: left">
        Select one:
      </Title>

      <div class="radio-selector">
        <RadioOption
          title="Start with a new repository"
          active={isNew}
          on:click={() => (currentSelection = NEW)}
          dataCy="new-project">
          <div slot="option-body">
            <Text
              style="margin-bottom: 12px; color:
              var(--color-foreground-level-6); text-align: left">
              Choose where you'd like to create the repository
            </Text>
            <Input.Directory
              valid={!(validations && validations.newRepositoryPath)}
              validationMessage={validations && validations.newRepositoryPath && validations.newRepositoryPath[0]}
              placeholder="~/path/to/folder"
              bind:path={newRepositoryPath} />
          </div>
        </RadioOption>

        <RadioOption
          title="Continue with an existing repository"
          active={isExisting}
          on:click={() => (currentSelection = EXISTING)}
          dataCy="existing-project">
          <div slot="option-body">
            <Text
              style="margin-bottom: 12px; color:
              var(--color-foreground-level-6); text-align:left">
              Choose an existing repository
            </Text>
            <Input.Directory
              placeholder="~/path/to/folder"
              valid={!(validations && validations.existingRepositoryPath)}
              validationMessage={validations && validations.existingRepositoryPath && validations.existingRepositoryPath[0]}
              bind:path={existingRepositoryPath} />
            <div class="default-branch-row">
              <Text style="color: var(--color-foreground-level-6)">
                Select the default branch
              </Text>
              {#if localBranches.length > 0}
                <Input.Dropdown
                  items={localBranches}
                  bind:value={defaultBranch}
                  style="min-width: 240px; --focus-outline-color:
                  var(--color-primary)" />
              {:else}
                <Input.Dropdown
                  items={[DEFAULT_BRANCH_FOR_NEW_PROJECTS]}
                  disabled
                  style="min-width: 240px" />
              {/if}
            </div>
          </div>
        </RadioOption>
      </div>

      {#if validations && validations.currentSelection}
        <div class="validation-row">
          <Icon.Important
            style="margin-right: 8px;fill: var(--color-negative)" />
          <Title style="color: var(--color-negative)">
            {validations.currentSelection[0]}
          </Title>
        </div>
      {/if}

      <Flex style="margin-top: 32px">
        <div slot="left">
          <Text
            variant="tiny"
            style="color: var(--color-foreground-level-5); padding-left: 15px;">
            * required
          </Text>
        </div>
        <div slot="right">
          <div class="double-button">
            <Button
              dataCy="cancel-button"
              variant="transparent"
              on:click={pop}
              style="margin-right: 24px;">
              Cancel
            </Button>
            <Button
              dataCy="create-project-button"
              disabled={!(name && currentSelection)}
              variant="primary"
              on:click={createProject}>
              Create project
            </Button>
          </div>
        </div>
      </Flex>
    </div>
  </div>
</ModalLayout>
