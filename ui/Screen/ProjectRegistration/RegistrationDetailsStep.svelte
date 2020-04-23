<script>
  import { Button, Flex, Title, Input } from "../../DesignSystem/Primitive";
  import { Dropdown } from "../../DesignSystem/Component";
  import { pop } from "svelte-spa-router";
  import { projects } from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";

  export let onNextStep = null;
  export let createNewProject = false;
  export let projectId = null;

  const orgs = [
    {
      id: "%monadic",
      metadata: {
        name: "monadic"
      },
      avatarFallback: {
        emoji: "☔️",
        background: {
          b: 61,
          g: 187,
          r: 148
        }
      }
    },
    {
      id: "%sveltejs",
      metadata: {
        name: "sveltejs"
      },
      avatarFallback: {
        emoji: "🚊",
        background: {
          b: 112,
          g: 27,
          r: 205
        }
      }
    }
  ];

  const identity = {
    id: "123abcd.git",
    shareableEntityIdentifier: "cloudhead@123abcd.git",
    metadata: {
      handle: "cloudhead",
      displayName: "Alexis Sellier",
      avatarUrl: "https://avatars1.githubusercontent.com/u/40774"
    },
    registered: null,
    avatarFallback: { background: { r: 122, g: 112, b: 90 }, emoji: "💡" }
  };

  const registrarDropdownOptions = [
    {
      variant: "avatar",
      value: "1",
      avatarProps: {
        variant: "user",
        title: identity.metadata.handle,
        avatarFallback: identity.avatarFallback,
        imageUrl: identity.imageUrl
      }
    },
    {
      variant: "avatar",
      value: "2",
      avatarProps: {
        variant: "project",
        title: orgs[0].metadata.name,
        avatarFallback: orgs[0].avatarFallback
      }
    },
    {
      variant: "avatar",
      value: "3",
      avatarProps: {
        variant: "project",
        title: orgs[1].metadata.name,
        avatarFallback: orgs[1].avatarFallback
      }
    }
  ];

  $: console.log($projects.data);
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

  $: console.log(projectDropdownOptions);
</script>

<style>
  .name {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
  }
</style>

{#if createNewProject}
  TODO: create new project
{:else}
  choose existing project choice: {projectId}
  <Dropdown
    placeholder="Choose a project"
    options={projectDropdownOptions}
    style="margin-bottom: 16px;" />
{/if}

<div class="name">
  <Dropdown value="1" options={registrarDropdownOptions} />
  <Title
    style="margin: 0 8px 0 8px; color: var(--color-foreground-level-5);"
    variant="regular">
    /
  </Title>
  <Input.Text
    placeholder="Project name*"
    style="width: 100%"
    valid={true}
    variant="project" />
</div>

<Input.Text placeholder="Project description" />

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={pop}
    style="margin-right: 24px;">
    Cancel
  </Button>
  <Button dataCy="next-button" on:click={onNextStep} variant="primary">
    Next
  </Button>
</Flex>
