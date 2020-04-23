<script>
  import { Button, Flex, Title, Input } from "../../DesignSystem/Primitive";
  import { Dropdown } from "../../DesignSystem/Component";
  import { pop } from "svelte-spa-router";

  export let createNewProject = false;
  export let onNextStep = null;
</script>

<style>
  .name {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
  }
</style>

<div class="name">
  <Dropdown />
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
