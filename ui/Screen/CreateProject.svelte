<script>
  import validatejs from "validate.js";
  import { gql } from "apollo-boost";
  import { getClient, query, mutate } from "svelte-apollo";
  import { pop, push } from "svelte-spa-router";

  import { showNotification } from "../store/notification.js";
  import * as path from "../lib/path.js";
  import { hash } from "../lib/hash.js";
  import { SINGLE_WORD_MATCH } from "../lib/validationHelpers.js";
  import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "../config.js";

  import {
    Button,
    Flex,
    Icon,
    Input,
    Text,
    Title
  } from "../DesignSystem/Primitive";
  import { ModalLayout, RadioOption } from "../DesignSystem/Component";

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

    if (validatejs.isEmpty(value)) {
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
        pattern: SINGLE_WORD_MATCH,
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
    mutation(
      $metadata: ProjectMetadataInput!
      $path: String!
      $publish: Boolean!
    ) {
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

    if (!validatejs.isEmpty(validations)) {
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
    margin-bottom: 24px;
  }

  .publish-row {
    display: flex;
    align-items: center;
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
        style="--focus-outline-color: var(--color-pink)"
        placeholder="Project name*"
        dataCy="name"
        bind:value={name}
        valid={!(validations && validations.name)}
        validationMessage={validations && validations.name && validations.name[0]} />

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
        validationMessage={validations && validations.imageUrl && validations.imageUrl[0]} />

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
              style="margin-bottom: 12px; color: var(--color-darkgray);
              text-align: left">
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
            <Text style="margin-bottom: 12px; color: var(--color-darkgray)">
              Choose the existing repository
            </Text>
            <Input.Directory
              placeholder="~/path/to/folder"
              valid={!(validations && validations.existingRepositoryPath)}
              validationMessage={validations && validations.existingRepositoryPath && validations.existingRepositoryPath[0]}
              bind:path={existingRepositoryPath} />
            <div class="default-branch-row" style="margin-top: 16px">
              <Text style="color: var(--color-darkgray)">
                Select the default branch
              </Text>
              {#if localBranches.length > 0}
                <Input.Dropdown
                  items={localBranches}
                  bind:value={defaultBranch}
                  style="min-width: 240px; --focus-outline-color:
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
        </RadioOption>
      </div>

      {#if validations && validations.currentSelection}
        <div class="validation-row">
          <Icon.Important style="margin-right: 8px;fill: var(--color-red)" />
          <Title style="color: var(--color-red)">
            {validations.currentSelection[0]}
          </Title>
        </div>
      {/if}

      <Flex style="margin-top: 32px">
        <div slot="left">
          <Text
            variant="tiny"
            style="color: var(--color-gray); padding-left: 15px;">
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
