<script lang="ts">
  export let style = "";

  export let hovering: boolean;

  const enter = () => (hovering = true);
  const leave = () => (hovering = false);
</script>

<div {style} on:mouseenter={enter} on:mouseleave={leave}>
  <slot {hovering} />
</div>
