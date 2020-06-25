<script>
  import Button from "../../DesignSystem/Primitive/Button.svelte";
  import Text from "../../DesignSystem/Primitive/Text.svelte";
  import Title from "../../DesignSystem/Primitive/Title.svelte";
</script>

<section>
  <header>
    <Title variant="large">Developer tools</Title>
  </header>
  <div class="item">
    <div class="info">
      <Text variant="medium">
        Unlink all unregistered projects from my profile
      </Text>
      <Text style="color: var(--color-foreground-level-6);">
        This unlinks your local repositories from Upstream. The local data will
        remain on your computer.
      </Text>
    </div>
    <div class="action">
      <Button variant="destructive">Remove</Button>
    </div>
  </div>
</section>
