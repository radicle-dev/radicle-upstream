<script>
  import { Header, Title, Text, Markdown } from "../../DesignSystem/Primitive";
  import { Comment, Timeline } from "../../DesignSystem/Component";

  $: openClass = issue.open ? "open" : "closed";

  // dummy issue content
  const issue = {
    hash: "blka",
    open: true,
    title: "Split server into CentralChain, and ChainApi",
    description: `
### Outstanding work for member management.

\`code\`

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
    author: {
      handle: "julien",
      avatar_url:
        "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
    },
    replies: 12,
    created_at: "Friday, August 9th",
    updated_at: "1 day",
    closed_at: null,
    timeline: [
      {
        variant: "open-issue",
        user: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        time_ago: "2 weeks ago",
      },
      {
        variant: "comment",
        user: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        comment: "This is a `code snippet` ",
        time_ago: "2 weeks ago",
      },
      {
        variant: "close-issue",
        user: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        time_ago: "2 weeks ago",
      },
      {
        variant: "reopen-issue",
        user: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        time_ago: "2 weeks ago",
      },
      {
        variant: "label",
        user: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        labels: [
          {
            title: "ui",
            color: "var(--color-primary-level-2)",
          },
          {
            title: "infra",
            color: "var(--color-secondary-level-2)",
          },
        ],
        time_ago: "2 weeks ago",
      },
      {
        variant: "assign",
        user: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        assignee: {
          handle: "julien",
          avatar_url:
            "https://avatars3.githubusercontent.com/u/2326909?s=24&u=1968a2daca982c6deaf89ec71c16d94333092fe3&v=4",
        },
        time_ago: "2 weeks ago",
      },
    ],
  };
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 0 auto;
    padding: var(--content-padding);
    min-width: var(--content-min-width);
  }

  .metadata {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-top: 12px;
  }
  .state-badge {
    color: var(--color-background);
    padding: 8px 12px;
    border-radius: 4px;
    margin-right: 16px;
  }
  .state-badge.open {
    background-color: var(--color-positive);
  }
  .state-badge.closed {
    background-color: var(--color-negative);
  }

  article {
    padding: 24px 16px 32px 16px;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
</style>

<div class="container">
  <Header.Back
    style="padding: 0 16px 24px 16px; border-bottom: 1px solid
    var(--color-foreground-level-3);">
    <Title variant="large">{issue.title}</Title>
    <div class="metadata">
      <div class="state-badge {openClass}">
        <Text variant="small">{issue.open ? 'Open' : 'Closed'}</Text>
      </div>
      <Text style="color: var(--color-foreground-level-5)">
        {issue.open ? 'Opened' : 'Closed'} {issue.created_at} by
        <span
          style="color: var(--color-foreground-level-6); font-family:
          var(--typeface-medium);">
          {issue.author.handle}
        </span>
      </Text>
    </div>
  </Header.Back>
  <article>
    <Markdown content={issue.description} />
  </article>
  <section class="timeline">
    <Timeline
      style="margin-top: 24px;"
      startDate={issue.created_at}
      items={issue.timeline} />
  </section>
  <Comment user={issue.author} style="margin-bottom: 64px;" />
</div>
