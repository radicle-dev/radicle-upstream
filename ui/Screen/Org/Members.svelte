<script>
  import { orgMemberTabStore } from "ui/src/org";
  import { ellipsed } from "ui/src/style";

  import { Icon } from "ui/DesignSystem/Primitive";
  import { List } from "ui/DesignSystem/Component";

  // TODO(rudolfs): make the link go to
  // `https://gnosis-safe.io/app/#/safes/${$orgMemberTabStore.gnosisSafeAddress}` for
  // mainnet
</script>

<style>
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }

  .metadata-container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    margin-bottom: 1.5rem;
    display: flex;
  }

  .metadata {
    width: 100%;
    display: flex;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    height: 4.5rem;
    align-items: center;
    color: var(--color-foreground-level-6);
    justify-content: space-between;
  }

  .left {
    display: flex;
    align-items: center;
  }

  .address {
    margin-right: 24px;
  }

  .address a {
    color: var(--color-foreground-level-6);
  }

  .member-name {
    color: var(--color-foreground-level-6);
  }

  .member-address {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="metadata-container">
  <div class="metadata">
    <span class="left typo-text">
      <Icon.Gnosis style="margin: 0 10px 0 27px;" />
      Managed by Gnosis Safe • Quorum
      {$orgMemberTabStore.threshold}/{$orgMemberTabStore.members.length}
    </span>
    <span class="address">
      <a
        href={`https://rinkeby.gnosis-safe.io/app/#/safes/${$orgMemberTabStore.gnosisSafeAddress}`}
        class="typo-link">
        {$orgMemberTabStore.gnosisSafeAddress}
      </a>↗</span>
  </div>
</div>

<List
  items={$orgMemberTabStore.members}
  let:item={member}
  styleHoverState={false}>
  <div class="list-item">
    <div class="member-name typo-text-bold">{member}</div>
    <div class="member-address">{ellipsed(member)}</div>
  </div>
</List>
