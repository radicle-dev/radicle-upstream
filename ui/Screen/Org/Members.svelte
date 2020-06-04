<script>
  import { org as store } from "../../src/org.ts";

  import { Icon, Text, Title } from "../../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    List,
    Remote,
  } from "../../DesignSystem/Component";

  // TODO(sos): replace console.log's with actual navigation
  const menuItems = (member) => [
    {
      icon: Icon.Member,
      title: "Go to member profile",
      event: () => console.log(`go to ${member.handle}'s profile`),
    },
    {
      icon: Icon.Cross,
      title: "Remove member",
      event: () => console.log(`remove ${member.handle}`),
    },
  ];

  const select = (event) =>
    console.log(`go to ${event.detail.handle}'s profile`);
</script>

<style>
  .member {
    display: flex;
    justify-content: space-between;
    flex: 1;
    padding: 14px 15px 14px 12px;
  }

  .info {
    display: flex;
    align-items: center;
  }

  .membership-details {
    display: flex;
    align-items: center;
  }

  .pending {
    height: 22px;
    border: 1px solid var(--color-caution);
    border-radius: 2px;
    display: flex;
    justify-content: center;
    align-items: center;
    margin-right: 24px;
  }
</style>

<Remote {store} let:data={org}>
  <List
    items={org.members}
    let:item={member}
    on:select={select}
    dataCy="member-list">
    <div class="member">
      <div class="info">
        <Title>{member.handle}</Title>
        <Icon.Badge style="margin-left: 6px; fill: var(--color-primary);" />
      </div>

      <div class="membership-details">
        {#if member.pending}
          <div class="pending">
            <Text
              variant="tiny"
              style="color: var(--color-caution); padding: 8px;">
              Pending
            </Text>
          </div>
        {/if}
        <AdditionalActionsDropdown menuItems={menuItems(member)} />
      </div>
    </div>
  </List>
</Remote>
