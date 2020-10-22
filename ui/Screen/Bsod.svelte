<script lang="ts">
  import { Button, Emoji } from "../DesignSystem/Primitive";
  import { Fullscreen } from "../DesignSystem/Component";

  const tweet = () => {
    window.location.href =
      "https://twitter.com/intent/tweet?text=@radicle_xyz help, my Upstream won't start.";
  };
</script>

<style>
  p {
    color: var(--color-background);
    margin-bottom: 1.5rem;
  }

  .container {
    display: flex;
    text-align: center;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
</style>

<Fullscreen
  escapable={false}
  style="background-color: var(--color-secondary);"
  contentStyle="display: flex; width: 312px; align-items: center;">
  <div class="container">
    <Emoji emoji="🧻" size="huge" style="margin-bottom: 1.5rem;" />
    <p>
      We're not totally sure what's going on, but we can't load the app right
      now because the background process won't start up.
    </p>
    <Button style="display: flex;" on:click={tweet}>
      Tweet at us for help
    </Button>
  </div>
</Fullscreen>
