<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";

  export let params = null;
</script>

<Button
  variant="transparent"
  icon={Icon.Plus}
  on:click={() => push(path.registerProject(params.id))}>
  Add project
</Button>
