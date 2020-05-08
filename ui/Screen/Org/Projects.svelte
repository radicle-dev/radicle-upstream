<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";

  import { Text, Button } from "../../DesignSystem/Primitive";
  import Placeholder from "../../DesignSystem/Component/Placeholder.svelte";

  const create = () => push(path.registerProject());
  const register = () => push(path.addOrgMember("888"));
</script>

<style>
  .wrapper {
    margin-top: 156px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .content {
    width: 420px;
  }

  .buttons {
    display: flex;
    justify-content: center;
  }
</style>

<div class="wrapper" data-cy="projects">
  <div class="content">
    <Placeholder style="width: 100%; height: 217px; margin-bottom: 24px;" />
    <Text style="margin-bottom: 24px;">
      There's nothing here yet. Get started by creating your first project or
      adding a member to your organization.
    </Text>
    <div class="buttons">
      <Button variant="vanilla" style="margin-right: 25px;" on:click={create}>
        Register a project
      </Button>
      <Button variant="vanilla" on:click={register} dataCy="add-member-button">
        Add a member
      </Button>
    </div>
  </div>
</div>
