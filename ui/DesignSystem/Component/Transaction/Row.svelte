<script>
  import { Flex } from "../../Primitive";

  export let style = null;
  export let dataCy = null;

  export let variant = "single"; // top || middle || bottom
</script>

<style>
  .row {
    padding-left: 16px;
    padding-right: 16px;
    height: 52px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .single {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 4px;
    height: 56px;
  }

  .bottom {
    border-left: 1px solid var(--color-foreground-level-2);
    border-right: 1px solid var(--color-foreground-level-2);
    border-bottom: 1px solid var(--color-foreground-level-2);
    border-radius: 0 0 4px 4px;
  }

  .middle {
    border-left: 1px solid var(--color-foreground-level-2);
    border-right: 1px solid var(--color-foreground-level-2);
  }

  .top {
    border-radius: 4px 4px 0 0;
    border-left: 1px solid var(--color-foreground-level-2);
    border-right: 1px solid var(--color-foreground-level-2);
    border-top: 1px solid var(--color-foreground-level-2);
  }
</style>

<div class="row {variant}" {style} data-cy={dataCy}>
  <Flex>
    <div slot="left">
      <slot name="left" />
    </div>

    <div slot="right">
      <slot name="right" />
    </div>
  </Flex>
</div>
