<script>
  export let title = null;
  export let description = null;
</script>

<style>
  .project-card {
    display: flex;
    width: 100%;
    max-height: 3.2rem;
  }

  .container {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    width: 10%;
  }

  .title-row {
    display: flex;
    margin-bottom: 2px;
    white-space: nowrap;
  }

  .desc {
    color: var(--color-foreground-level-5);
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>

<div class="project-card">
  <div class="container">
    <div class="title-row">
      <p class="typo-text-bold">{title}</p>
    </div>
    <p class="desc">
      {#if description}{description}{/if}
    </p>
  </div>
</div>
