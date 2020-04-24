<script>
  import { createEventDispatcher } from "svelte";
  import { pop } from "svelte-spa-router";

  import { Button, Flex, Title } from "../../DesignSystem/Primitive";
  import { RadioOption } from "../../DesignSystem/Component";

  const dispatch = createEventDispatcher();

  export let createNewProject = null;
</script>

<Title style="margin: 16px 0 12px 16px; text-align: left">Select one:</Title>
<RadioOption
  title="Create and register a new project in this org"
  active={createNewProject !== null && createNewProject}
  on:click={() => {
    createNewProject = true;
  }} />

<RadioOption
  title="Register an existing project in this org"
  active={createNewProject !== null && !createNewProject}
  on:click={() => (createNewProject = false)} />

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
    disabled={createNewProject === null}
    on:click={() => {
      dispatch('next');
    }}
    variant="primary">
    Next
  </Button>
</Flex>
