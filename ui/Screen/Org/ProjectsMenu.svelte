<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";

  export let params = null;
  const session = getContext("session");
</script>

<Button
  dataCy="projects-menu-button"
  variant="transparent"
  disabled={!session.permissions.registerProject}
  icon={Icon.Plus}
  on:click={() => push(path.registerProject(params.id))}>
  Add project
</Button>
