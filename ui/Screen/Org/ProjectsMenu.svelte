<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";
  import { Icon, Button } from "../../DesignSystem/Primitive";
  import * as path from "../../lib/path.js";

  const orgId = getContext("orgId");
</script>

<Button
  size="small"
  variant="transparent"
  icon={Icon.Plus}
  on:click={() => push(path.registerProject(null, orgId))}>
  Add project
</Button>
