<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M5 11C4.44771 11 4 11.4477 4 12L4 17C4 18.6569 5.34315 20 7 20L17 20C18.6569 20 20 18.6569 20 17L20 12C20 11.4477 19.5523 11 19 11C18.4477 11 18 11.4477 18 12L18 17C18 17.5523 17.5523 18 17 18L7 18C6.44772 18 6 17.5523 6 17L6 12C6 11.4477 5.55229 11 5 11Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 11C12.5523 11 13 10.5523 13 10L13 5C13 4.44772 12.5523 4 12 4C11.4477 4 11 4.44772 11 5L11 10C11 10.5523 11.4477 11 12 11Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M7.29289 9.29289C7.68342 8.90237 8.31658 8.90237 8.70711 9.29289L12 12.5858L15.2929 9.29289C15.6834 8.90237 16.3166 8.90237 16.7071 9.29289C17.0976 9.68342 17.0976 10.3166 16.7071 10.7071L12.7071 14.7071C12.3166 15.0976 11.6834 15.0976 11.2929 14.7071L7.29289 10.7071C6.90237 10.3166 6.90237 9.68342 7.29289 9.29289Z" />
</IconWrapper>
