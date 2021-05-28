<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import { Avatar, Icon } from "../../DesignSystem/Primitive";

  import * as style from "../../src/style";

  export let orgAddress: string;
  export let gnosisSafeAddress: string;
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
  <Avatar
    style="margin-right: 32px"
    size="huge"
    variant="square"
    avatarFallback={radicleAvatar.generate(
      orgAddress,
      radicleAvatar.Usage.Any
    )} />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis">
      {style.ellipsed(orgAddress)}
    </h1>
    <div class="safe-addr">
      <Icon.Gnosis />{gnosisSafeAddress}
    </div>
  </div>
</div>
