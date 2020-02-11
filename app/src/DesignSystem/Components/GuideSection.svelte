<script>
  import { Text } from "../../DesignSystem/Primitives";

  export let title = null;
  export let subTitle = null;
</script>

<style>
  .section {
    margin-bottom: 180px;
    display: grid;
    grid-template-columns: 342px auto 100px;
  }

  .titles {
    grid-column-start: 1;
    margin-right: 90px;
  }
</style>

<div class="section">
  <div class="titles">
    <Text variant="bigTitle">{title}</Text>
    <Text style="color: var(--color-gray)">{subTitle}</Text>
  </div>
  <div class="content">
    <slot />
  </div>
</div>
