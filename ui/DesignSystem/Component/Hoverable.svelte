<script lang="typescript">
  export let hovering: boolean = false;
  export let style: string = "";

  const enter = () => (hovering = true);
  const leave = () => (hovering = false);

</script>

<div {style} on:mouseenter={enter} on:mouseleave={leave}>
  <slot {hovering} />
</div>
