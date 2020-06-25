<script>
  import Button from "../../DesignSystem/Primitive/Button.svelte";
  import Text from "../../DesignSystem/Primitive/Text.svelte";
  import Title from "../../DesignSystem/Primitive/Title.svelte";
</script>

<section>
  <header>
    <Title variant="large">Version</Title>
  </header>
  <div class="item">
    <div class="info">
      <Text style="color: var(--color-foreground-level-6);">
        Version 01.45.02
      </Text>
    </div>
    <div class="action">
      <Text style="color: var(--color-foreground-level-6);">
        There’s a new version of Radicle Upstream
      </Text>
      <Button style="margin-left: 16px;">Update to Version 01.45.03</Button>
    </div>
  </div>
</section>
