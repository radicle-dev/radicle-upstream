<script lang="ts">
  import { Button, Emoji } from "../DesignSystem/Primitive";
  import { Fullscreen } from "../DesignSystem/Component";
  import { fatalError } from "../src/error";

  const tweet = () => {
    window.location.href = "https://radicle.community/c/support/13";
  };
</script>

<style>
  p {
    margin-bottom: 1.5rem;
  }

  .container {
    display: flex;
    text-align: center;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    color: #fff; /* I know... but this design doesn't work in dark mode. */
  }
</style>

{#if $fatalError}
  <Fullscreen
    escapable={false}
    style="background-color: var(--color-secondary); z-index: 200;"
    contentStyle="display: flex; width: 312px; align-items: center;">
    <div class="container" data-cy="blue-screen-of-death">
      <Emoji emoji="🧻" size="huge" style="margin-bottom: 1.5rem;" />
      <p>
        We're not totally sure what's going on, but we can't load the app right
        now because the background process won't start up.
      </p>
      <Button style="display: flex; color: #fff;" on:click={tweet}>
        Reach out for support
      </Button>
    </div>
  </Fullscreen>
{/if}
