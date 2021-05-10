<script lang="typescript">
  import { state } from "ui/src/router";

</script>

<svelte:component this={$state.component} {...$state.props} />
