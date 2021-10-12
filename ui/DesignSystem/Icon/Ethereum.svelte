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
    d="M13.4837 3.23366C12.727 2.30942 11.3342 2.25021 10.5018 3.10689L5.2828
       8.47811C4.89793 8.87421 4.90703 9.50731 5.30313 9.89218C5.69923 10.277
       6.33233 10.2679 6.7172 9.87185L11.9362 4.50063L17.2485 10.9892L13
       12.1424V9C13 8.44772 12.5523 8 12 8C11.4477 8 11 8.44772 11
       9V12.1424L7.84333 11.2856C6.05795 10.801 4.62821 12.7918 5.65778
       14.3288L10.4047 21.4152C11.16 22.5428 12.7951 22.6067 13.6361
       21.5414L18.7849 15.0196C19.1271 14.5861 19.0531 13.9573 18.6197
       13.6151C18.1862 13.2729 17.5574 13.3469 17.2151 13.7803L12.0663
       20.3021L7.31943 13.2157L11 14.2147V18C11 18.5523 11.4477 19 12
       19C12.5523 19 13 18.5523 13 18V14.2147L17.7724 12.9194C19.1715 12.5396
       19.7144 10.844 18.796 9.72224L13.4837 3.23366Z" />
</IconWrapper>
