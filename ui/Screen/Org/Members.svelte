<script>
  import { orgMembersStore } from "../../src/org";
  import { List } from "../../DesignSystem/Component";
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
</style>

<List items={$orgMembersStore} let:item={member} styleHoverState={false}>
  <div class="list-item">
    <div>emoji / name</div>
    {member}
  </div>
</List>
