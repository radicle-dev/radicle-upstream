<script>
  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  variant="transparent"
  icon={Icon.ArrowUp}
  on:click={() => console.log('event(send-button-click)')}>
  Send
</Button>

<Button
  variant="transparent"
  icon={Icon.ArrowDown}
  on:click={() => console.log('event(receive-button-click)')}>
  Receive
</Button>
