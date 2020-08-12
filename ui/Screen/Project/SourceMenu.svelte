<script>
  import { isDev } from "../../../native/ipc.js";

  import { SupportButton } from "../../DesignSystem/Component";
</script>

{#if isDev()}
  <SupportButton on:click={() => console.log('event(support-project)')} />
{/if}
