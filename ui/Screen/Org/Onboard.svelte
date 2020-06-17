<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Flex, Text, Button } from "../../DesignSystem/Primitive";
  import { Placeholder } from "../../DesignSystem/Component";

  export let orgId = null;
  export let registerProjectPermission = null;

  const create = () => push(path.registerProject(orgId));
  const register = () => push(path.memberRegistration(orgId));
</script>

<style>
  .wrapper {
    margin-top: 156px;
    display: flex;
    justify-content: center;
  }

  .create-project {
    text-align: center;
    width: 480px;
  }
</style>

<div class="wrapper">
  <div class="create-project">
    <Placeholder style="width: 420px; height: 217px; margin-bottom: 24px;" />
    <Text style="color: var(--color-foreground-level-6); margin-bottom: 28px;">
      There's nothing here yet, get started by creating your first project or
      adding a member to your organism.
    </Text>
    <Flex align="center" style="flex-direction: row">
      <Button
        dataCy="add-project"
        disabled={!registerProjectPermission}
        style="margin-right: 26px"
        variant="vanilla"
        on:click={create}>
        Register a project
      </Button>
      <Button dataCy="add-member-button" variant="vanilla" on:click={register}>
        Add a member
      </Button>
    </Flex>
  </div>
</div>
