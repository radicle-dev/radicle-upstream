<script>
  import * as notification from "../src/notification.ts";

  import {
    Avatar,
    Button,
    Code,
    Caption,
    Icon,
    Input,
    Text,
    Title,
    Numeric,
  } from "../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    Dropdown,
    ModalLayout,
    Notification,
    Placeholder,
    ProjectCard,
    Rad,
    Row,
    SegmentedControl,
    Stats,
    StepCounter,
    TrackToggle,
    TransactionAccordion,
    TransactionStatusbar,
    UserCard,
  } from "../DesignSystem/Component";

  import Section from "./DesignSystemGuide/Section.svelte";
  import Swatch from "./DesignSystemGuide/Swatch.svelte";
  import TypographySwatch from "./DesignSystemGuide/TypographySwatch.svelte";
  import { ValidationStatus } from "../src/validation.ts";

  const colors = Array.from(document.styleSheets)
    .filter(
      (sheet) =>
        sheet.href === null || sheet.href.startsWith(window.location.origin)
    )
    .reduce(
      (acc, sheet) =>
        (acc = [
          ...acc,
          ...Array.from(sheet.cssRules).reduce(
            (def, rule) =>
              (def =
                rule.selectorText === ":root"
                  ? [
                      ...def,
                      ...Array.from(rule.style).filter((name) =>
                        name.startsWith("--color")
                      ),
                    ]
                  : def),
            []
          ),
        ]),
      []
    );

  const user = {
    username: "Rudolfs Osins",
    avatar: "https://avatars.dicebear.com/v2/jdenticon/two.svg",
  };

  const avatarFallback1 = {
    emoji: "📐",
    background: {
      r: 24,
      g: 105,
      b: 216,
    },
  };

  const avatarFallback2 = {
    background: {
      r: 122,
      g: 112,
      b: 90,
    },
    emoji: "💡",
  };

  const additionalActionsDropdownItems = [
    {
      title: "Add something",
      icon: Icon.Plus,
      event: () => console.log("event(Add Something)"),
    },
    {
      title: "Add something else",
      icon: Icon.Plus,
      event: () => console.log("event(Add Something Else)"),
    },
    {
      title: "Send something",
      icon: Icon.ArrowUp,
      event: () => console.log("event(Send Something)"),
    },
    {
      title: "Send something",
      icon: Icon.ArrowUp,
      event: () => console.log("event(Send Something)"),
      disabled: true,
      tooltip: "This item is disabled because of reason!",
    },
  ];

  const transactions1 = [
    {
      message: "Project registration",
      state: "pending",
      progress: 50,
    },
    {
      message: "Member registration",
      state: "error",
    },
    {
      message: "Org registration",
      state: "success",
    },
  ];

  const transactions2 = [
    {
      message: "Project registration",
      state: "pending",
      progress: 70,
    },
    {
      message: "Org registration",
      state: "pending",
      progress: 0,
    },
  ];

  const transactions3 = [
    {
      message: "Org registration",
      state: "success",
    },
  ];

  const orgs = [
    {
      id: "%monadic",
      metadata: {
        name: "monadic",
      },
      avatarFallback: {
        emoji: "☔️",
        background: {
          b: 61,
          g: 187,
          r: 148,
        },
      },
    },
    {
      id: "%sveltejs",
      metadata: {
        name: "sveltejs",
      },
      avatarFallback: {
        emoji: "🚊",
        background: {
          b: 112,
          g: 27,
          r: 205,
        },
      },
    },
  ];

  const identity = {
    id: "123abcd.git",
    shareableEntityIdentifier: "cloudhead@123abcd.git",
    metadata: {
      handle: "cloudhead",
      displayName: "Alexis Sellier",
      avatarUrl: "https://avatars1.githubusercontent.com/u/40774",
    },
    registered: null,
    avatarFallback: { background: { r: 122, g: 112, b: 90 }, emoji: "💡" },
  };

  const dropdownOptions1 = [
    { variant: "text", value: "1", textProps: { title: "Option 1" } },
    {
      variant: "text",
      value: "2",
      textProps: { title: "Longer option keeps going" },
    },
    { variant: "text", value: "3", textProps: { title: "Option 3" } },
  ];

  const dropdownOptions2 = [
    {
      variant: "avatar",
      value: "1",
      avatarProps: {
        variant: "circle",
        title: identity.metadata.handle,
        avatarFallback: identity.avatarFallback,
        imageUrl: identity.imageUrl,
      },
    },
    {
      variant: "avatar",
      value: "2",
      avatarProps: {
        variant: "square",
        title: orgs[0].metadata.name,
        avatarFallback: orgs[0].avatarFallback,
      },
    },
    {
      variant: "avatar",
      value: "3",
      avatarProps: {
        variant: "square",
        title: orgs[1].metadata.name,
        avatarFallback: orgs[1].avatarFallback,
      },
    },
  ];

  const segmentedControlOptions = [
    {
      title: "Open",
      value: 0,
    },
    {
      title: "Closed",
      value: 1,
    },
    {
      title: "All",
      value: 2,
    },
  ];

  const stats = [
    { icon: Icon.Commit, count: 12 },
    { icon: Icon.Branch, count: 1 },
    { icon: Icon.Member, count: 2 },
    { icon: Icon.Graph, count: 32 },
  ];
</script>

<style>
  table {
    margin-bottom: 32px;
  }

  td {
    vertical-align: middle;
    padding: 5px;
  }

  .layout {
    padding: 32px;
  }
</style>

<ModalLayout full>
  <div class="layout">
    <Title variant="huge" style="margin-bottom: 92px">Primitives</Title>

    <Section title="Colors" subTitle="Primary, secondary and grays">

      {#each colors as color}
        <Text
          style="background-color: var({color}); margin-bottom: 8px;
          border-radius: 2px; padding: 4px 8px;">
          {color}
        </Text>
      {/each}
    </Section>

    <Section
      title="Typography"
      subTitle="Using Inter and Source Code Pro fonts">

      <TypographySwatch title="huge Title">
        <Title variant="huge">Open Source Coin</Title>
      </TypographySwatch>

      <TypographySwatch title="big Title">
        <Title variant="big">Open Source Coin</Title>
      </TypographySwatch>

      <TypographySwatch title="medium Title">
        <Title variant="medium">Open Source Coin</Title>
      </TypographySwatch>

      <TypographySwatch title="regular Title">
        <Title>Open Source Coin</Title>
      </TypographySwatch>

      <TypographySwatch title="regular Text">
        <Text>Open Source Coin</Text>
      </TypographySwatch>

      <TypographySwatch title="small Text">
        <Text variant="small">Open Source Coin</Text>
      </TypographySwatch>

      <TypographySwatch title="tiny Text">
        <Text variant="tiny">Open Source Coin</Text>
      </TypographySwatch>

      <TypographySwatch title="Code">
        <Code>Open Source Coin</Code>
      </TypographySwatch>

      <TypographySwatch title="Caption">
        <Caption>Open Source Coin</Caption>
      </TypographySwatch>

      <TypographySwatch title="big Numeric">
        <Numeric variant="big">0123456789</Numeric>
      </TypographySwatch>

      <TypographySwatch title="regular Numeric">
        <Numeric>0123456789</Numeric>
      </TypographySwatch>

      <TypographySwatch title="small Numeric">
        <Numeric variant="small">0123456789</Numeric>
      </TypographySwatch>

      <TypographySwatch title="tiny Numeric">
        <Numeric variant="tiny">0123456789</Numeric>
      </TypographySwatch>
    </Section>

    <Section
      title="Icons"
      subTitle="Icons at 16px, 24px, 36px and 64px width and height with 2px
      stroke weight, multiple color variations">

      <Icon.Badge />
      <Icon.ArrowDown />
      <Icon.ArrowUp />
      <Icon.Branch />
      <Icon.Carret />
      <Icon.CarretBig />
      <Icon.Check />
      <Icon.CheckCircle />
      <Icon.CloseIssue />
      <Icon.Commit />
      <Icon.Copy />
      <Icon.Cross />
      <Icon.Cross size="big" />
      <Icon.Ellipse />
      <Icon.EllipseBig />
      <Icon.Ellipses />
      <Icon.Feed />
      <Icon.File />
      <Icon.Folder />
      <Icon.Fund />
      <Icon.Graph />
      <Icon.Home />
      <Icon.Important />
      <Icon.Inbox />
      <Icon.Info />
      <Icon.Issue />
      <Icon.Member />
      <Icon.Minus />
      <Icon.Peer />
      <Icon.Plus />
      <Icon.Projects />
      <Icon.Register />
      <Icon.Revisions />
      <Icon.Replies />
      <Icon.Search />
      <Icon.SearchSmall />
      <Icon.Settings />
      <Icon.Source />
    </Section>

    <Section
      title="Complex icons"
      subTitle="Icons at 24px and 32px width and height with animations and
      reactive coloring.">
      <Icon.Spinner />
      <Icon.TransactionState state="positive" />
      <Icon.TransactionState progress={0} variant="small" />
      <Icon.TransactionState progress={0} />
      <Icon.TransactionState progress={10} />
      <Icon.TransactionState progress={100 / 3} />
      <Icon.TransactionState state="negative" progress={0} />
      <Icon.TransactionState state="negative" progress={80} />
      <Icon.TransactionState state="negative" progress={100} />
      <Icon.TransactionState state="negative" />
    </Section>

    <Section
      title="Buttons"
      subTitle="Vanilla, Primary, Secondary, Cancel, disabled state">

      <table>
        <thead>
          <tr>
            <td>
              <Caption>Variant</Caption>
            </td>
            <td>
              <Caption>Disabled</Caption>
            </td>
            <td>
              <Caption>Variant</Caption>
            </td>
            <td>
              <Caption>Disabled</Caption>
            </td>
          </tr>
        </thead>
        <tr>
          <td>
            <Button variant="primary">Primary</Button>
          </td>
          <td>
            <Button variant="primary" disabled>Primary</Button>
          </td>
          <td>
            <Button icon={Icon.Minus} variant="primary">Primary</Button>
          </td>
          <td>
            <Button icon={Icon.Minus} variant="primary" disabled>
              Primary
            </Button>
          </td>
        </tr>
        <tr>
          <td>
            <Button variant="secondary">Secondary</Button>
          </td>
          <td>
            <Button variant="secondary" disabled>Secondary</Button>
          </td>
          <td>
            <Button icon={Icon.SearchSmall} variant="secondary">
              Secondary
            </Button>
          </td>
          <td>
            <Button icon={Icon.SearchSmall} variant="secondary" disabled>
              Secondary
            </Button>
          </td>
        </tr>
        <tr>
          <td>
            <Button variant="vanilla">Vanilla</Button>
          </td>
          <td>
            <Button variant="vanilla" disabled>Vanilla</Button>
          </td>
          <td>
            <Button icon={Icon.Plus} variant="vanilla">Vanilla</Button>
          </td>
          <td>
            <Button icon={Icon.Plus} variant="vanilla" disabled>Vanilla</Button>
          </td>
        </tr>
        <tr>
          <td>
            <Button variant="outline">Outline</Button>
          </td>
          <td>
            <Button variant="outline" disabled>Outline</Button>
          </td>
          <td>
            <Button icon={Icon.Graph} variant="outline">Outline</Button>
          </td>
          <td>
            <Button icon={Icon.Graph} variant="outline" disabled>
              Outline
            </Button>
          </td>
        </tr>
        <tr>
          <td>
            <Button variant="transparent">Transparent</Button>
          </td>
          <td>
            <Button variant="transparent" disabled>Transparent</Button>
          </td>
          <td>
            <Button icon={Icon.Check} variant="transparent">Transparent</Button>
          </td>
          <td>
            <Button icon={Icon.Check} variant="transparent" disabled>
              Transparent
            </Button>
          </td>
        </tr>
        <tr>
          <td>
            <Button variant="destructive">Destructive</Button>
          </td>
          <td>
            <Button variant="destructive" disabled>Destructive</Button>
          </td>
          <td>
            <Button icon={Icon.Cross} variant="destructive">Destructive</Button>
          </td>
          <td>
            <Button icon={Icon.Cross} variant="destructive" disabled>
              Destructive
            </Button>
          </td>
        </tr>

      </table>
    </Section>

    <Section
      title="Form elements"
      subTitle="Inputs, text areas, dropdowns, etc.">

      <Swatch>
        <Input.Text placeholder="Hey, I'm an input." />
      </Swatch>

      <Swatch>
        <Input.Text
          placeholder="Hey, I'm a full-width input."
          style="flex: 1" />
      </Swatch>

      <Swatch>
        <Input.Text
          placeholder="And I'm an input with a validation error."
          style="flex: 1"
          validation={{ status: ValidationStatus.Error, message: "Well, that didn't go well..." }} />
      </Swatch>

      <Swatch>
        <Input.Text
          placeholder="Enter user name"
          style="width: 100%"
          showSuccessCheck
          validation={{ status: ValidationStatus.Success }}
          on:input={() => {
            console.log('event(Input changed)');
          }} />
      </Swatch>

      <Swatch>
        <Input.Text
          placeholder="Enter user name"
          style="width: 100%"
          showSuccessCheck
          validation={{ status: ValidationStatus.Success }}
          value="user123">
          <div slot="avatar">
            <Avatar
              size="small"
              imageUrl="https://avatars1.githubusercontent.com/u/40774" />
          </div>
        </Input.Text>
      </Swatch>

      <Swatch>
        <Input.Text
          placeholder="Enter user name"
          style="width: 100%"
          validation={{ status: ValidationStatus.Loading }}
          value="user123">
          <div slot="avatar">
            <Avatar size="small" avatarFallback={avatarFallback1} />
          </div>
        </Input.Text>
      </Swatch>

      <Swatch>
        <Input.Text
          avatarFallback={avatarFallback2}
          placeholder="Enter user name."
          style="width: 100%"
          valid={false}
          validation={{ status: ValidationStatus.Error, message: 'Handle already taken' }}
          value="myUser">
          <div slot="avatar">
            <Avatar size="small" avatarFallback={avatarFallback2} />
          </div>
        </Input.Text>
      </Swatch>

      <Swatch>
        <Input.Checkbox>How about a checkbox?</Input.Checkbox>
      </Swatch>

      <Swatch>
        <Dropdown placeholder="Select option..." options={dropdownOptions1} />
      </Swatch>

      <Swatch>
        <Dropdown
          placeholder="Select option..."
          options={dropdownOptions1}
          disabled={true} />
      </Swatch>

      <Swatch>
        <Dropdown
          placeholder="Select domain..."
          value="2"
          options={dropdownOptions2} />
      </Swatch>
    </Section>

    <Title variant="huge" style="margin-bottom: 92px">Components</Title>

    <Section
      title="Avatars"
      subTitle="User, project, etc avatars in various sizes and shapes.">

      <Swatch>
        <Avatar
          style="margin-right: 16px"
          size="small"
          variant="circle"
          avatarFallback={avatarFallback1} />
        <Avatar
          style="margin-right: 16px"
          size="small"
          variant="square"
          avatarFallback={avatarFallback2} />
        <Avatar
          style="margin-right: 16px"
          size="small"
          variant="circle"
          imageUrl="https://avatars1.githubusercontent.com/u/40774" />
        <Avatar
          style="margin-right: 16px"
          size="small"
          variant="circle"
          avatarFallback={avatarFallback1}
          title="cloudhead" />
      </Swatch>

      <Swatch>
        <Avatar
          style="margin-right: 16px"
          size="regular"
          variant="circle"
          avatarFallback={avatarFallback1} />
        <Avatar
          style="margin-right: 16px"
          size="regular"
          variant="square"
          avatarFallback={avatarFallback2} />
        <Avatar
          style="margin-right: 16px"
          size="regular"
          variant="circle"
          imageUrl="https://avatars1.githubusercontent.com/u/40774" />
        <Avatar
          style="margin-right: 16px"
          size="regular"
          variant="circle"
          registered={true}
          avatarFallback={avatarFallback1}
          title="cloudhead" />
      </Swatch>

      <Swatch>
        <Avatar
          style="margin-right: 16px"
          size="medium"
          variant="circle"
          avatarFallback={avatarFallback1} />
        <Avatar
          style="margin-right: 16px"
          size="medium"
          variant="square"
          avatarFallback={avatarFallback2} />
        <Avatar
          style="margin-right: 16px"
          size="medium"
          variant="circle"
          imageUrl="https://avatars1.githubusercontent.com/u/40774" />
        <Avatar
          style="margin-right: 16px"
          size="medium"
          variant="circle"
          avatarFallback={avatarFallback1}
          title="cloudhead" />
      </Swatch>

      <Swatch>
        <Avatar
          style="margin-right: 16px"
          size="big"
          variant="circle"
          avatarFallback={avatarFallback1} />
        <Avatar
          style="margin-right: 16px"
          size="big"
          variant="square"
          avatarFallback={avatarFallback2} />
        <Avatar
          style="margin-right: 16px"
          size="big"
          variant="circle"
          imageUrl="https://avatars1.githubusercontent.com/u/40774" />
        <Avatar
          style="margin-right: 16px"
          size="big"
          variant="circle"
          avatarFallback={avatarFallback1}
          title="cloudhead" />
      </Swatch>

      <Swatch>
        <Avatar
          style="margin-right: 16px"
          size="huge"
          variant="circle"
          avatarFallback={avatarFallback1} />
        <Avatar
          style="margin-right: 16px"
          size="huge"
          variant="square"
          avatarFallback={avatarFallback2} />
        <Avatar
          style="margin-right: 16px"
          size="huge"
          variant="circle"
          imageUrl="https://avatars1.githubusercontent.com/u/40774" />
        <Avatar
          style="margin-right: 16px"
          size="huge"
          variant="circle"
          avatarFallback={avatarFallback1}
          title="cloudhead" />
      </Swatch>
    </Section>

    <Section title="Notifications" subTitle="Info, Warnings and Errors">
      <Swatch>
        <Notification
          showIcon="true"
          message="This is harmless, but you should know anyway." />
      </Swatch>

      <Swatch>
        <Notification message="This is harmless without an icon." />
      </Swatch>

      <Swatch>
        <Notification
          level={notification.Level.Error}
          showIcon="true"
          message="Something bad happened, halp!" />
      </Swatch>

      <Swatch>
        <Notification
          level={notification.Level.Error}
          message="Something bad happened with no icon!" />
      </Swatch>
    </Section>

    <Section title="Cards" subTitle="Project, user, etc">
      <Swatch>
        <ProjectCard title="Radicle" />
      </Swatch>

      <Swatch>
        <ProjectCard
          title="Radicle"
          description="Best project in the world"
          showRegisteredBadge={true} />
      </Swatch>

      <Swatch>
        <UserCard {user} />
      </Swatch>
    </Section>

    <Section title="Transaction" subTitle="Row, Accordion and Statusbar">
      <Swatch>
        <Row style="width:100%" disabled={false}>
          <div slot="left">
            <Title>Your Wallet</Title>
          </div>

          <div slot="right">
            <Avatar title="user" avatarFallback={avatarFallback1} />
          </div>
        </Row>
      </Swatch>

      <Swatch>
        <div style="flex-direction: column; width: 100%">
          <Row variant="top">
            <div slot="left">
              <Title>Cost 1</Title>
            </div>

            <div slot="right">
              <Rad amount={4} />
            </div>
          </Row>
          <Row variant="middle">
            <div slot="left">
              <Title>Cost 2</Title>
            </div>

            <div slot="right">
              <Rad amount={4} />
            </div>
          </Row>
          <Row variant="bottom">
            <div slot="left">
              <Title>Total</Title>
            </div>

            <div slot="right">
              <Rad amount={8} size="big" />
            </div>
          </Row>
        </div>
      </Swatch>

      <Swatch>
        <div style="display: flex;">
          <div style="position: relative; height: 200px; width: 280px;">
            <TransactionAccordion
              transactions={transactions1}
              style="position: absolute; bottom: 0; right: 0;" />
          </div>
          <div style="position: relative; height: 200px; width: 280px;">
            <TransactionAccordion
              transactions={transactions2}
              style="position: absolute; bottom: 0; right: 0;" />
          </div>
          <div style="position: relative; height: 200px; width: 280px;">
            <TransactionAccordion
              transactions={transactions3}
              style="position: absolute; bottom: 0; right: 0;" />
          </div>
        </div>
      </Swatch>

      <Swatch>
        <div style="flex-direction: column; width: 100%">
          <TransactionStatusbar style="margin-bottom: 5px;" />
          <TransactionStatusbar progress={30} style="margin-bottom: 5px;" />
          <TransactionStatusbar
            variant="negative"
            style="margin-bottom: 5px;"
            time="1585819617" />
          <TransactionStatusbar variant="positive" time="1585819617" />
        </div>
      </Swatch>
    </Section>

    <Section title="Misc" subTitle="Everything else">

      <Swatch>
        <Placeholder style="width: 300px; height: 100px" />
      </Swatch>

      <Swatch>
        <Rad amount="200" />
      </Swatch>

      <Swatch>
        <StepCounter selectedStep={1} steps={['Step 1', 'Step 2', 'Step 3']} />
      </Swatch>

      <Swatch>
        <AdditionalActionsDropdown
          headerTitle="Copy this title"
          menuItems={additionalActionsDropdownItems} />
      </Swatch>

      <Swatch>
        <TrackToggle peerCount="2.3k" />
      </Swatch>

      <Swatch>
        <SegmentedControl
          active={1}
          options={segmentedControlOptions}
          on:select={() => console.log('event(select)')} />
      </Swatch>

      <Swatch>
        <Stats {stats} />
      </Swatch>
    </Section>
  </div>
</ModalLayout>
