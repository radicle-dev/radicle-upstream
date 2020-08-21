<script>
  export let style = null;

  let hovering;

  const enter = () => (hovering = true);
  const leave = () => (hovering = false);
</script>

<div {style} on:mouseenter={enter} on:mouseleave={leave}>
  <slot {hovering} />
</div>
