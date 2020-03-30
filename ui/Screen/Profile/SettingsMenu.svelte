<script>
  import { push } from "svelte-spa-router";
  import { Icon, Button } from "../../DesignSystem/Primitive";
  import { registerUser } from "../../lib/path.js";
</script>

<Button
  size="small"
  variant="transparent"
  icon={Icon.Register}
  on:click={() => push(registerUser())}>
  Register handle
</Button>
