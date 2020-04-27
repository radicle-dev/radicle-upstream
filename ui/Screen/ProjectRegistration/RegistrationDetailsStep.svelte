<script>
  import { createEventDispatcher } from "svelte";
  import { pop } from "svelte-spa-router";

  import { Button, Flex, Title, Input } from "../../DesignSystem/Primitive";
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
    valid={true}
    variant="project" />
</div>

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
    on:click={() => {
      dispatch('next');
    }}
    variant="primary">
    Next
  </Button>
</Flex>
