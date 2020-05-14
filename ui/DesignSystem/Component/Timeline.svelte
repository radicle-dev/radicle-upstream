<script>
  import TimelineItem from "./Timeline/TimelineItem.svelte";
  export let items = null;
</script>

<style>
  ul {
    margin-top: 24px;
  }
</style>

<ul>
  {#each items as item}
    <TimelineItem {item} />
  {/each}
  <TimelineItem />
</ul>
