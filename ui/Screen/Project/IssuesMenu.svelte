<script>
  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  variant="transparent"
  icon={Icon.PlusBig}
  on:click={() => console.log('event(new-issue)')}>
  New issue
</Button>
