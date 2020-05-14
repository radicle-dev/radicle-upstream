<script>
  import { createEventDispatcher } from "svelte";
  import { pop } from "svelte-spa-router";
  import validatejs from "validate.js";

  import * as notification from "../../src/notification.ts";
  import { getOrgProject, Domain } from "../../src/project.ts";

  import { Text, Title, Input } from "../../DesignSystem/Primitive";
  import { Dropdown, NavigationButtons } from "../../DesignSystem/Component";

  const dispatch = createEventDispatcher();

  export let projectId = null;
  export let domainId = null;
  export let projectName = null;

  export let projects = null;
  export let session = null;
  export let orgs = null;

  export let skipNamePreselection = false;

  const next = () => {
    dispatch("next", {
      domainHandle: selectedDomain().value,
      domainType: selectedDomain().type,
    });
  };

  const selectedDomain = () => {
    return domainDropdownOptions.find((option) => {
      return option.value === domainId;
    });
  };

  const identityOption = {
    variant: "avatar",
    value: session.identity.id,
    type: Domain.User,
    avatarProps: {
      variant: "circle",
      title: session.identity.metadata.handle,
      avatarFallback: session.identity.avatarFallback,
      imageUrl: session.identity.imageUrl,
    },
  };

  const orgOptions = orgs.map((org) => {
    return {
      variant: "avatar",
      value: org.id,
      type: Domain.Org,
      avatarProps: {
        variant: "square",
        title: org.id,
        avatarFallback: org.avatarFallback,
      },
    };
  });

  const domainDropdownOptions = [identityOption, ...orgOptions];
  console.log(domainDropdownOptions);

  const projectDropdownOptions = projects.map((project) => {
    return {
      variant: "text",
      value: project.id,
      textProps: { title: project.metadata.name },
    };
  });

  const selectedProject = () => {
    return projectDropdownOptions.find((option) => {
      return option.value === projectId;
    });
  };

  // Pre-select existing project name as the to-be-registered name
  $: if (projectId && !skipNamePreselection) {
    projectName = selectedProject().textProps.title;
  } else {
    skipNamePreselection = false;
  }

  const NAME_MATCH = "^[a-z0-9][a-z0-9_-]+$";

  let validating = false;
  let validations = false;

  validatejs.options = {
    fullMessages: false,
  };

  const validateProjectNameAvailability = async () => {
    try {
      const present = await getOrgProject(
        selectedDomain().avatarProps.title,
        projectName
      );

      if (present) {
        validations = { projectName: ["Project name already taken"] };
      }
    } catch (error) {
      notification.error(`Proxy: ${JSON.stringify(error)}`);
    }
  };

  const constraints = {
    projectId: {
      presence: { message: "Choose a project to register", allowEmpty: false },
    },
    projectName: {
      presence: {
        message: "Project name is required",
        allowEmpty: false,
      },
      format: {
        pattern: new RegExp(NAME_MATCH),
        message: `Project name should match ${NAME_MATCH}`,
      },
    },
  };

  const validate = async () => {
    validating = true;
    validations = validatejs(
      { projectId: projectId, projectName: projectName },
      constraints
    );

    if (!validatejs.isEmpty(validations)) {
      validating = false;
    } else {
      await validateProjectNameAvailability();
      validating = false;
    }
  };

  // Start validating only when the user has touched the form
  $: (projectId || projectName) && validate(projectId, projectName);
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
  optionStyle="width: 538px"
  placeholder="Select project to register"
  valid={!(validations && validations.projectId)}
  validationMessage={validations && validations.projectId && validations.projectId[0]}
  bind:value={projectId}
  options={projectDropdownOptions}
  style="margin-bottom: 16px;" />

<div class="name">
  <Dropdown bind:value={domainId} options={domainDropdownOptions} />
  <Title
    style="margin: 0 8px 0 8px; color: var(--color-foreground-level-5);"
    variant="regular">
    /
  </Title>
  <Input.Text
    placeholder="Project name*"
    style="width: 100%;"
    bind:value={projectName}
    valid={!(validations && validations.projectName)}
    variant="project"
    validationPending={validating} />
</div>

{#if validations && validations.projectName}
  <div class="name-validation">
    <Text style="color: var(--color-negative); text-align: left;">
      {validations.projectName[0]}
    </Text>
  </div>
{/if}

<NavigationButtons
  style="margin-top: 32px;"
  on:cancel={pop}
  on:submit={next}
  disableSubmit={!projectName || validating || validations} />
