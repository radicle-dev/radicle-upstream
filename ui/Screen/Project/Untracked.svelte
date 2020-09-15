<script>
  import {
    SidebarLayout,
    TrackToggle,
    Urn,
  } from "../../DesignSystem/Component";
  import { Emoji } from "../../DesignSystem/Primitive";

  export let params = null;
  const urn = params.urn;
</script>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    flex-direction: column;
  }
</style>

<SidebarLayout>
  <div class="container">
    <Emoji emoji={'🎷'} size="huge" style="margin-bottom: 1.5rem;" />

    <h3 style="color: var(--color-foreground-level-6); margin-bottom: 1.5rem;">
      my-new-project
    </h3>

    <Urn {urn} showOnHover style="margin-bottom: 1.5rem;" />

    <p
      style="max-width: 24rem; color: var(--color-foreground-level-6);
      margin-bottom: 1.5rem;"
      align="center">
      You’re not following this project yet, so there’s nothing to show here.
      Follow it and you’ll be notified as soon as it’s available.
    </p>

    <TrackToggle variant="expanded" />
  </div>
</SidebarLayout>
