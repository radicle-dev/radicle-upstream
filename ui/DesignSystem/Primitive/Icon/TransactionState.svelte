<script>
  export let progress = 0;
  export let rotate = false;
  export let style = null;
  export let state = "caution"; // negative | caution | positive
  export let variant = "regular"; // small | regular | inverted

  const max = 100;
  $: size = {
    regular: 32,
    inverted: 24,
    small: 16,
  }[variant];

  $: strokeWidth = {
    regular: 4,
    inverted: 6,
    small: 2,
  }[variant];

  $: center = size / 2;
  $: radius = size / 2 - strokeWidth / 2;
  $: circumference = 2 * Math.PI * radius;
  $: color = {
    caution: "var(--color-caution)",
    positive: "var(--color-positive)",
    negative: "var(--color-negative)",
  }[state];
  $: defaultDashLength = {
    caution: (1 / 6) * 100,
    positive: max,
    negative: 0,
  }[state];
  $: dashLength = progress === 0 ? defaultDashLength : progress;
  $: dashArray = (dashLength * circumference) / max;
  $: spin = progress === 0 || rotate;
</script>

<style>
  .opaque {
    opacity: 0.3;
  }

  .spin {
    transform-origin: var(--origin);
    animation: spin 5s infinite linear;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(359deg);
    }
  }
</style>

<svg
  on:click
  {style}
  width={size}
  height={size}
  viewBox="0 0 {size}
  {size}"
  fill="none"
  xmlns="http://www.w3.org/2000/svg">
  <circle
    class="opaque"
    cx={center}
    cy={center}
    r={radius}
    stroke={variant === 'inverted' ? 'var(--color-background)' : color}
    stroke-width={strokeWidth} />
  <circle
    style="--origin: {center}px {center}px"
    class:spin
    cx={center}
    cy={center}
    r={radius}
    transform="rotate(-90, {center}, {center})"
    stroke={variant === 'inverted' ? 'var(--color-background)' : color}
    stroke-width={strokeWidth}
    stroke-dasharray="{dashArray}, {circumference}" />
  {#if variant === 'inverted'}
    <circle cx={center} cy={center} r={radius} fill={color} />
  {/if}
  {#if state === 'positive'}
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M21.7071 12.2929C22.0976 12.6834 22.0976 13.3166 21.7071
      13.7071L14.7071 20.7071C14.3166 21.0976 13.6834 21.0976 13.2929
      20.7071L9.29289 16.7071C8.90237 16.3166 8.90237 15.6834 9.29289
      15.2929C9.68342 14.9024 10.3166 14.9024 10.7071 15.2929L14 18.5858L20.2929
      12.2929C20.6834 11.9024 21.3166 11.9024 21.7071 12.2929Z"
      fill={color} />
  {:else if state === 'negative'}
    <path
      d="M16 22C15.6022 22 15.2206 21.842 14.9393 21.5607C14.658 21.2794 14.5
      20.8978 14.5 20.5C14.5 20.1022 14.658 19.7207 14.9393 19.4394C15.2206
      19.158 15.6022 19 16 19C16.3978 19 16.7793 19.158 17.0607 19.4394C17.342
      19.7207 17.5 20.1022 17.5 20.5C17.5 20.8978 17.342 21.2794 17.0607
      21.5607C16.7793 21.842 16.3978 22 16 22ZM17 16.1C16.87 17.3 15.12 17.3 15
      16.1L14.5 11.1C14.486 10.9605 14.5014 10.8197 14.5454 10.6866C14.5893
      10.5534 14.6607 10.4311 14.755 10.3273C14.8493 10.2236 14.9644 10.1409
      15.0927 10.0845C15.2211 10.0281 15.3598 9.99931 15.5 10H16.5C16.6402
      9.99931 16.7789 10.0281 16.9073 10.0845C17.0356 10.1409 17.1507 10.2236
      17.2449 10.3273C17.3392 10.4311 17.4107 10.5534 17.4546 10.6866C17.4986
      10.8197 17.514 10.9605 17.5 11.1L17 16.1Z"
      fill={color} />
  {/if}
</svg>
