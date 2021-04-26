<script lang="typescript">
  export let name: string;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
</style>

<div style="display: flex">
  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis" title={name}>
      {name}
    </h1>
  </div>
</div>
