<script>
  import { SidebarLayout } from "../DesignSystem/Component";
  import { Icon, Input, Title } from "../DesignSystem/Primitive";

  import Project from "./Discovery/Project.svelte";

  const projectFeed = [
    {
      domain: "radicle",
      name: "radicle-upstream",
      description: "A nice app for nice things",
      shareableEntityIdentifier:
        "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe",
      stats: {
        commits: 220,
        branches: 12,
        contributors: 4,
      },
      maintainers: [
        {
          handle: "rafalca",
          avatar: {
            background: {
              r: 255,
              g: 0,
              b: 179,
            },
            emoji: "🖕🏼",
          },
        },
      ],
    },
    {
      domain: "radicle",
      name: "radicle-surf",
      description: "A nice app for nice things",
      shareableEntityIdentifier:
        "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe",
      stats: {
        commits: 220,
        branches: 12,
        contributors: 4,
      },
      maintainers: [
        {
          handle: "rafalca",
          avatar: {
            background: {
              r: 255,
              g: 0,
              b: 179,
            },
            emoji: "🖕🏼",
          },
        },
      ],
    },
    {
      domain: "radicle",
      name: "radicle-link",
      description: "A nice app for nice things",
      shareableEntityIdentifier:
        "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe",
      stats: {
        commits: 220,
        branches: 12,
        contributors: 4,
      },
      maintainers: [
        {
          handle: "rafalca",
          avatar: {
            background: {
              r: 255,
              g: 0,
              b: 179,
            },
            emoji: "🖕🏼",
          },
        },
      ],
    },
    {
      domain: "radicle",
      name: "radicle-registry",
      description: "A nice app for nice things",
      shareableEntityIdentifier:
        "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe",
      stats: {
        commits: 220,
        branches: 12,
        contributors: 4,
      },
      maintainers: [
        {
          handle: "rafalca",
          avatar: {
            background: {
              r: 255,
              g: 0,
              b: 179,
            },
            emoji: "🖕🏼",
          },
        },
      ],
    },
    {
      domain: "radicle",
      name: "radicle-keystore",
      description: "A nice app for nice things",
      shareableEntityIdentifier:
        "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe",
      stats: {
        commits: 220,
        branches: 12,
        contributors: 4,
      },
      maintainers: [
        {
          handle: "rafalca",
          avatar: {
            background: {
              r: 255,
              g: 0,
              b: 179,
            },
            emoji: "🖕🏼",
          },
        },
      ],
    },
  ];
</script>

<style>
  .container {
    margin: 64px 100px;
  }

  .projects {
    display: grid;
    grid-template-columns: 1fr 1fr;
    column-gap: 32px;
    row-gap: 32px;
  }
</style>

<SidebarLayout>
  <div class="container">
    <Title variant="huge" style="margin-bottom: 28px;">Discover</Title>
    <Input.Text
      placeholder="Filter projects"
      showLeftItem
      style="margin-bottom: 28px; max-width: 350px;">
      <div slot="left">
        <Icon.Search />
      </div>
    </Input.Text>

    <div class="projects">
      {#each projectFeed as project}
        <Project {project} />
      {/each}
    </div>
  </div>
</SidebarLayout>
