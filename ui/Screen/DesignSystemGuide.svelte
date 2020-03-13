<script>
  import {
    Avatar,
    Button,
    Code,
    Caption,
    Icon,
    Input,
    Text,
    Title,
    Numeric
  } from "../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    Notification,
    Placeholder,
    ProjectCard,
    Rad,
    Row,
    StepCounter,
    UserCard
  } from "../DesignSystem/Component";

  import Section from "./DesignSystemGuide/Section.svelte";
  import Swatch from "./DesignSystemGuide/Swatch.svelte";
  import TypographySwatch from "./DesignSystemGuide/TypographySwatch.svelte";

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
                      )
                    ]
                  : def),
            []
          )
        ]),
      []
    );

  const user = {
    username: "Rudolfs Osins",
    avatar: "https://avatars.dicebear.com/v2/jdenticon/two.svg"
  };

  const avatarFallback1 = {
    emoji: "📐",
    background: {
      r: 24,
      g: 105,
      b: 216
    }
  };

  const avatarFallback2 = {
    emoji: "🛠",
    background: {
      r: 181,
      g: 25,
      b: 111
    }
  };

  const dropdownMenuItems = [
    {
      title: "Add something",
      icon: Icon.Plus,
      event: () => console.log("event(Add Something)")
    },
    {
      title: "Add something else",
      icon: Icon.Plus,
      event: () => console.log("event(Add Something Else)")
    },
    {
      title: "Send something",
      icon: Icon.ArrowUp,
      event: () => console.log("event(Send Something)")
    }
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
    margin: 160px;
  }
</style>

<div class="layout">
  <Title variant="huge" style="margin-bottom: 92px">Primitives</Title>

  <Section title="Colors" subTitle="Primary, secondary and grays">

    {#each colors as color}
      <Text style="background-color: var({color})">{color}</Text>
    {/each}
  </Section>

  <Section title="Typography" subTitle="Using Inter and Source Code Pro fonts">

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
    <Icon.ArrowUp />
    <Icon.Branch />
    <Icon.Carret />
    <Icon.CarretBig />
    <Icon.Check />
    <Icon.CheckCircle />
    <Icon.Commit />
    <Icon.Copy />
    <Icon.Cross />
    <Icon.CrossBig />
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
    <Icon.Issues />
    <Icon.Member />
    <Icon.Minus />
    <Icon.Peer />
    <Icon.Plus />
    <Icon.Projects />
    <Icon.Revisions />
    <Icon.Search />
    <Icon.SearchSmall />
    <Icon.Source />
    <Icon.Spinner />
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
          <Button variant="primary">Primary</Button>
        </td>
        <td>
          <Button variant="primary" disabled>Primary</Button>
        </td>
        <td>
          <Button icon={Icon.Minus} variant="primary">Primary</Button>
        </td>
        <td>
          <Button icon={Icon.Minus} variant="primary" disabled>Primary</Button>
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
          <Button icon={Icon.SearchSmall} variant="secondary">Secondary</Button>
        </td>
        <td>
          <Button icon={Icon.SearchSmall} variant="secondary" disabled>
            Secondary
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
          <Button variant="outline">Outline</Button>
        </td>
        <td>
          <Button variant="outline" disabled>Outline</Button>
        </td>
        <td>
          <Button icon={Icon.Graph} variant="outline">Outline</Button>
        </td>
        <td>
          <Button icon={Icon.Graph} variant="outline" disabled>Outline</Button>
        </td>
      </tr>
    </table>

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
          <Button size="small" variant="vanilla">Vanilla</Button>
        </td>
        <td>
          <Button size="small" variant="vanilla" disabled>Vanilla</Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Plus} variant="vanilla">
            Vanilla
          </Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Plus} variant="vanilla" disabled>
            Vanilla
          </Button>
        </td>
      </tr>

      <tr>
        <td>
          <Button size="small" variant="primary">Primary</Button>
        </td>
        <td>
          <Button size="small" variant="primary" disabled>Primary</Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Minus} variant="primary">
            Primary
          </Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Minus} variant="primary" disabled>
            Primary
          </Button>
        </td>
      </tr>

      <tr>
        <td>
          <Button size="small" variant="secondary">Secondary</Button>
        </td>
        <td>
          <Button size="small" variant="secondary" disabled>Secondary</Button>
        </td>
        <td>
          <Button size="small" icon={Icon.SearchSmall} variant="secondary">
            Secondary
          </Button>
        </td>
        <td>
          <Button
            size="small"
            icon={Icon.SearchSmall}
            variant="secondary"
            disabled>
            Secondary
          </Button>
        </td>
      </tr>

      <tr>
        <td>
          <Button size="small" variant="transparent">Transparent</Button>
        </td>
        <td>
          <Button size="small" variant="transparent" disabled>
            Transparent
          </Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Check} variant="transparent">
            Transparent
          </Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Check} variant="transparent" disabled>
            Transparent
          </Button>
        </td>
      </tr>

      <tr>
        <td>
          <Button size="small" variant="outline">Outline</Button>
        </td>
        <td>
          <Button size="small" variant="outline" disabled>Outline</Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Graph} variant="outline">
            Outline
          </Button>
        </td>
        <td>
          <Button size="small" icon={Icon.Graph} variant="outline" disabled>
            Outline
          </Button>
        </td>
      </tr>
    </table>
  </Section>

  <Section title="Form elements" subTitle="Inputs, text areas, dropdowns, etc.">

    <Swatch>
      <Input.Text placeholder="Hey, I'm an input." />
    </Swatch>

    <Swatch>
      <Input.Text placeholder="Hey, I'm a full-width input." style="flex: 1" />
    </Swatch>

    <Swatch>
      <Input.Text
        placeholder="And I'm an input with a validation error."
        style="flex: 1"
        valid={false}
        validationMessage="Well, that didn't go well..." />
    </Swatch>

    <Swatch>
      <Input.Text
        placeholder="Enter user name"
        style="width: 100%"
        valid={true}
        variant="handle"
        on:input={() => {
          console.log('event(Input changed)');
        }} />
    </Swatch>

    <Swatch>
      <Input.Text
        avatar={avatarFallback1}
        placeholder="Enter user name"
        style="width: 100%"
        valid={true}
        value="user123"
        variant="handle" />
    </Swatch>

    <Swatch>
      <Input.Text
        avatar={avatarFallback1}
        placeholder="Enter user name"
        style="width: 100%"
        valid={true}
        validationPending="true"
        value="user123"
        variant="handle" />
    </Swatch>

    <Swatch>
      <Input.Text
        avatar={avatarFallback2}
        placeholder="Enter user name."
        style="width: 100%"
        valid={false}
        validationMessage="Handle already taken"
        value="myUser"
        variant="handle" />
    </Swatch>

    <Swatch>
      <Input.Dropdown items={['master', 'dev']} value={'dev'} />
    </Swatch>

    <Swatch>
      <Input.Dropdown disabled items={['master', 'dev']} value={'dev'} />
    </Swatch>

    <Swatch>
      <Input.Checkbox>How about a checkbox?</Input.Checkbox>
    </Swatch>
  </Section>

  <Title variant="huge" style="margin-bottom: 92px">Components</Title>

  <Section
    title="Avatars"
    subTitle="User, project, etc avatars in various sizes and shapes.">

    <Swatch>
      <Avatar avatarFallback={avatarFallback1} />
      <Avatar size="big" avatarFallback={avatarFallback1} />
      <Avatar imageUrl="https://avatars1.githubusercontent.com/u/40774" />
      <Avatar
        imageUrl="https://avatars1.githubusercontent.com/u/40774"
        size="big" />
      <Avatar
        imageUrl="https://avatars.dicebear.com/v2/jdenticon/one.svg"
        variant="project" />
      <Avatar
        imageUrl="https://avatars.dicebear.com/v2/jdenticon/two.svg"
        variant="project"
        size="big" />
    </Swatch>

    <Swatch>
      <Avatar title="My name" avatarFallback={avatarFallback2} />
      <Avatar size="big" title="My name" avatarFallback={avatarFallback2} />
    </Swatch>
  </Section>

  <Section title="Notifications" subTitle="Info, Warnings and Errors">
    <Swatch>
      <Notification>This is harmless, but you should know anyway.</Notification>
    </Swatch>

    <Swatch>
      <Notification level="warning">This is slightly concerning.</Notification>
    </Swatch>

    <Swatch>
      <Notification level="error">Something bad happened, halp!</Notification>
    </Swatch>
  </Section>

  <Section title="Cards" subTitle="Project, user, etc">

    <Swatch>
      <ProjectCard
        title="Radicle"
        description="Best project in the world"
        isRegistered={true} />
    </Swatch>

    <Swatch>
      <UserCard {user} />
    </Swatch>
  </Section>

  <Section title="Transaction Row" subTitle="Single row and combined">
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
        menuItems={dropdownMenuItems} />
    </Swatch>
  </Section>
</div>
