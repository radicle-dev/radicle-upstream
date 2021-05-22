<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy = "";
  export let style = "";
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M7 5C6.44771 5 6 5.44772 6 6V12H18V6C18 5.44772 17.5523 5 17 5H7ZM18.3406 14H5.65939L4.11393 17.6061C3.83113 18.2659 4.31516 19 5.03308 19H18.9669C19.6848 19 20.1689 18.2659 19.8861 17.6061L18.3406 14ZM4 12.7947V6C4 4.34315 5.34314 3 7 3H17C18.6569 3 20 4.34315 20 6V12.7947L21.7244 16.8182C22.5728 18.7978 21.1207 21 18.9669 21H5.03308C2.87933 21 1.42724 18.7978 2.27564 16.8182L4 12.7947Z" />
</IconWrapper>
