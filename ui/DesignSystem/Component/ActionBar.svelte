<script lang="typescript">
  let wrapper: Element;
  let scrollHeight: number;
  let scrollY = 0;

  $: if (wrapper) {
    scrollHeight = wrapper.getBoundingClientRect().top;
  }

</script>

<style>
  .action-bar-wrapper {
    background-color: var(--color-background);
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .action-bar {
    display: flex;
    justify-content: space-between;
    height: var(--topbar-height);
    width: 100%;
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
    align-items: center;
  }

  .elevation {
    box-shadow: var(--elevation-low);
  }

</style>

<svelte:window bind:scrollY />

<div
  bind:this={wrapper}
  class="action-bar-wrapper"
  class:elevation={scrollY > scrollHeight}>
  <div class="action-bar">
    <slot name="left" />
    <slot name="right" />
  </div>
</div>
