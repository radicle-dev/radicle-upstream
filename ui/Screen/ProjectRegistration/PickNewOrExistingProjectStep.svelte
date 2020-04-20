<script>
  import { Button, Flex, Title } from "../../DesignSystem/Primitive";
  import { RadioOption } from "../../DesignSystem/Component";
  import { pop } from "svelte-spa-router";

  export let onNextStep = null;
  export let createNewProject = null;

  const NEW = "new";
  const EXISTING = "existing";

  let currentSelection = null;

  $: createNewProject = currentSelection === NEW;

  $: isNew = currentSelection === NEW;
  $: isExisting = currentSelection === EXISTING;
</script>

<Title style="margin: 16px 0 12px 16px; text-align: left">Select one:</Title>
<RadioOption
  title="Create and register a new project in this org"
  active={isNew}
  on:click={() => (currentSelection = NEW)} />

<RadioOption
  title="Register an existing project in this org"
  active={isExisting}
  on:click={() => (currentSelection = EXISTING)} />

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
    disabled={!currentSelection}
    on:click={onNextStep}
    variant="primary">
    Next
  </Button>
</Flex>
