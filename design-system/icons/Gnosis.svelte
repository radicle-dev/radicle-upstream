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
    d="M12 4C7.58172 4 4 7.58172 4 12C4 16.4183 7.58172 20 12 20C16.4183 20 20
    16.4183 20 12C20 7.58172 16.4183 4 12 4ZM2 12C2 6.47715 6.47715 2 12
    2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2
    17.5228 2 12ZM12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12
    14C12.4103 14 12.789 13.8775 13.1052 13.6673C13.5651 13.3615 14.1858
    13.4864 14.4916 13.9463C14.7974 14.4062 14.6725 15.0269 14.2126
    15.3327C13.5785 15.7544 12.8164 16 12 16C9.79086 16 8 14.2091 8 12C8
    9.79086 9.79086 8 12 8C13.8638 8 15.4299 9.27477 15.874 11H17C17.5523 11 18
    11.4477 18 12C18 12.5523 17.5523 13 17 13H15C14.4477 13 14 12.5523 14 12C14
    10.8954 13.1046 10 12 10Z" />
</IconWrapper>
