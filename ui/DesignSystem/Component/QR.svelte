<script>
  import { qrcode } from "pure-svg-code";
  export let key = null;
  export let size = null;
  export let color = "var(--color-primary)";

  const svgString = qrcode({
    content: key,
    width: size,
    height: size,
    color: color,
    background: "var(--color-background)",
    ecl: "M",
  });
</script>

<div class="container">
  {@html svgString}
</div>
