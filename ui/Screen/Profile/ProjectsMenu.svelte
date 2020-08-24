<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  dataCy="new-project-button"
  variant="transparent"
  icon={Icon.PlusLarge}
  on:click={() => push(path.createProject())}>
  New project
</Button>
