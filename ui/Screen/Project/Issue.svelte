<script>
  import marked from "marked";

  import { Title, Text } from "../../DesignSystem/Primitive";

  $: openClass = issue.open ? "open" : "closed";

  // dummy issue content
  const issue = {
    hash: "blka",
    open: true,
    title: "Split server into CentralChain, and ChainApi",
    description: `# H1 heading

## H2 heading

### H3 heading

---

**bold text**

*italicized text*

---

1. First item
2. Second item
3. Third item

- First item
- Second item
- Third item

[Svelte](https://svelte.dev/)`,
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
    {@html marked(issue.description)}
  </div>
</div>
