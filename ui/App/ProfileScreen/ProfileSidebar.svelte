<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as ensResolver from "ui/src/org/ensResolver";

  import EthereumIcon from "design-system/icons/Ethereum.svelte";
  import GithubIcon from "design-system/icons/Github.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import GnosisIcon from "design-system/icons/Gnosis.svelte";
  import ProposalsIcon from "design-system/icons/Proposals.svelte";
  import ServerIcon from "design-system/icons/Server.svelte";
  import TwitterIcon from "design-system/icons/Twitter.svelte";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

</script>

<style>
  aside {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    height: fit-content;
  }

  .row {
    padding: 1.5rem;
    color: var(--color-foreground-level-6);
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .row:not(:last-of-type) {
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
</style>

<aside>
  <div class="row">
    <div class="title">
      <TwitterIcon />
      <span class="typo-text-bold">Twitter</span>
    </div>
    <a
      class="typo-link"
      href={`https://www.twitter.com/juliendonck`}>twitter.com/juliendonck</a>
  </div>
</aside>
