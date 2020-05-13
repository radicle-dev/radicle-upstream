<script>
  import { Title, Text, Markdown } from "../../DesignSystem/Primitive";

  $: openClass = issue.open ? "open" : "closed";

  // dummy issue content
  const issue = {
    hash: "blka",
    open: true,
    title: "Split server into CentralChain, and ChainApi",
    description: `
### Outstanding work for member management.

**proxy**

- [x] list members for org
- [ ] add member endpoint
- [ ] remove member endpoint
- [ ] refactor user org list to filter by handle instead key

**ui**

- [x] add member flow
- [ ] member list
- [ ] ensure correct handling of member txs in transaction center

Part of #277
    `,
    author: "julien",
    replies: 12,
    created_at: "12 days",
    updated_at: "1 day",
    closed_at: null,
  };
</script>

<style>
  .container {
    max-width: 71.25rem;
    margin: 0 auto;
    padding: 32px 0;
    min-width: 500px;
  }
  .issueHeader {
    padding: 0 16px 24px 16px;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
  .metadata {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-top: 12px;
  }
  .stateBadge {
    color: var(--color-background);
    padding: 8px 12px;
    border-radius: 4px;
    margin-right: 16px;
  }
  .stateBadge.open {
    background-color: var(--color-positive);
  }
  .stateBadge.closed {
    background-color: var(--color-negative);
  }

  .issueDescription {
    padding: 24px 16px 32px 16px;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
</style>

<div class="container">
  <div class="issueHeader">
    <Title variant="large">{issue.title}</Title>
    <div class="metadata">
      <div class="stateBadge {openClass}">
        <Text variant="small">{issue.open ? 'Open' : 'Closed'}</Text>
      </div>
      {#if issue.open}
        <Text style="color: var(--color-foreground-level-5)">
          Openend {issue.created_at} by
          <span
            style="color: var(--color-foreground-level-6); font-family:
            var(--typeface-medium);">
            {issue.author}
          </span>
        </Text>
      {:else}
        <Text style="color: var(--color-foreground-level-5)">
          closed {issue.closed_at} by
          <span
            style="color: var(--color-foreground-level-6); font-family:
            var(--typeface-medium);">
            {issue.author}
          </span>
        </Text>
      {/if}
    </div>
  </div>
  <div class="issueDescription">
    <Markdown content={issue.description} />
  </div>
</div>
