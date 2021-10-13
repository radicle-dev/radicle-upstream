<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let checked: boolean = false;
</script>

<style>
  .wrapper {
    display: inline-flex;
    position: relative;
    user-select: none;
    cursor: pointer;
  }

  input {
    height: 24px;
    width: 24px;
    -webkit-appearance: none;
    appearance: none;
    outline: none;
    cursor: pointer;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
  }

  input:hover,
  input:checked:hover {
    background-color: var(--color-foreground-level-2);
  }

  input:checked {
    background: url('data:image/svg+xml;utf8,<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M16.4961 8.13174C16.9757 8.40575 17.1423 9.01661 16.8682 9.49613L13.8426 14.791C13.0247 16.2222 11.0698 16.484 9.90422 15.3184L8.29289 13.7071C7.90237 13.3166 7.90237 12.6834 8.29289 12.2929C8.68342 11.9024 9.31658 11.9024 9.70711 12.2929L11.3184 13.9042C11.5516 14.1373 11.9425 14.085 12.1061 13.7987L15.1318 8.50385C15.4058 8.02433 16.0166 7.85773 16.4961 8.13174Z" fill="%2390A0AF"/></svg>')
      no-repeat center center;
  }

  span {
    margin-left: 14px;
  }
</style>

<label class="wrapper">
  <input type="checkbox" class="button-transition" bind:checked />
  <span>
    <slot />
  </span>
</label>
