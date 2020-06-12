<script>
  import {
    projectIdValidationStore,
    projectNameValidationStore,
  } from "../../src/project.ts";
  import { ValidationStatus } from "../../src/validation.ts";

  import { Text, Title, Input } from "../../DesignSystem/Primitive";
  import { Dropdown } from "../../DesignSystem/Component";

  export let projectId = null;
  export let registrantId = null;
  export let projectName = null;
  export let valid = null;

  export let registrantOptions;
  export let projects = null;

  export let skipNamePreselection = false;

  const getSelectedProject = () => {
    return projectDropdownOptions.find((option) => {
      return option.value === projectId;
    });
  };

  const projectDropdownOptions = projects.map((project) => {
    return {
      variant: "text",
      value: project.id,
      textProps: { title: project.metadata.name },
    };
  });

  // Pre-select existing project name as the to-be-registered name
  $: if (projectId && !skipNamePreselection) {
    projectName = getSelectedProject().textProps.title;
  } else {
    skipNamePreselection = false;
  }

  const projectIdValidation = projectIdValidationStore();

  // Make sure we update validation when the registrant changes
  $: projectNameValidation = projectNameValidationStore(registrantId);

  // Start validating only when the user has touched the form
  $: if (projectId || projectName) {
    projectIdValidation.validate(projectId);
    projectNameValidation.validate(projectName, registrantId);
  }

  $: valid =
    $projectIdValidation.status === ValidationStatus.Success &&
    $projectNameValidation.status === ValidationStatus.Success;
</script>

<style>
  .name {
    display: flex;
    align-items: center;
  }

  .name-validation {
    margin-left: 12px;
    margin-top: 12px;
  }
</style>

<Dropdown
  dataCy="project-dropdown"
  optionStyle="width: 538px"
  placeholder="Select project to register"
  valid={$projectIdValidation.status === ValidationStatus.Success || $projectIdValidation.status === ValidationStatus.NotStarted}
  validationMessage={$projectIdValidation.message}
  bind:value={projectId}
  options={projectDropdownOptions}
  style="margin-bottom: 16px;" />

<div class="name">
  <Dropdown
    dataCy="domain-dropdown"
    bind:value={registrantId}
    options={registrantOptions} />
  <Title
    style="margin: 0 8px 0 8px; color: var(--color-foreground-level-5);"
    variant="regular">
    /
  </Title>
  <Input.Text
    dataCy="name-input"
    placeholder="Project name*"
    style="width: 100%;"
    bind:value={projectName}
    variant="project"
    validation={$projectNameValidation}
    hideValidationMessages />
</div>

{#if $projectNameValidation.status === ValidationStatus.Error}
  <div class="name-validation">
    <Text style="color: var(--color-negative); text-align: left;">
      {$projectNameValidation.message}
    </Text>
  </div>
{/if}
