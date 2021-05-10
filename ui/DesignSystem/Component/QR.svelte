<script>
  import { qrcode } from "pure-svg-code";
  export let key = "";
  export let size = 225;
  export let color = "black";
  export let bgColor = "white";

  const svgString = qrcode({
    content: key,
    width: size,
    height: size,
    color: color,
    background: bgColor,
    ecl: "M",
  });

</script>

<div class="container" data-cy="qr-code">
  {@html svgString}
</div>
