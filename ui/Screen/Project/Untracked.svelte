<script>
  import {
    ShareableIdentifier,
    SidebarLayout,
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
      Still looking…
    </h3>

    <ShareableIdentifier {urn} style="margin-bottom: 1.5rem;" />

    <p
      style="max-width: 24rem; color: var(--color-foreground-level-6);
      margin-bottom: 1.5rem;"
      align="center">
      This hasn't been found in your network yet<br />
      or doesn't exist.
    </p>
  </div>
</SidebarLayout>
