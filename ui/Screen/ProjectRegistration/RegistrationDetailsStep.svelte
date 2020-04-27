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

  import { projects } from "../../src/project.ts";
  import { session } from "../../src/session.ts";
  import * as remote from "../../src/remote.ts";
  import { orgMocks } from "../../lib/orgMocks.js";

  const dispatch = createEventDispatcher();

  export let projectId = null;
  export let registrarId = null;
  export let projectName = null;

  $: identity =
    ($session.status === remote.Status.Success && [
      {
        variant: "avatar",
        value: $session.data.identity.id,
        avatarProps: {
          variant: "user",
          title: $session.data.identity.metadata.handle,
          avatarFallback: $session.data.identity.avatarFallback,
          imageUrl: $session.data.identity.imageUrl
        }
      }
    ]) ||
    [];

  $: orgs = orgMocks.data.orgs.map(org => {
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

  $: registrarDropdownOptions = [...identity, ...orgs];

  $: projectDropdownOptions =
    ($projects.status === remote.Status.Success &&
      $projects.data.map(project => {
        return {
          variant: "text",
          value: project.id,
          textProps: { title: project.metadata.name }
        };
      })) ||
    [];

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  let validating = false;
  let validations = false;

  const validateProjectNameAvailability = async () => {
    try {
      // TODO(rudolfs): wait for #312 to land and fix this
      // const present = await project.get(registrarId, projectName);
      await new Promise(r => setTimeout(r, 500));
      const present = Math.random() > 0.5;

      if (present) {
        validations = { projectName: ["Project name already taken"] };
      }
    } catch (error) {
      validations = { projectName: [error] };
    }
  };

  validatejs.options = {
    fullMessages: false
  };

  const constraints = {
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
    validations = validatejs({ projectName: projectName }, constraints);

    if (!validatejs.isEmpty(validations)) {
      validating = false;
    } else {
      await validateProjectNameAvailability();
      validating = false;
    }
  };

  $: validate(projectName);
</script>

<style>
  .name {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
  }
</style>

<Dropdown
  placeholder="Select project to register"
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
  <div>
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
    on:click={() => {
      dispatch('next');
    }}
    variant="primary">
    Next
  </Button>
</Flex>
