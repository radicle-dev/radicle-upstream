<script>
  export let style = null;
  export let dataCy = null;
</script>

<svg
  on:click
  style={`flex-shrink:0;${  style}`}
  data-cy={dataCy}
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="var(--color-foreground-level-5)"
  xmlns="http://www.w3.org/2000/svg">
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M5.9 6C5.40294 6 5 6.40294 5 6.9V16.1C5 16.5971 5.40294 17 5.9
    17H18.1C18.5971 17 19 16.5971 19 16.1V8.9C19 8.40294 18.5971 8 18.1
    8H12.618C11.8605 8 11.168 7.572 10.8292 6.89443L10.6307 6.49751C10.4783
    6.1926 10.1666 6 9.82574 6H5.9ZM3 6.9C3 5.29837 4.29837 4 5.9
    4H9.82574C10.9242 4 11.9283 4.62061 12.4196 5.60308L12.618 6H18.1C19.7016 6
    21 7.29837 21 8.9V16.1C21 17.7016 19.7016 19 18.1 19H5.9C4.29837 19 3
    17.7016 3 16.1V6.9Z" />
</svg>
