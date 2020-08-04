<script>
  import {
    costSummary,
    headerIcon,
    formatMessage,
    formatSubject,
    formatDate,
    iconState,
    iconProgress,
    isIncoming,
    statusText,
    subjectAvatarShape,
    StateType,
    IconState,
  } from "../../../src/transaction.ts";
  import Rad from "../Rad.svelte";
  import { Avatar, Icon, Text, Title } from "../../Primitive";

  export let tx = null;
  export let accountId = null;

  const subject = formatSubject(tx.messages[0], accountId);

  let avatar;
  const updateAvatar = async () => (avatar = await subject.avatarSource);

  const summary = costSummary(tx);

  $: updateAvatar();
</script>

<style>
  .item {
    display: grid;
    align-items: center;
    grid-template-columns: 2.5rem auto auto;
    grid-column-gap: 1rem;
    padding: 0.75rem;
    border-bottom: 1px solid var(--color-foreground-level-2);
    cursor: pointer;
  }
  .item:last-child {
    border: none;
  }
  .item:hover {
    background-color: var(--color-foreground-level-1);
  }
  .date {
    text-align: center;
    color: var(--color-foreground-level-5);
  }
  .description {
    display: flex;
  }
  .meta {
    display: flex;
    justify-content: flex-end;
    align-items: center;
  }
  .status {
    display: flex;
    align-items: center;
    margin-right: 1rem;
  }
</style>

<div class="item" on:click>
  <div class="date">
    {#if tx.state.type === StateType.Settled}
      <p
        class="all-caps"
        style="color: var(--color-foreground-level-3); margin-bottom: 1px;">
        {formatDate(tx.state.timestamp.secs, 'month').substring(0, 3)}
      </p>
      <Title>{formatDate(tx.state.timestamp.secs, 'day')}</Title>
    {:else}
      <p class="all-caps" style="color: var(--color-foreground-level-3)">
        {formatDate(tx.timestamp.secs, 'month').substring(0, 3)}
      </p>
      <Title>{formatDate(tx.timestamp.secs, 'day')}</Title>
    {/if}
  </div>
  <div class="description">
    <svelte:component this={Icon[headerIcon(tx.messages[0])]} />
    <Title dataCy="message" style="margin: 0 .5rem; white-space: nowrap;">
      {formatMessage(tx.messages[0], accountId)}
    </Title>
    {#if avatar}
      <Avatar
        title={subject.name}
        size="small"
        imageUrl={avatar.url}
        avatarFallback={avatar.emoji && avatar}
        variant={subjectAvatarShape(subject.type)}
        style="--title-color: var(--color-foreground-level-5);"
        dataCy="subject-avatar" />
    {:else}
      <Title
        truncate
        style="color: var(--color-foreground-level-5); max-width: 15rem;"
        dataCy="subject">
        {subject.name}
      </Title>
    {/if}
  </div>
  <div class="meta">
    {#if tx.state.type !== StateType.Settled}
      <div class="status">
        {#if iconState(tx.state) === IconState.Negative}
          <Icon.Important
            style="margin-right: 8px; fill: var(--color-negative)" />
        {:else if iconState(tx.state) === IconState.positive}
          <Icon.Check
            variant="filled"
            style="margin-right: 8px; fill: var(--color-positive)" />
        {:else}
          <Icon.TransactionState
            progress={iconProgress(tx.state)}
            style="margin-right: 8px;"
            variant="small"
            state={iconState(tx.state)} />
        {/if}
        <Text
          style="align-self: center; color: var(--color-foreground-level-6);
          white-space: nowrap;">
          {statusText(tx.state)}
        </Text>
      </div>
    {/if}

    {#if isIncoming(tx.messages[0], accountId)}
      <Rad
        variant="debit"
        rad={summary.transferAmount.rad}
        usd="{summary.transferAmount.usd}}" />
    {:else}
      <Rad
        variant="credit"
        rad={summary.total.rad}
        usd="{summary.total.usd}}" />
    {/if}

  </div>
</div>
