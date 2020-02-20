<script>
  export let style = null;
  export let dataCy = null;
</script>

<svg
  on:click
  {style}
  data-cy={dataCy}
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="var(--color-gray)"
  xmlns="http://www.w3.org/2000/svg">
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M9.29289 16.7071C8.90237 16.3166 8.90237 15.6834 9.29289 15.2929L12.5858
    12L9.29289 8.70711C8.90237 8.31658 8.90237 7.68342 9.29289 7.29289C9.68342
    6.90237 10.3166 6.90237 10.7071 7.29289L14.7071 11.2929C15.0976 11.6834
    15.0976 12.3166 14.7071 12.7071L10.7071 16.7071C10.3166 17.0976 9.68342
    17.0976 9.29289 16.7071Z" />
</svg>
