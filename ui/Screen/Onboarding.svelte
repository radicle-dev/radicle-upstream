<script>
  import { loc } from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import Button from "../DesignSystem/Primitive/Button.svelte";
  import Text from "../DesignSystem/Primitive/Text.svelte";
  import RadicleLogo from "../DesignSystem/Component/RadicleLogo.svelte";

  import Modal from "../Layout/Modal.svelte";

  const onStart = () => {
    console.log("start pressed");

    path.modalCreateIdentity($loc);
  };
</script>

<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
  }
</style>

<Modal escapable={false}>
  <main>
    <RadicleLogo />
    <Text
      style="text-align: center; color: var(--color-foreground-level-5); margin:
      2.5rem 0; max-width: 20rem;">
      A free and open-source way to host, share, and build software together.
    </Text>
    <Button
      style="flex-shrink: 0; margin-bottom: 24px;"
      on:click={onStart}
      dataCy="get-started-button">
      Get started
    </Button>
  </main>
</Modal>
