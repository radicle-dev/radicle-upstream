<script>
  import { createEventDispatcher } from "svelte";
  import { pop } from "svelte-spa-router";
  import validatejs from "validate.js";

  import {
    Button,
    Flex,
    Text,
    Title,
    Input
  } from "../../DesignSystem/Primitive";
  import { Dropdown } from "../../DesignSystem/Component";

  const dispatch = createEventDispatcher();

  export let projectId = null;
  export let registrarId = null;
  export let projectName = null;

  export let projects = null;
  export let session = null;
  export let orgs = null;

  export let skipNamePreselection = false;

  const next = () => {
    dispatch("next", {
      registrarHandle: selectedRegistrar().avatarProps.title,
      registrarImageUrl: selectedRegistrar().avatarProps.imageUrl,
      registrarAvatarFallback: selectedRegistrar().avatarProps.avatarFallback,
      registrarVariant: selectedRegistrar().avatarProps.variant
    });
  };

  const selectedRegistrar = () => {
    return registrarDropdownOptions.find(option => {
      return option.value === registrarId;
    });
  };

  const identityOption = {
    variant: "avatar",
    value: session.identity.id,
    avatarProps: {
      variant: "user",
      title: session.identity.metadata.handle,
      avatarFallback: session.identity.avatarFallback,
      imageUrl: session.identity.imageUrl
    }
  };

  const orgOptions = orgs.map(org => {
    return {
      variant: "avatar",
      value: org.id,
      avatarProps: {
        variant: "project",
        title: org.metadata.name,
        avatarFallback: org.avatarFallback
      }
    };
  });

  const registrarDropdownOptions = [identityOption, ...orgOptions];

  const projectDropdownOptions = projects.map(project => {
    return {
      variant: "text",
      value: project.id,
      textProps: { title: project.metadata.name }
    };
  });

  const selectedProject = () => {
    return projectDropdownOptions.find(option => {
      return option.value === projectId;
    });
  };

  // Pre-select existing project name as the to-be-registered name
  $: if (projectId && !skipNamePreselection) {
    projectName = selectedProject().textProps.title;
  } else {
    skipNamePreselection = false;
  }

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");

  let validating = false;
  let validations = false;

  validatejs.options = {
    fullMessages: false
  };

  const validateProjectNameAvailability = async () => {
    try {
      // TODO(rudolfs): wait for the endpoint to land in proxy and fix this
      // const present = await project.get(registrarId, projectName);
      await new Promise(r => setTimeout(r, 500));
      const present = false;

      if (present) {
        validations = { projectName: ["Project name already taken"] };
      }
    } catch (error) {
      validations = { projectName: [error] };
    }
  };

  const constraints = {
    projectId: {
      presence: { message: "Choose a project to register", allowEmpty: false }
    },
    projectName: {
      presence: {
        message: "Project name is required",
        allowEmpty: false
      },
      format: {
        pattern: VALID_NAME_MATCH,
        message: "Project name should match [a-z0-9][a-z0-9_-]+"
      }
    }
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
    margin-bottom: 16px;
  }

  .name-validation {
    margin-left: 12px;
  }
</style>

<Dropdown
  placeholder="Select project to register"
  valid={!(validations && validations.projectId)}
  validationMessage={validations && validations.projectId && validations.projectId[0]}
  bind:value={projectId}
  options={projectDropdownOptions}
  style="margin-bottom: 16px;" />

<div class="name">
  <Dropdown bind:value={registrarId} options={registrarDropdownOptions} />
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

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={pop}
    style="margin-right: 24px;">
    Cancel
  </Button>

  <Button
    dataCy="next-button"
    disabled={!projectName || validating || validations}
    on:click={next}
    variant="primary">
    Next
  </Button>
</Flex>
