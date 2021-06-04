<script lang="typescript">
  export let dataCy: string = "";
  export let style: string = "";
</script>

<style>
  .container {
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 1rem;
    padding: 2rem;
    box-shadow: var(--color-shadows);
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .container:focus {
    outline: none;
  }
  .content {
    width: 100%;
  }
</style>

<div class="container" data-cy={dataCy}>
  <div class="content" {style}>
    <slot />
  </div>
</div>
