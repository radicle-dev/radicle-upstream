<script>
  import { clear, clearCache } from "../../src/session.ts";

  import Button from "../../DesignSystem/Primitive/Button.svelte";
  import Text from "../../DesignSystem/Primitive/Text.svelte";
  import Title from "../../DesignSystem/Primitive/Title.svelte";
</script>

<section>
  <header>
    <Title variant="large">Session management</Title>
  </header>
  <div class="item">
    <div class="info">
      <Text variant="medium">Clear local cache</Text>
      <Text style="color: var(--color-foreground-level-6);">
        Removes all locally-stored temporary data from your device.
      </Text>
    </div>
    <div class="action">
      <Button
        dataCy="clear-cache-button"
        variant="outline"
        on:click={clearCache}>
        Clear cache
      </Button>
    </div>
  </div>
  <div class="item">
    <div class="info">
      <Text variant="medium">Clears all authentication data</Text>
      <Text style="color: var(--color-foreground-level-6);">
        This is similar to how logout works. You will have to create a new
        identity or restore your existing identity.
      </Text>
    </div>
    <div class="action">
      <Button dataCy="clear-session-button" variant="outline" on:click={clear}>
        Clear session
      </Button>
    </div>
  </div>
</section>
