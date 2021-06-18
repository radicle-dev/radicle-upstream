<script lang="typescript">
  import type * as org from "ui/src/org";

  import { ellipsed } from "ui/src/style";

  import { List } from "ui/DesignSystem/Component";

  export let members: org.Member[];
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }

  .member-name {
    color: var(--color-foreground-level-6);
  }

  .member-address {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container">
  <List items={members} let:item={member} styleHoverState={false}>
    <div class="list-item">
      <div class="member-name typo-text-bold">{member}</div>
      <div class="member-address">{ellipsed(member)}</div>
    </div>
  </List>
</div>
