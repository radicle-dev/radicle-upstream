<script>
  import { Icon } from "../Primitive";

  export let title = null;
  export let description = null;
  export let showRegisteredBadge = false;
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

  .registered {
    display: flex;
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
      {#if showRegisteredBadge}
        <div data-cy="registered" class="registered">
          <Icon.RegisteredSmall style="fill: var(--color-primary);" />
        </div>
      {/if}
    </div>
    <p class="desc">
      {#if description}{description}{/if}
    </p>
  </div>
</div>
