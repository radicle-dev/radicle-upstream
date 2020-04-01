<script>
  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  size="small"
  variant="transparent"
  icon={Icon.Plus}
  on:click={() => console.log('event(new-issue)')}>
  New issue
</Button>
