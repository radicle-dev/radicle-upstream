<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  variant="transparent"
  icon={Icon.Register}
  on:click={() => push(path.registerUser())}>
  Register handle
</Button>
