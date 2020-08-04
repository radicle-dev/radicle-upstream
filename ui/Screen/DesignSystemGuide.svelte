<script>
  import * as notification from "../src/notification.ts";
  import * as transaction from "../src/transaction.ts";

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
    SupportButton,
    TrackToggle,
    TransactionCenter,
    TransactionStatusbar,
    UserCard,
    Urn,
  } from "../DesignSystem/Component";

  import Section from "./DesignSystemGuide/Section.svelte";
  import Swatch from "./DesignSystemGuide/Swatch.svelte";
  import TypographySwatch from "./DesignSystemGuide/TypographySwatch.svelte";
  import IconSwatch from "./DesignSystemGuide/IconSwatch.svelte";
  import { ValidationStatus } from "../src/validation.ts";

  const colors = Array.from(document.styleSheets)
    .filter(
      sheet =>
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
                      ...Array.from(rule.style).filter(name =>
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
      id: "0a1b2c3a",
      messages: [
        {
          type: transaction.MessageType.UserRegistration,
          handle: "xla",
          id: "xla@123abcd.git",
        },
      ],
      state: {
        type: transaction.StateType.Settled,
        minConfirmations: 6,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
    },
    {
      id: "0a1b2c3b",
      messages: [
        {
          type: transaction.MessageType.OrgRegistration,
          orgId: "monadic",
        },
      ],
      state: {
        type: transaction.StateType.Confirmed,
        confirmations: 2,
        minConfirmations: 6,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
    },
    {
      id: "0a1b2c3c",
      messages: [
        {
          type: transaction.MessageType.OrgRegistration,
          orgId: "monadic",
        },
      ],
      state: {
        type: transaction.StateType.Pending,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
    },
    {
      id: "0a1b2c3d",
      messages: [
        {
          type: transaction.MessageType.MemberRegistration,
          orgId: "monadic",
          handle: "xla",
        },
      ],
      state: {
        type: transaction.StateType.Failed,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
    },
  ];

  const transactions2 = [
    {
      id: "0a1b2c3a",
      messages: [
        {
          type: transaction.MessageType.OrgRegistration,
          orgId: "monadic",
        },
      ],
      state: {
        type: transaction.StateType.Pending,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
    },
    {
      id: "0a1b2c3b",
      messages: [
        {
          type: transaction.MessageType.ProjectRegistration,
          domainType: "org",
          domainId: "monadic",
          projectName: "upstream",
          cocId: "upstream@123abcd.git",
        },
      ],
      state: {
        type: transaction.StateType.Pending,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
    },
  ];

  const transactions3 = [
    {
      id: "0a1b2c3a",
      messages: [
        {
          type: transaction.MessageType.OrgRegistration,
          orgId: "monadic",
        },
      ],
      state: {
        type: transaction.StateType.Settled,
        minConfirmations: 6,
        timestamp: {
          nanos: 0,
          secs: 1589806729,
        },
      },
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

    <Section title="Icons" subTitle="Icons at 24px width and height">
      <Swatch>
        <Caption>Main</Caption>
      </Swatch>
      <IconSwatch>
        <Icon.Home />
        <Icon.Source />
        <Icon.Fund />
        <Icon.Member />
        <Icon.Issue />
        <Icon.Settings />
      </IconSwatch>
      <Swatch>
        <Caption>Functional</Caption>
      </Swatch>
      <IconSwatch>
        <Icon.ArrowDown />
        <Icon.ArrowUp />
        <Icon.Check />
        <Icon.Check variant="filled" />
        <Icon.Chevron />
        <Icon.Copy />
        <Icon.Cross />
        <Icon.Cross variant="medium" />
        <Icon.Cross variant="big" />
        <Icon.Cross variant="filled" />
        <Icon.Ellipses />
        <Icon.Expand />
        <Icon.Important variant="no-circle" />
        <Icon.Important />
        <Icon.Important variant="big" />
        <Icon.Info variant="no-circle" />
        <Icon.Info />
        <Icon.Info variant="big" />
        <Icon.Minus />
        <Icon.Plus />
        <Icon.Plus variant="small" />
        <Icon.Search />
        <Icon.Search variant="small" />
      </IconSwatch>
      <Swatch>
        <Caption>Representational</Caption>
      </Swatch>
      <IconSwatch>
        <Icon.Replies />
        <Icon.Register />
        <Icon.Edit />
        <Icon.Inbox />
        <Icon.File />
        <Icon.Feed />
        <Icon.Folder />
        <Icon.At />
        <Icon.Key />
        <Icon.CloseIssue />
        <Icon.Projects />
        <Icon.Eye />
        <Icon.Trash />
        <Icon.Label />
        <Icon.Verified size="large" />
      </IconSwatch>
      <Swatch>
        <Caption>Code</Caption>
      </Swatch>
      <IconSwatch>
        <Icon.Commit />
        <Icon.Revision />
        <Icon.Peer />
        <Icon.Branch />
        <Icon.Graph />
        <Icon.Merge />
      </IconSwatch>
    </Section>

    <Section title="Small Icons" subTitle="Icons at 16px width and height">
      <IconSwatch>
        <Icon.Verified />
      </IconSwatch>

    </Section>

    <Section
      title="Complex icons"
      subTitle="Icons at 24px and 32px width and height with animations and
      reactive coloring.">
      <Icon.Spinner />
      <Icon.TransactionState state="positive" />
      <Icon.TransactionState variant="small" />
      <Icon.TransactionState />
      <Icon.TransactionState progress={(1 / 6) * 100} />
      <Icon.TransactionState progress={(2 / 6) * 100} />
      <Icon.TransactionState state="negative" progress={(2 / 6) * 100} rotate />
      <Icon.TransactionState state="negative" progress={(4 / 6) * 100} />
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
            <Button icon={Icon.Search} variant="secondary">Secondary</Button>
          </td>
          <td>
            <Button icon={Icon.Search} variant="secondary" disabled>
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
          <div slot="left">
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
          <div slot="left">
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
          validation={{ status: ValidationStatus.Error, message: 'Id already taken' }}
          value="myUser">
          <div slot="left">
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
              <Rad rad={4} usd={4} />
            </div>
          </Row>
          <Row variant="middle">
            <div slot="left">
              <Title>Cost 2</Title>
            </div>

            <div slot="right">
              <Rad rad={4} usd={4} />
            </div>
          </Row>
          <Row variant="bottom">
            <div slot="left">
              <Title>Total</Title>
            </div>

            <div slot="right">
              <Rad rad={8} usd={8} size="big" />
            </div>
          </Row>
        </div>
      </Swatch>

      <Swatch>
        <div style="display: flex;">
          <div style="position: relative; height: 200px; width: 280px;">
            <TransactionCenter
              summary={transaction.summarizeTransactions(transactions1)}
              transactions={transactions1}
              style="position: absolute; bottom: 0; right: 0;" />
          </div>
          <div style="position: relative; height: 200px; width: 280px;">
            <TransactionCenter
              summary={transaction.summarizeTransactions(transactions2)}
              transactions={transactions2}
              style="position: absolute; bottom: 0; right: 0;" />
          </div>
          <div style="position: relative; height: 200px; width: 280px;">
            <TransactionCenter
              summary={transaction.summarizeTransactions(transactions3)}
              transactions={transactions3}
              style="position: absolute; bottom: 0; right: 0;" />
          </div>
        </div>
      </Swatch>

      <Swatch>
        <div style="flex-direction: column; width: 100%">
          <TransactionStatusbar
            text={transaction.statusText({
              type: transaction.StateType.Pending,
              timestamp: {
                nanos: 0,
                secs: 1589806729,
              },
            })}
            progress={0}
            variant="caution"
            style="margin-bottom: 5px;" />
          <TransactionStatusbar
            text={transaction.statusText({
              type: transaction.StateType.Confirmed,
              confirmations: 2,
              minConfirmations: 6,
              timestamp: {
                nanos: 0,
                secs: 1589806729,
              },
            })}
            progress={(2 / 6) * 100}
            variant="caution"
            style="margin-bottom: 5px;" />
          <TransactionStatusbar
            text={transaction.statusText({
              type: transaction.StateType.Failed,
              timestamp: {
                nanos: 0,
                secs: 1585819617,
              },
            })}
            variant="negative"
            style="margin-bottom: 5px;" />
          <TransactionStatusbar
            text={transaction.statusText({
              type: transaction.StateType.Settled,
              minConfirmations: 6,
              timestamp: {
                nanos: 0,
                secs: 1585819617,
              },
            })}
            variant="positive" />
        </div>
      </Swatch>
    </Section>

    <Section title="Misc" subTitle="Everything else">

      <Swatch>
        <Placeholder style="width: 300px; height: 100px" />
      </Swatch>

      <Swatch>
        <Rad rad="200" usd="2" />
      </Swatch>

      <Swatch>
        <Rad rad="200" usd="2" variant="debit" />
      </Swatch>

      <Swatch>
        <Rad rad="10" usd="0.1" />
      </Swatch>

      <Swatch>
        <Rad rad="20" usd="2" />
      </Swatch>

      <Swatch>
        <Rad rad="20" usd="2" />
      </Swatch>

      <Swatch>
        <AdditionalActionsDropdown
          headerTitle="Copy this title"
          menuItems={additionalActionsDropdownItems} />
      </Swatch>

      <Swatch>
        <TrackToggle />
      </Swatch>

      <Swatch>
        <SupportButton />
      </Swatch>

      <Swatch>
        <SegmentedControl
          active={1}
          options={segmentedControlOptions}
          on:select={() => console.log('event(select)')} />
      </Swatch>

      <Swatch>
        <Stats branches={2} commits={12} contributors={4} />
      </Swatch>

      <Swatch>
        <Urn urn="5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu" />
      </Swatch>

      <Swatch>
        <Urn urn="%rad:git:copy-me-to-see-the-full-urn" showOnHover />
      </Swatch>
    </Section>
  </div>
</ModalLayout>
