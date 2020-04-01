<script>
  import { push } from "svelte-spa-router";
  import { Icon, Button } from "../../DesignSystem/Primitive";
  import { createProject } from "../../lib/path.js";
</script>

<Button
  size="small"
  variant="transparent"
  icon={Icon.Plus}
  on:click={() => push(createProject())}>
  New project
</Button>
