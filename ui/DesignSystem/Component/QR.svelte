<script>
  import { qrcode } from "pure-svg-code";
  export let key = null;
  export let size = null;
  export let color = null;

  const svgString = qrcode({
    content: key,
    width: size,
    height: size,
    color: color,
    background: "#ffffff",
    ecl: "M",
  });
</script>

<style>

</style>

<div class="container">
  {@html svgString}
</div>
