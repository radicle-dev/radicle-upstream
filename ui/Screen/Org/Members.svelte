<script>
  import { Avatar, Icon, Title } from "../../DesignSystem/Primitive";
  import { AdditionalActionsDropdown } from "../../DesignSystem/Component";

  const showRegisteredBadge = true;

  const menuItems = [
    {
      icon: Icon.Member,
      title: "Go to member profile",
      event: () => console.log("go to member profile"),
    },
    {
      icon: Icon.Cross,
      title: "Remove member",
    },
  ];
</script>

<style>
  .container {
    display: flex;
    justify-content: space-between;
  }

  .member {
    display: flex;
  }
</style>

<div class="container">
  <div class="member">
    <Avatar />
    <Title>Bessie Murphy</Title>
    {#if showRegisteredBadge}
      <Icon.Badge />
    {/if}
  </div>

  <AdditionalActionsDropdown {menuItems} />
</div>
