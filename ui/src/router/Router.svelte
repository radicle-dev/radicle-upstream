<script lang="typescript">
  import { routeStore } from "ui/src/router";

</script>

<svelte:component this={$routeStore.component} {...$routeStore.props} />
