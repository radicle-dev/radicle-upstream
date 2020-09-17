<script>
  import { isExperimental } from "../../../native/ipc.js";

  import { SupportButton } from "../../DesignSystem/Component";
</script>

{#if isExperimental()}
  <SupportButton on:click={() => console.log('event(support-project)')} />
{/if}
