<script>
  import ModalLayout from "../layouts/ModalLayout.svelte";

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

  const options = {
    NEW: "new",
    EXISTING: "existing"
  };

  let currentSelection;

  const selectNew = () => {
    currentSelection = options.NEW;
  };
  const selectExisting = () => {
    currentSelection = options.EXISTING;
  };
  $: isNew = currentSelection === options.NEW;
  $: isExisting = currentSelection === options.EXISTING;

  let name = "";
  let description = "";
  let defaultBranch = "master";
  let publish = true;
  let newRepositoryPath = "";
  let existingRepositoryPath = "";

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  let showValidations = false;

  $: isFormValid = isNameValid && isPathValid;
  $: isNameValid = VALID_NAME_MATCH.test(name);
  $: isPathValid =
    (isNew && newRepositoryPath.length > 0) ||
    (isExisting && existingRepositoryPath.length > 0);

  const client = getClient();

  const LOCAL_BRANCHES = gql`
    query($projectId: IdInput!) {
      branches(id: $projectId)
    }
  `;

  $: branches = query(client, {
    query: LOCAL_BRANCHES,
    variables: {
      projectId: {
        name: isNew ? newRepositoryPath : existingRepositoryPath,
        domain: "buu"
      }
    }
  });

  const CREATE_PROJECT = gql`
    mutation($name: String!, $description: String!, $imgUrl: String!) {
      registerProject(name: $name, description: $description, imgUrl: $imgUrl) {
        name
        description
        imgUrl
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

  let project;

  const createProject = async () => {
    try {
      project = await mutate(client, {
        mutation: CREATE_PROJECT,
        variables: {
          name: name,
          description: description,
          imgUrl: `https://avatars.dicebear.com/v2/jdenticon/${anonymize(
            name + description
          )}.svg`
          // defaultBranch: defaultBranch
          // path: (isNew ? newRepositoryPath : existingRepositoryPath)
          // publish: (isNew ? true : publish)
        }
      });
      push(path.projectOverview("rad", name));
      // TODO: show info message
    } catch (error) {
      console.log("Error: " + error);
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

  .option .active {
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
    display: none;
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
</style>

<ModalLayout dataCy="page">
  <div class="wrapper">
    <div class="create-project">
      <Title.Big style="margin-bottom: 32px; text-align: left">
        Create a new project
      </Title.Big>
      <Input
        style="margin-bottom: 16px; --focus-outline-color: var(--color-pink)"
        placeholder="Project name*"
        bind:value={name}
        on:change={() => (showValidations = true)}
        valid={!showValidations || isNameValid} />
      <Input
        style="margin-bottom: 16px; --focus-outline-color: var(--color-pink)"
        placeholder="Project description"
        bind:value={description} />

      <Title.Regular style="margin: 0 0 12px 16px; text-align: left">
        Select one:
      </Title.Regular>

      <div class="radio-selector">
        <div class="option" class:active={isNew}>
          <div class="option-header" on:click={selectNew}>
            <Title.Regular style="color: var(--color-darkgray)">
              Start with a new repository
            </Title.Regular>
            <Icon.CheckCircle
              style={isNew ? 'display: block' : 'display: none'} />
          </div>
          <div class="option-body" class:show={isNew}>
            <Text.Regular
              style="margin-bottom: 12px; color: var(--color-darkgray)">
              Choose where you'd like to create the repository
            </Text.Regular>
            <DirectoryInput bind:path={newRepositoryPath} />
          </div>
        </div>

        <div class="option" class:active={isExisting}>
          <div class="option-header" on:click={selectExisting}>
            <Title.Regular style="color: var(--color-darkgray)">
              Continue with an existing repository
            </Title.Regular>
            <Icon.CheckCircle
              style={isExisting ? 'display: block' : 'display: none'} />
          </div>
          <div class="option-body" class:show={isExisting}>
            <Text.Regular
              style="margin-bottom: 12px; color: var(--color-darkgray)">
              Choose the existing repository
            </Text.Regular>
            <DirectoryInput
              style="margin-bottom: 16px"
              bind:path={existingRepositoryPath} />
            <div class="default-branch-row">
              <Text.Regular style="color: var(--color-darkgray)">
                Select the default branch
              </Text.Regular>
              {#if existingRepositoryPath.length > 0}
                {#await $branches then result}
                  <Select
                    items={result.data.branches}
                    bind:value={defaultBranch}
                    style="min-width: 240px" />
                {/await}
              {:else}
                <Select
                  items={[defaultBranch]}
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
        </div>
      </div>

      <div class="validation-row">
        {#if showValidations && !isNameValid}
          <Icon.Important style="margin-right: 8px;fill: var(--color-red)" />
          <Title.Regular style="color: var(--color-red)">
            Project name should be [a-z0-9][a-z0-9_-]+
          </Title.Regular>
        {/if}
      </div>

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
          variant="primary"
          on:click={createProject}
          disabled={!isFormValid}>
          Create project
        </Button>
      </div>
    </div>
  </div>
</ModalLayout>
