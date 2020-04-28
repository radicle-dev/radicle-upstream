<script>
  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  variant="transparent"
  icon={Icon.ArrowUp}
  on:click={() => console.log('event(org-send-funds)')}>
  Send
</Button>

<Button
  variant="transparent"
  icon={Icon.ArrowDown}
  on:click={() => console.log('event(org-receive-funds)')}>
  Receive
</Button>
