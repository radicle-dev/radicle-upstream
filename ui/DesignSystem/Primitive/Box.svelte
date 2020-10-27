<script lang="typescript">
  export let style = "";
</script>

<style>
  .box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;

    background: var(--color-foreground-level-1);
    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;

    padding: var(--content-padding);
    text-align: center;
  }
</style>

<div class="box" {style}>
  <slot />
</div>
