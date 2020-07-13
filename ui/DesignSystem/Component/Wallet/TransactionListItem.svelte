<script>
  export let tx = null;
  import {
    headerIcon,
    formatMessage,
    formatSubject,
    iconState,
    iconProgress,
    statusText,
    subjectAvatarShape,
  } from "../../../src/transaction.ts";
  import { microRadToRad } from "../../../src/currency.ts";

  import { Rad } from "../../Component";
  import { Avatar, Caption, Icon, Text, Title } from "../../Primitive";

  const subject = formatSubject(tx.messages[0]);
  console.log(subject);

  const formatDate = (timestamp, option) => {
    const time = new Date(timestamp * 1000);
    const day = {
      day: "numeric",
    };
    const month = {
      month: "long",
    };

    const options = option === "day" ? day : month;
    return `${time.toLocaleString(undefined, options)}`;
  };

  let avatar;
  const updateAvatar = async () => (avatar = await subject.avatarSource);

  $: updateAvatar();
</script>

<style>
  .item {
    display: grid;
    align-items: center;
    grid-template-columns: 2.5rem auto auto;
    grid-column-gap: 1rem;
    padding: 0.75rem;
    border-top: 1px solid var(--color-foreground-level-2);
    cursor: pointer;
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
    {#if tx.state.type === 'settled'}
      <Caption
        style="color: var(--color-foreground-level-3); margin-bottom: 1px;">
        {formatDate(tx.state.timestamp.secs, 'month').substring(0, 3)}
      </Caption>
      <Title>{formatDate(tx.state.timestamp.secs, 'day')}</Title>
    {:else}
      <Caption style="color: var(--color-foreground-level-3)">
        {formatDate(tx.timestamp.secs, 'month').substring(0, 3)}
      </Caption>
      <Title>{formatDate(tx.timestamp.secs, 'day')}</Title>
    {/if}
  </div>
  <div class="description">
    <svelte:component this={Icon[headerIcon(tx.messages[0])]} />
    <Title style="margin: 0 .5rem">{formatMessage(tx.messages[0])}</Title>
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
      <Title style="color: var(--color-foreground-level-5)" dataCy="subject">
        {subject.name}
      </Title>
    {/if}
  </div>
  <div class="meta">
    {#if tx.state.type !== 'settled'}
      <div class="status">
        {#if iconState(tx.state) === 'negative'}
          <Icon.Important
            style="margin-right: 8px; fill: var(--color-negative)" />
        {:else if iconState(tx.state) === 'positive'}
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
          style="align-self: center; color: var(--color-foreground-level-6);">
          {statusText(tx.state)}
        </Text>
      </div>
    {/if}
    <Rad rad={`-${microRadToRad(tx.fee)}`} usd={`-${microRadToRad(tx.fee)}`} />
  </div>
</div>
