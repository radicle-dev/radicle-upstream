<script>
  import { Text } from "../../DesignSystem/Primitive";
  export let title = null;
</script>

<style>
  .title {
    width: 8em;
  }

  .swatch {
    display: flex;
    align-items: flex-end;
    padding: 2rem 0;
    border-top: 1px solid var(--color-foreground-level-2);
  }
</style>

<div class="swatch">
  <div class="title">
    <Text variant="tiny" style="color: var(--color-foreground-level-5)">
      {title}
    </Text>
  </div>
  <slot />
</div>
