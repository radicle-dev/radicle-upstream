<style>
  header {
    display: flex;
    align-items: center;
  }

  .align-left {
    display: flex;
    flex: 1;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }
</style>

<header>
  <div class="align-left">
    <slot name="left" />
  </div>

  <div class="align-right">
    <slot name="right" />
  </div>
</header>
