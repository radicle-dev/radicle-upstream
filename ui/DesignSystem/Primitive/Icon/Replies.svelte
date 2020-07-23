<script>
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = null;
  export let style = null;
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    d="M8 9C7.44772 9 7 9.44772 7 10C7 10.5523 7.44772 11 8 11H16C16.5523 11 17
    10.5523 17 10C17 9.44772 16.5523 9 16 9H8Z" />
  <path
    d="M8 12C7.44772 12 7 12.4477 7 13C7 13.5523 7.44772 14 8 14H13C13.5523 14
    14 13.5523 14 13C14 12.4477 13.5523 12 13 12H8Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M6 5C4.34315 5 3 6.34315 3 8V15C3 16.6569 4.34315 18 6 18H16.5858L18.0808
    19.4951C19.1581 20.5723 21 19.8094 21 18.2859V8C21 6.34315 19.6569 5 18
    5H6ZM5 8C5 7.44772 5.44772 7 6 7H18C18.5523 7 19 7.44772 19
    8V17.5858L17.7071 16.2929C17.5196 16.1054 17.2652 16 17 16H6C5.44772 16 5
    15.5523 5 15V8Z" />
</IconWrapper>
