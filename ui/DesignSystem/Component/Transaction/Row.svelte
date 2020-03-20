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
    border: 1px solid var(--color-lightgray);
    height: 72px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .row:hover {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-pink));
    border: 1px solid var(--focus-outline-color, var(--color-pink));
    cursor: pointer;
  }

  .row.disabled {
    box-shadow: none;
    border: 1px solid var(--color-lightgray);
    cursor: default;
  }

  .row.active {
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-pink));
    border: 1px solid var(--focus-outline-color, var(--color-pink));
  }

  .single {
    border-radius: 2px;
  }

  .bottom {
    border-radius: 0 0 2px 2px;
  }

  .middle {
    margin-bottom: -1px;
  }

  .top {
    margin-bottom: -1px;
    border-radius: 2px 2px 0 0;
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
