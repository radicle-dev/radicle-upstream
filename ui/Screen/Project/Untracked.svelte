<script>
  import twemoji from "twemoji";

  import {
    SidebarLayout,
    TrackToggle,
    Urn,
  } from "../../DesignSystem/Component";

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

  .emoji {
    height: 3rem;
    width: 3rem;
    margin-bottom: 1.5rem;
  }
</style>

<SidebarLayout>
  <div class="container">
    <div class="emoji">
      {@html twemoji.parse('🎷', { base: '', folder: 'twemoji/', ext: '.svg' })}
    </div>

    <h3 style="color: var(--color-foreground-level-6); margin-bottom: 1.5rem;">
      my-new-project
    </h3>

    <Urn {urn} showOnHover style="margin-bottom: 1.5rem;" />

    <p
      style="max-width: 24rem; color: var(--color-foreground-level-6);
      margin-bottom: 1.5rem;"
      align="center">
      You’re not tracking this project yet, so there’s nothing to show here.
      Track it and you’ll be notified as soon as it’s available.
    </p>

    <TrackToggle variant="expanded" />
  </div>
</SidebarLayout>
