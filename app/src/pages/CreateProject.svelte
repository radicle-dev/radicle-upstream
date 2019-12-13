<script>
  import ModalLayout from "../layouts/ModalLayout.svelte";
  import { pop } from "svelte-spa-router";
  import {
    Button,
    Header,
    Icon,
    Input,
    DirectoryInput,
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

  const createProject = () => {};
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

  .active {
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

  .footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 32px;
  }

  .left {
    display: flex;
    flex: 1;
    justify-content: flex-start;
    align-items: flex-start;
    text-align: left;
    padding-left: 15px;
  }

  .body {
    border-top: 1px solid var(--color-lightgray);
    background-color: var(--color-almostwhite);
    padding: 16px 22px 24px 22px;
    border-radius: 0 0 4px 4px;
    display: none;
  }

  .show {
    display: block;
  }

  .default-branch {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .publish {
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
        style="margin-bottom: 16px"
        placeholder="Project name*"
        bind:value={name} />
      <Input
        style="margin-bottom: 16px"
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
          <div class="body" class:show={isNew}>
            <Text.Regular
              style="margin-bottom: 12px; color: var(--color-darkgray)">
              Choose where you'd like to create the repository
            </Text.Regular>
            <DirectoryInput />
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
          <div class="body" class:show={isExisting}>
            <Text.Regular
              style="margin-bottom: 12px; color: var(--color-darkgray)">
              Choose the existing repository
            </Text.Regular>
            <DirectoryInput style="margin-bottom: 16px" />
            <div class="default-branch">
              <Text.Regular style="color: var(--color-darkgray)">
                Select the default branch
              </Text.Regular>
              <Select
                items={['master', 'dev']}
                bind:value={defaultBranch}
                style="min-width: 240px" />
            </div>
            <div class="publish">
              <Icon.CheckCircle style="margin-right: 14px" />
              <Text.Regular style="color: var(--color-darkgray)">
                Publish the {defaultBranch} branch to the network
              </Text.Regular>
            </div>
          </div>
        </div>
      </div>

      <div class="footer">
        <div class="left">
          <Text.Small style="color: var(--color-gray)">* required</Text.Small>
        </div>
        <Button
          variant="transparent"
          on:click={pop}
          style="margin-right: 24px;">
          Cancel
        </Button>
        <Button variant="primary" on:click={createProject} disabled>
          Create project
        </Button>
      </div>
    </div>
  </div>
</ModalLayout>
