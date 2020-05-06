<script>
  import { push } from "svelte-spa-router";
  import { Icon, Button } from "../../DesignSystem/Primitive";
  import * as path from "../../lib/path.js";

  export let params = null;
</script>

<Button
  variant="transparent"
  icon={Icon.Plus}
  on:click={() => push(path.registerProject(params.id))}>
  Add project
</Button>
