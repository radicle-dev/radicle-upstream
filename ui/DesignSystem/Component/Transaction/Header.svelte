<script>
  import {
    formatMessage,
    headerIcon,
    subjectAvatarShape,
    timestamp,
  } from "../../../src/transaction.ts";

  import { Avatar, Icon } from "../../../DesignSystem/Primitive";

  export let avatar = null;
  export let transaction = null;
  export let subject = null;
  export let accountId = null;
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

<div data-cy="summary" variant={transaction.id ? 'top' : 'single'}>
  <div data-cy="message" class="container">
    <div class="icon">
      <svelte:component
        this={Icon[headerIcon(transaction.messages[0])]}
        style="fill: var(--color-primary)" />
    </div>
    <h2>{formatMessage(transaction.messages[0], accountId)}</h2>
    <caption style="display: inline-flex; margin:16px 0; max-width: 20rem;">
      {#if avatar}
        <Avatar
          title={subject.name}
          imageUrl={avatar.url}
          avatarFallback={avatar.emoji && avatar}
          variant={subjectAvatarShape(subject.type)}
          style="color: var(--color-foreground)"
          dataCy="subject-avatar" />
      {:else}
        <p class="typo-text-bold typo-overflow-ellipsis" data-cy="subject">
          {subject.name}
        </p>
      {/if}
    </caption>
    {#if transaction.state}
      <p style="color:var(--color-foreground-level-4);">
        {timestamp(transaction.state)}
      </p>
    {/if}
  </div>
</div>
