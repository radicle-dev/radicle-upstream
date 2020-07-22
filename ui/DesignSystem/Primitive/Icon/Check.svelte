<script>
  export let style = null;
  export let variant = "normal"; // normal | filled
  export let size = "normal"; // normal | small
</script>

{#if size === 'normal'}
  <svg
    on:click
    style={`flex-shrink:0;${  style}`}
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="var(--color-foreground-level-5)"
    xmlns="http://www.w3.org/2000/svg">
    {#if variant === 'normal'}
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M18.7071 8.29289C19.0976 8.68342 19.0976 9.31658 18.7071
        9.70711L11.7071 16.7071C11.3166 17.0976 10.6834 17.0976 10.2929
        16.7071L6.29289 12.7071C5.90237 12.3166 5.90237 11.6834 6.29289
        11.2929C6.68342 10.9024 7.31658 10.9024 7.70711 11.2929L11
        14.5858L17.2929 8.29289C17.6834 7.90237 18.3166 7.90237 18.7071 8.29289Z" />
    {:else if variant === 'filled'}
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M12 24C18.6274 24 24 18.6274 24 12C24 5.37258 18.6274 0 12 0C5.37258
        0 0 5.37258 0 12C0 18.6274 5.37258 24 12 24ZM18.7071 9.70711C19.0976
        9.31658 19.0976 8.68342 18.7071 8.29289C18.3166 7.90237 17.6834 7.90237
        17.2929 8.29289L11 14.5858L7.70711 11.2929C7.31658 10.9024 6.68342
        10.9024 6.29289 11.2929C5.90237 11.6834 5.90237 12.3166 6.29289
        12.7071L10.2929 16.7071C10.6834 17.0976 11.3166 17.0976 11.7071
        16.7071L18.7071 9.70711Z" />
    {/if}
  </svg>
{:else if size === 'small'}
  <svg
    on:click
    style={`flex-shrink:0;${  style}`}
    width="16"
    height="16"
    viewBox="0 0 16 16"
    fill="var(--color-foreground-level-5)"
    xmlns="http://www.w3.org/2000/svg">
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M13.7071 4.29289C14.0976 4.68342 14.0976 5.31658 13.7071
      5.70711L7.70711 11.7071C7.31658 12.0976 6.68342 12.0976 6.29289
      11.7071L3.29289 8.70711C2.90237 8.31658 2.90237 7.68342 3.29289
      7.29289C3.68342 6.90237 4.31658 6.90237 4.70711 7.29289L7 9.58579L12.2929
      4.29289C12.6834 3.90237 13.3166 3.90237 13.7071 4.29289Z" />
  </svg>
{/if}
