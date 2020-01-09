<script>
  import validatejs from "validate.js";
  import ModalLayout from "../layouts/ModalLayout.svelte";
  import { showNotification } from "../stores.js";
  import { slide } from "svelte/transition";

  import { gql } from "apollo-boost";
  import { getClient, query, mutate } from "svelte-apollo";

  import * as path from "../path.js";

  import { pop, push } from "svelte-spa-router";
  import {
    Button,
    Header,
    Icon,
    Input,
    DirectoryInput,
    CheckboxInput,
    Text,
    Title,
    Select
  } from "../DesignSystem";

  const DEFAULT_BRANCH = "master";

  let currentSelection;

  $: isNew = currentSelection === "new";
  $: isExisting = currentSelection === "existing";

  let name;
  let description = "";
  let defaultBranch = DEFAULT_BRANCH;
  let publish = true;
  let newRepositoryPath = "";
  let existingRepositoryPath = "";
  let imageUrl = "";

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");

  let validations = false;
  let validationStarted = false;

  const validate = () => {
    if (!validationStarted) {
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

  const anonymize = str => {
    return str
      .split("")
      .reduce(
        (prevHash, currVal) =>
          ((prevHash << 5) - prevHash + currVal.charCodeAt(0)) | 0,
        0
      );
  };

  const createProject = async () => {
    validationStarted = true;
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
              `https://avatars.dicebear.com/v2/jdenticon/${anonymize(
                name + description
              )}.svg`,
            defaultBranch: defaultBranch
          },
          path: isNew ? newRepositoryPath : existingRepositoryPath,
          publish: isNew ? true : publish
        }
      });

      push(path.projectOverview(response.data.createProject.id));
      showNotification(
        `Project ${response.data.createProject.metadata.name} successfully created`
      );
    } catch (error) {
      console.log(error);
      push(path.projects());
      showNotification("Could not create project");
    }
  };

  let localBranches;
  let localBranchesError;

  const fetchBranches = async path => {
    // Reset to defaults whenever the path changes so that we show the defaults
    // in case this query fails or the user clicks cancel in the directory
    // selection dialog.
    localBranches = "";
    localBranchesError = "";
    defaultBranch = DEFAULT_BRANCH;

    // This function gets executed even for the first path change which sets
    // the path variable to an empty string on page load. If we don't ignore
    // this then the backend will throw an exception.
    if (path === "") {
      return;
    }
    validationStarted = true;

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
    validate();
  };

  $: fetchBranches(isNew ? newRepositoryPath : existingRepositoryPath);

  function isEmpty(v) {
    return ["", null, undefined].includes(v);
  }

  validatejs.validators.optional = (value, options) => {
    return !isEmpty(value) ? validatejs.single(value, options) : null;
  };

  validatejs.options = {
    fullMessages: false
  };

  validatejs.validators.validateNewRepositoryPath = (
    value,
    options,
    key,
    attributes
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
    options,
    key,
    attributes
  ) => {
    if (isNew) {
      return;
    }

    if (isEmpty(value)) {
      return "Pick a directory for the new project";
    }

    if (localBranches.length < 1) {
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
        message: "Project name is required!",
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
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    margin: 92px 0 72px 0;
  }

  .create-project {
    text-align: center;
    width: 540px;
  }

  .radio-selector {
    text-align: left;
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

  .show {
    display: block;
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
    text-align: left;
    padding-left: 15px;
  }

  .validation-row {
    display: flex;
    align-items: center;
  }

  .validationMessage {
    color: var(--color-red);
    font-size: 14px;
    font-family: "GT America Medium";
    text-align: left;
    margin-top: 4px;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper">
    <div class="create-project">
      <Title.Big style="margin-bottom: 32px; text-align: left">
        Create a new project
      </Title.Big>

      <Input
        style="--focus-outline-color: var(--color-pink)"
        placeholder="Project name*"
        bind:value={name}
        valid={!(validations && validations.name)} />
      {#if validations && validations.name}
        <div class="validationMessage">{validations.name[0]}</div>
      {/if}

      <Input
        style="margin-top: 16px; margin-bottom: 16px; --focus-outline-color:
        var(--color-pink)"
        placeholder="Project description"
        bind:value={description} />

      <Input
        style="--focus-outline-color: var(--color-pink)"
        placeholder="http://my-project-website.com/project-avatar.png"
        bind:value={imageUrl}
        valid={!(validations && validations.imageUrl)} />
      {#if validations && validations.imageUrl}
        <div class="validationMessage">{validations.imageUrl[0]}</div>
      {/if}

      <Title.Regular style="margin: 16px 0 12px 16px; text-align: left">
        Select one:
      </Title.Regular>

      <div class="radio-selector">
        <div class="option" class:active={isNew}>
          <div
            class="option-header"
            on:click={() => (currentSelection = 'new')}>
            <Title.Regular style="color: var(--color-darkgray)">
              Start with a new repository
            </Title.Regular>
            <Icon.CheckCircle
              style={isNew ? 'display: block' : 'display: none'} />
          </div>
          {#if isNew}
            <div class="option-body" in:slide>
              <Text.Regular
                style="margin-bottom: 12px; color: var(--color-darkgray)">
                Choose where you'd like to create the repository
              </Text.Regular>
              <DirectoryInput
                valid={!(validations && validations.newRepositoryPath)}
                placeholder="~/path/to/folder"
                bind:path={newRepositoryPath} />
              {#if validations && validations.newRepositoryPath}
                <div class="validationMessage" style="text-align: left">
                  {validations.newRepositoryPath[0]}
                </div>
              {/if}
            </div>
          {/if}
        </div>

        <div class="option" class:active={isExisting}>
          <div
            class="option-header"
            on:click={() => (currentSelection = 'existing')}>
            <Title.Regular style="color: var(--color-darkgray)">
              Continue with an existing repository
            </Title.Regular>
            <Icon.CheckCircle
              style={isExisting ? 'display: block' : 'display: none'} />
          </div>
          {#if isExisting}
            <div class="option-body" in:slide>
              <Text.Regular
                style="margin-bottom: 12px; color: var(--color-darkgray)">
                Choose the existing repository
              </Text.Regular>
              <DirectoryInput
                placeholder="~/path/to/folder"
                valid={!(validations && validations.existingRepositoryPath)}
                bind:path={existingRepositoryPath} />
              {#if validations && validations.existingRepositoryPath}
                <div class="validationMessage" style="text-align: left">
                  {validations.existingRepositoryPath[0]}
                </div>
              {/if}
              <div class="default-branch-row" style="margin-top: 16px">
                <Text.Regular style="color: var(--color-darkgray)">
                  Select the default branch
                </Text.Regular>
                {#if localBranches.length > 0}
                  <Select
                    items={localBranches}
                    bind:value={defaultBranch}
                    style=" min-width: 240px; --focus-outline-color:
                    var(--color-pink)" />
                {:else}
                  <Select
                    items={[DEFAULT_BRANCH]}
                    disabled
                    style="min-width: 240px" />
                {/if}
              </div>
              <div class="publish-row">
                <CheckboxInput bind:checked={publish}>
                  Publish the {defaultBranch} branch to the network
                </CheckboxInput>
              </div>
            </div>
          {/if}
        </div>
      </div>

      {#if validations && validations.currentSelection}
        <div class="validation-row">
          <Icon.Important style="margin-right: 8px;fill: var(--color-red)" />
          <Title.Regular style="color: var(--color-red)">
            {validations.currentSelection[0]}
          </Title.Regular>
        </div>
      {/if}

      <div class="button-row">
        <div class="footnote">
          <Text.Small style="color: var(--color-gray)">* required</Text.Small>
        </div>
        <Button
          variant="transparent"
          on:click={pop}
          style="margin-right: 24px;">
          Cancel
        </Button>
        <Button
          disabled={!(name && currentSelection)}
          variant="primary"
          on:click={createProject}>
          Create project
        </Button>
      </div>
    </div>
  </div>
</ModalLayout>
