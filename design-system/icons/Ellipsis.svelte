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
  export let animate: boolean = false;
</script>

<style>
  @keyframes pop {
    0%,
    25% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.3);
    }
    75%,
    100% {
      transform: scale(1);
    }
  }

  .dot-1,
  .dot-2,
  .dot-3 {
    animation: pop 0.8s infinite;
  }

  .dot-1 {
    transform-origin: 5px 12px;
  }

  .dot-2 {
    transform-origin: 12px 12px;
    animation-delay: 0.1s;
  }

  .dot-3 {
    transform-origin: 19px 12px;
    animation-delay: 0.2s;
  }
</style>

<IconWrapper on:click {dataCy} {style}>
  <path
    class:dot-1={animate}
    d="M5 14C6.10457 14 7 13.1046 7 12C7 10.8954 6.10457 10 5 10C3.89543 10 3
    10.8954 3 12C3 13.1046 3.89543 14 5 14Z" />
  <path
    class:dot-2={animate}
    d="M14 12C14 13.1046 13.1046 14 12 14C10.8954 14 10 13.1046 10 12C10 10.8954
    10.8954 10 12 10C13.1046 10 14 10.8954 14 12Z" />
  <path
    class:dot-3={animate}
    d="M21 12C21 13.1046 20.1046 14 19 14C17.8954 14 17 13.1046 17 12C17 10.8954
    17.8954 10 19 10C20.1046 10 21 10.8954 21 12Z" />
</IconWrapper>
