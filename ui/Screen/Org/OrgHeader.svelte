<script lang="typescript">
  import { Avatar, Icon } from "../../DesignSystem/Primitive";

  import * as style from "../../src/style";

  export let name: string | undefined = undefined;
  export let gnosisSafeWalletAddress: string;
  export let gnosisSafeAddress: string | null = null;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
  .safe-addr {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>

<div style="display: flex">
  <Avatar style="margin-right: 32px" size="huge" variant="square" />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis" title={name}>
      {name || style.ellipsed(gnosisSafeWalletAddress)}
    </h1>
    <div class="safe-addr">
      <Icon.Gnosis />
      {gnosisSafeAddress || 'loading…'}
    </div>
  </div>
</div>
