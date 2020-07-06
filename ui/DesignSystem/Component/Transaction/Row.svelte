<script>
  import { Flex } from "../../Primitive";

  export let style = null;
  export let dataCy = null;

  export let variant = "single"; // top || middle || bottom
  export let active = false;
  export let disabled = true;

  $: disabledClass = disabled ? "disabled" : null;
  $: activeClass = active ? "active" : null;
  $: rowClass = ["row", activeClass, disabledClass, variant].join(" ");
</script>

<style>
  .row {
    padding-left: 16px;
    padding-right: 16px;
    border: 1px solid var(--color-foreground-level-2);
    height: 56px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .row:hover {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-primary));
    border: 1px solid var(--focus-outline-color, var(--color-primary));
    cursor: pointer;
  }

  .row.disabled {
    box-shadow: none;
    border: 1px solid var(--color-foreground-level-2);
    cursor: default;
  }

  .row.active {
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-primary));
    border: 1px solid var(--focus-outline-color, var(--color-primary));
  }

  .single {
    border-radius: 4px;
  }

  .bottom {
    border-radius: 0 0 4px 4px;
  }

  .middle {
    margin-bottom: -1px;
  }

  .top {
    margin-bottom: -1px;
    border-radius: 4px 4px 0 0;
  }
</style>

<div class={rowClass} {style} data-cy={dataCy}>
  <Flex>
    <div slot="left">
      <slot name="left" />
    </div>

    <div slot="right">
      <slot name="right" />
    </div>
  </Flex>
</div>
