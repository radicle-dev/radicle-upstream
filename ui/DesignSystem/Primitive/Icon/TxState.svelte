<script>
  export let style = null;
  export let state = "caution"; // negative | caution | positive
  export let progress = null; // 0-100% of progress, overwrites the defaults
  export let variant = "regular"; // small | regular

  const size = {
    regular: 32,
    small: 16
  }[variant];

  const strokeWidth = {
    regular: 4,
    small: 2
  }[variant];

  const center = size / 2;
  const radius = size / 2 - strokeWidth / 2;
  const circumference = 2 * Math.PI * radius;

  const color = {
    caution: "var(--color-orange)",
    positive: "var(--color-green)",
    negative: "var(--color-red)"
  }[state];

  const defaultDashLength = {
    caution: progress || 100 / 6,
    positive: 100,
    negative: 0
  };

  const dashLength =
    progress === 0 ? 100 / 6 : progress || defaultDashLength[state];

  const rotate = progress === 0;
</script>

<style>
  .opaque {
    opacity: 0.3;
  }

  .rotate {
    transform-origin: var(--origin);
    animation: rotate 5s infinite linear;
  }

  @keyframes rotate {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(359deg);
    }
  }
</style>

<svg
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
    stroke={color}
    stroke-width={strokeWidth} />
  <circle
    style="--origin: {center}px {center}px"
    class:rotate
    cx={center}
    cy={center}
    r={radius}
    transform="rotate(-90, {center}, {center})"
    stroke={color}
    stroke-width={strokeWidth}
    stroke-dasharray="{(dashLength * circumference) / 100}, {circumference}" />
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
      d="M16.505 21C16.1072 21 15.7257 20.842 15.4444 20.5607C15.163 20.2794
      15.005 19.8978 15.005 19.5C15.005 19.1022 15.163 18.7207 15.4444
      18.4394C15.7257 18.158 16.1072 18 16.505 18C16.9028 18 17.2844 18.158
      17.5657 18.4394C17.847 18.7207 18.005 19.1022 18.005 19.5C18.005 19.8978
      17.847 20.2794 17.5657 20.5607C17.2844 20.842 16.9028 21 16.505 21ZM17.505
      15.1C17.375 16.3 15.625 16.3 15.505 15.1L15.005 10.1C14.991 9.96054
      15.0065 9.81967 15.0504 9.68656C15.0943 9.55344 15.1658 9.43105 15.2601
      9.32733C15.3544 9.2236 15.4694 9.14086 15.5977 9.08447C15.7261 9.02809
      15.8648 8.99931 16.005 9.00001H17.005C17.1452 8.99931 17.2839 9.02809
      17.4123 9.08447C17.5406 9.14086 17.6557 9.2236 17.75 9.32733C17.8443
      9.43105 17.9157 9.55344 17.9596 9.68656C18.0036 9.81967 18.019 9.96054
      18.005 10.1L17.505 15.1Z"
      fill={color} />
  {/if}
</svg>
