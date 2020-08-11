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
    height: 44px;
    width: 44px;
    margin-bottom: 24px;
  }
</style>

<SidebarLayout>
  <div class="container">
    <div class="emoji">
      {@html twemoji.parse('🎷', { base: '', folder: 'twemoji/', ext: '.svg' })}
    </div>

    <Urn {urn} />

    <p
      style="max-width: 384px; color: var(--color-foreground-level-6);
      margin-bottom: 24px;"
      align="center">
      You’re not tracking this project yet, so there’s nothing to show here.
      Track it and you’ll be notified as soon as it’s available.
    </p>

    <TrackToggle />
  </div>
</SidebarLayout>
