<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import { Avatar, Icon } from "../../DesignSystem/Primitive";

  import * as style from "../../src/style";

  export let orgAddress: string;
  export let gnosisSafeAddress: string;
  export let threshold: number;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }
  .name {
    margin-bottom: 0.5rem;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
</style>

<div style="display: flex">
  <Avatar
    style="margin-right: 2rem;"
    size="huge"
    variant="square"
    avatarFallback={radicleAvatar.generate(
      orgAddress,
      radicleAvatar.Usage.Any
    )} />

  <div class="metadata">
    <h1 data-cy="entity-name" class="typo-overflow-ellipsis name">
      {style.ellipsed(orgAddress)}
    </h1>
    <div class="row">
      <Icon.Gnosis />{gnosisSafeAddress}
    </div>
    <div class="row">
      <Icon.Orgs />
      {threshold}
      {threshold === 1 ? "signature" : "signatures"} required for quorum
    </div>
  </div>
</div>
