<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Icon from "./Icon";

  export let message = "Something went wrong";
</script>

<style>
  .error {
    color: white;
    background-color: var(--color-negative-level-2);
    vertical-align: middle;
    margin: 1rem;
    padding: 1rem;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: left;
  }
  .error p {
    padding-left: 0.5rem;
  }
</style>

<div class="error">
  <Icon.ExclamationCircle style="fill: white" />
  <p>Error: {message}</p>
</div>
