<script>
  import validatejs from "validate.js";
  import { gql } from "apollo-boost";
  import { getClient, query, mutate } from "svelte-apollo";
  import { pop, push } from "svelte-spa-router";

  import { showNotification } from "../stores/notification.js";
  import * as path from "../lib/path.js";
  import { hash } from "../lib/hash.js";
  import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "../config.js";
  import { slide } from "svelte/transition";

  import { Button, Icon, Input, Text } from "../DesignSystem/Primitives";
  import { ModalLayout } from "../DesignSystem/Components";

  let currentSelection;

  const NEW = "new";
  const EXISTING = "existing";

  $: isNew = currentSelection === NEW;
  $: isExisting = currentSelection === EXISTING;

  let name;
  let description = "";
  let defaultBranch = DEFAULT_BRANCH_FOR_NEW_PROJECTS;
  let publish = true;
  let newRepositoryPath = "";
  let existingRepositoryPath = "";
  let imageUrl = "";

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");

  let validations = false;
  let beginValidation = false;

  validatejs.options = {
    fullMessages: false
  };

  const isEmpty = v => {
    return ["", null, undefined].includes(v);
  };

  validatejs.validators.optional = (value, options) => {
    return !isEmpty(value) ? validatejs.single(value, options) : null;
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

    if (isEmpty(value)) {
      return "Pick a directory for the new project";
    }

    if (!localBranchesError.match("could not find repository")) {
      return "The directory should not contain an existing repository";
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

    if (isEmpty(value)) {
      return "Pick an existing repository for the new project";
    }

    if (localBranchesError.match("could not find repository")) {
      return "The directory should contain a valid git repository";
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
        pattern: VALID_NAME_MATCH,
        message: "Project name should match [a-z0-9][a-z0-9_-]+"
      }
    },
    imageUrl: {
      optional: {
        url: {
          schemes: ["http", "https"],
          message: "Not a valid avatar URL",
          allowLocal: false
        }
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
        imageUrl: imageUrl,
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
    imageUrl,
    currentSelection,
    newRepositoryPath,
    existingRepositoryPath
  );

  const client = getClient();

  const LOCAL_BRANCHES = gql`
    query($path: String!) {
      localBranches(path: $path)
    }
  `;

  const CREATE_PROJECT = gql`
    mutation($metadata: MetadataInput!, $path: String!, $publish: Boolean!) {
      createProject(metadata: $metadata, path: $path, publish: $publish) {
        id
        metadata {
          name
        }
      }
    }
  `;

  const createProject = async () => {
    beginValidation = true;
    validate();

    if (validations !== undefined) {
      return;
    }

    let response;

    try {
      response = await mutate(client, {
        mutation: CREATE_PROJECT,
        variables: {
          metadata: {
            name: name,
            description: description,
            imgUrl:
              imageUrl ||
              `https://avatars.dicebear.com/v2/jdenticon/${hash(
                name + description
              )}.svg`,
            defaultBranch: defaultBranch
          },
          path: isNew ? newRepositoryPath : existingRepositoryPath,
          publish: isNew ? true : publish
        }
      });

      push(path.projectOverview(response.data.createProject.id));
      showNotification({
        text: `Project ${response.data.createProject.metadata.name} successfully created`,
        level: "info"
      });
    } catch (error) {
      push(path.projects());
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
      const response = await query(client, {
        query: LOCAL_BRANCHES,
        variables: {
          path: path
        }
      });

      const result = await response.result();
      localBranches = result.data.localBranches;
    } catch (error) {
      localBranchesError = error.message;
    }

    // Now that we have a response with potential branches or an error from the
    // backend, we can perform path validation.
    validate();
  };

  // Re-fetch branches whenever the user selects a new path.
  $: fetchBranches(isNew ? newRepositoryPath : existingRepositoryPath);
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    margin: 92px 0 72px 0;
  }

  .create-project {
    text-align: left;
    width: 540px;
  }

  .option {
    border: 1px solid var(--color-lightgray);
    margin-bottom: 16px;
    border-radius: 4px;
  }

  .option.active {
    box-shadow: 0 0 0 1px var(--color-pink);
    border: 1px solid var(--color-pink);
  }

  .option:hover {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-pink);
    border: 1px solid var(--color-pink);
  }

  .option-header {
    display: flex;
    justify-content: space-between;
    height: 72px;
    align-items: center;
    padding: 0 24px 0 24px;
    cursor: pointer;
    user-select: none;
  }

  .option-body {
    border-top: 1px solid var(--color-lightgray);
    background-color: var(--color-almostwhite);
    padding: 16px 22px 24px 22px;
    border-radius: 0 0 4px 4px;
  }

  .button-row {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 32px;
  }

  .default-branch-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .publish-row {
    display: flex;
    align-items: center;
  }

  .footnote {
    display: flex;
    flex: 1;
    justify-content: flex-start;
    align-items: flex-start;
    padding-left: 15px;
  }

  .validation-row {
    display: flex;
    align-items: center;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper" data-cy="create-project">
    <div class="create-project">
      <Text variant="bigTitle" style="margin-bottom: 32px; text-align: left">
        Create a new project
      </Text>

      <Input.Text
        style="--focus-outline-color: var(--color-pink)"
        placeholder="Project name*"
        dataCy="name"
        bind:value={name}
        valid={!(validations && validations.name)}
        errorMessage={validations && validations.name && validations.name[0]} />

      <Input.Text
        style="margin-top: 16px; margin-bottom: 16px; --focus-outline-color:
        var(--color-pink)"
        placeholder="Project description"
        bind:value={description} />

      <Input.Text
        dataCy="avatar-url"
        style="--focus-outline-color: var(--color-pink)"
        placeholder="http://my-project-website.com/project-avatar.png"
        bind:value={imageUrl}
        valid={!(validations && validations.imageUrl)}
        errorMessage={validations && validations.imageUrl && validations.imageUrl[0]} />

      <Text variant="title" style="margin: 16px 0 12px 16px; text-align: left">
        Select one:
      </Text>

      <div class="radio-selector">
        <div class="option" class:active={isNew} data-cy="new-project">
          <div class="option-header" on:click={() => (currentSelection = NEW)}>
            <Text variant="title" style="color: var(--color-darkgray)">
              Start with a new repository
            </Text>
            <Icon.CheckCircle
              style={isNew ? 'display: block; fill: var(--color-pink)' : 'display: none'} />
          </div>
          {#if isNew}
            <div class="option-body" in:slide>
              <Text style="margin-bottom: 12px; color: var(--color-darkgray)">
                Choose where you'd like to create the repository
              </Text>
              <Input.Directory
                valid={!(validations && validations.newRepositoryPath)}
                errorMessage={validations && validations.newRepositoryPath && validations.newRepositoryPath[0]}
                placeholder="~/path/to/folder"
                bind:path={newRepositoryPath} />
            </div>
          {/if}
        </div>

        <div
          class="option"
          class:active={isExisting}
          data-cy="existing-project">
          <div
            class="option-header"
            on:click={() => (currentSelection = EXISTING)}>
            <Text variant="title" style="color: var(--color-darkgray)">
              Continue with an existing repository
            </Text>
            <Icon.CheckCircle
              style={isExisting ? 'display: block; fill: var(--color-pink)' : 'display: none'} />
          </div>
          {#if isExisting}
            <div class="option-body" in:slide>
              <Text style="margin-bottom: 12px; color: var(--color-darkgray)">
                Choose the existing repository
              </Text>
              <Input.Directory
                placeholder="~/path/to/folder"
                valid={!(validations && validations.existingRepositoryPath)}
                errorMessage={validations && validations.existingRepositoryPath && validations.existingRepositoryPath[0]}
                bind:path={existingRepositoryPath} />
              <div class="default-branch-row" style="margin-top: 16px">
                <Text style="color: var(--color-darkgray)">
                  Select the default branch
                </Text>
                {#if localBranches.length > 0}
                  <Input.Dropdown
                    items={localBranches}
                    bind:value={defaultBranch}
                    style=" min-width: 240px; --focus-outline-color:
                    var(--color-pink)" />
                {:else}
                  <Input.Dropdown
                    items={[DEFAULT_BRANCH_FOR_NEW_PROJECTS]}
                    disabled
                    style="min-width: 240px" />
                {/if}
              </div>
              <div class="publish-row">
                <Input.Checkbox bind:checked={publish}>
                  Publish the {defaultBranch} branch to the network
                </Input.Checkbox>
              </div>
            </div>
          {/if}
        </div>
      </div>

      {#if validations && validations.currentSelection}
        <div class="validation-row">
          <Icon.Important style="margin-right: 8px;fill: var(--color-red)" />
          <Text variant="title" style="color: var(--color-red)">
            {validations.currentSelection[0]}
          </Text>
        </div>
      {/if}

      <div class="button-row">
        <div class="footnote">
          <Text variant="smallText" style="color: var(--color-gray)">
            * required
          </Text>
        </div>
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
  </div>
</ModalLayout>
