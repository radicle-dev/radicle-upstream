<script>
  import {
    formatMessage,
    headerIcon,
    SubjectType,
  } from "../../../src/transaction.ts";

  import { Avatar, Icon, Title, Text } from "../../../DesignSystem/Primitive";

  export let avatar;
  export let transaction;
  export let subject;

  const subjectAvatarShape = () => {
    switch (subject.type) {
      case SubjectType.User:
      case SubjectType.Member:
      case SubjectType.UserProject:
        return "circle";
      default:
        return "square";
    }
  };

  const icon = () => {
    const val = headerIcon(transaction.messages[0]);
    switch (val) {
      case "Project":
        return Icon.Source;
      case "User":
        return Icon.Member;
      case "Register":
        return Icon.Register;
      default:
        return Icon.Register;
    }
  };
</script>

<style>
  .container {
    text-align: center;
    background-color: var(--color-foreground-level-1);
    padding: 48px;
    border-radius: 4px;
    border: 1px solid var(--color-foreground-level-2);
    margin-bottom: 24px;
  }
  .icon {
    background: var(--color-background);
    border-radius: 28px;
    height: 56px;
    margin: 0 auto 16px;
    padding: 14px;
    width: 56px;
    border: 2px solid var(--color-primary-level-1);
  }
</style>

<div dataCy="summary" variant={transaction.id ? 'top' : 'single'}>
  <div data-cy="message" class="container">
    <div class="icon">
      <svelte:component this={icon()} style="fill: var(--color-primary)" />
    </div>
    <Title variant="big">{formatMessage(transaction.messages[0])}</Title>
    <caption style="display: inline-flex; margin:16px 0;">
      {#if avatar}
        <Avatar
          title={subject.name}
          imageUrl={avatar.url}
          avatarFallback={avatar.emoji && avatar}
          variant={subjectAvatarShape()}
          style="color: var(--color-foreground)"
          dataCy="subject-avatar" />
      {:else}
        <Title>{subject.name}</Title>
      {/if}
    </caption>
    <!-- TO DO Make the timestamp real -->
    <Text variant="normal" style="color:var(--color-foreground-level-4);">
      4 July 2020 16:20:21
    </Text>
  </div>
</div>
