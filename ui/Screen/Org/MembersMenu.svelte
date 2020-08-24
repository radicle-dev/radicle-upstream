<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";

  export let params = null;
</script>

<Button
  variant="transparent"
  icon={Icon.PlusLarge}
  on:click={() => push(path.memberRegistration(params.id))}>
  Add member
</Button>
