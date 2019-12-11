<script>
  export let dataCy = null;
</script>

<style>
  .content {
    margin-top: 64px;
    margin-left: 8vw;
    margin-right: 8vw;
  }
</style>

<div class="content" data-cy={dataCy}>
  <slot />
</div>
