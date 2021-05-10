<script lang="typescript">
  export let dataCy = "";

</script>

<style>
  .container {
    width: 38.5rem;
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

</style>

<div class="container" data-cy={dataCy}>
  <slot />
</div>
