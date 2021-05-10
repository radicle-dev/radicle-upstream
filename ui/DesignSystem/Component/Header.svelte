<style>
  .banner {
    height: 12.5rem;
    background-color: var(--color-foreground-level-1);
    display: flex;
  }

  .banner-content {
    display: flex;
    justify-content: space-between;
    max-width: var(--content-max-width);
    width: 100%;
    margin: 0 auto;
    padding: 0 var(--content-padding);
  }

  .left {
    display: flex;
    min-width: 16rem;
    margin-right: 1.5rem;
  }

  .right {
    display: flex;
    align-items: center;
  }
</style>

<div data-cy="header" class="banner">
  <div class="banner-content">
    <div class="left">
      <slot name="left" />
    </div>
    <div class="right">
      <slot name="right" />
    </div>
  </div>
</div>
