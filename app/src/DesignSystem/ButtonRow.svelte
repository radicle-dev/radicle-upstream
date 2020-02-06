<script>
  export let style = null;
</script>

<style>
  .button-row {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 48px;
  }

  div.left-button {
    display: flex;
    flex: 1;
    align-items: flex-start;
  }
</style>

<div class="button-row">
  <div class="left-button">
    <slot name="left" />
  </div>

  <slot name="right" />
</div>
