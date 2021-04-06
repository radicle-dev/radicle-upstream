<script>
  import { Variant as IllustrationVariant } from "../src/illustration.ts";
  import * as notification from "../src/notification.ts";
  import { CSSPosition } from "../src/style";
  import { ValidationStatus } from "../src/validation.ts";

  import { Avatar, Button, Icon, Input } from "../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    BranchBox,
    CompareBranches,
    Copyable,
    ConnectionStatusIndicator,
    EmptyState,
    FollowToggle,
    Fullscreen,
    Dropdown,
    Illustration,
    Notification,
    ProjectCard,
    SegmentedControl,
    RadicleId,
    Spinner,
    Stats,
    SupportButton,
    StyledCopyable,
    Tooltip,
  } from "../DesignSystem/Component";

  import Section from "./DesignSystemGuide/Section.svelte";
  import Swatch from "./DesignSystemGuide/Swatch.svelte";
  import TypographySwatch from "./DesignSystemGuide/TypographySwatch.svelte";
  import IconSwatch from "./DesignSystemGuide/IconSwatch.svelte";
  import ColorSwatch from "./DesignSystemGuide/ColorSwatch.svelte";

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

  const colorGroups = [
    ...new Set(
      colors.map(color => {
        return color.match(/--color-(\w*)-?/)[1];
      })
    ),
  ];

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

  const dropdownOptions1 = [
    { value: "1", title: "Option 1" },
    {
      value: "2",
      title: "Longer option keeps going",
    },
    { value: "3", title: "Option 3" },
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

<Fullscreen>
  <div class="layout">
    <h1 style="margin-bottom: 92px">Primitives</h1>

    <Section title="Colors" subTitle="Primary, secondary and grays">
      {#each colorGroups as colorGroup}
        <div>
          {#each colors.filter(color => {
            return color.match(colorGroup);
          }) as color}
            <ColorSwatch {color} style="margin: 0 1rem 1rem 0;" />
          {/each}
        </div>
      {/each}
    </Section>

    <Section
      title="Typography"
      subTitle="Using Inter and Source Code Pro fonts">
      <TypographySwatch title="<h1>">
        <h1>Radicle Upstream</h1>
      </TypographySwatch>

      <TypographySwatch title="<h2>">
        <h2>Radicle Upstream</h2>
      </TypographySwatch>

      <TypographySwatch title="<h3>">
        <h3>Radicle Upstream</h3>
      </TypographySwatch>

      <TypographySwatch title={`<h3 class="typo-mono-bold">`}>
        <h3 class="typo-mono-bold">Radicle Upstream</h3>
      </TypographySwatch>

      <TypographySwatch title="<h4>">
        <h4>Radicle Upstream</h4>
      </TypographySwatch>

      <TypographySwatch title={`<p> or <p class="typo-text">`}>
        <p>Radicle Upstream</p>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-text-bold">`}>
        <p class="typo-text-bold">Radicle Upstream</p>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-text-mono">`}>
        <p class="typo-text-mono">Radicle Upstream</p>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-mono-bold">`}>
        <p class="typo-mono-bold">Radicle Upstream</p>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-text-small">`}>
        <p class="typo-text-small">Radicle Upstream</p>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-text-small-bold">`}>
        <p class="typo-text-small-bold">Radicle Upstream</p>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-text-small-bold">`}>
        <p class="typo-text-small-bold">0123456789</p>
      </TypographySwatch>

      <TypographySwatch title={`<a href="/" class="typo-link">`}>
        <a href="/" class="typo-link">Radicle Upstream</a>
      </TypographySwatch>

      <TypographySwatch title={`<p class="typo-all-caps">`}>
        <p class="typo-all-caps">Radicle Upstream</p>
      </TypographySwatch>
    </Section>

    <Section title="Icons" subTitle="Icons at 24px width and height">
      <IconSwatch>
        {#each Object.keys(Icon) as iconName}
          <Tooltip value={`<Icon.${iconName} />`} position="top">
            <Copyable
              notificationText="Icon markup copied to your clipboard"
              iconBeforeCopy={null}
              styleContent={false}
              copyContent={`<Icon.${iconName} />`}>
              <svelte:component this={Icon[iconName]} />
            </Copyable>
          </Tooltip>
        {/each}
      </IconSwatch>
    </Section>

    <Section title="Illustrations" subTitle="Scaleable illustrations">
      <Swatch>
        {#each Object.keys(IllustrationVariant) as key}
          <Tooltip
            value={`<Illustration variant={IllustrationVariant.${key}} />`}
            position="top">
            <Copyable
              notificationText="Illustration markup copied to your clipboard"
              iconBeforeCopy={null}
              styleContent={false}
              copyContent={`<Illustration variant={IllustrationVariant.${key}} />`}>
              <Illustration
                style="margin-right: 2em;"
                variant={IllustrationVariant[key]} />
            </Copyable>
          </Tooltip>
        {/each}
      </Swatch>
    </Section>

    <Section
      title="Buttons"
      subTitle="Vanilla, Primary, Secondary, Cancel, disabled state">
      <table>
        <thead>
          <tr>
            <td>
              <h5>Variant</h5>
            </td>
            <td>
              <h5>Disabled</h5>
            </td>
            <td>
              <h5>Variant</h5>
            </td>
            <td>
              <h5>Disabled</h5>
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
            <Button icon={Icon.MagnifyingGlass} variant="secondary">
              Secondary
            </Button>
          </td>
          <td>
            <Button icon={Icon.MagnifyingGlass} variant="secondary" disabled>
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
            <Button icon={Icon.Fork} variant="outline">Outline</Button>
          </td>
          <td>
            <Button icon={Icon.Fork} variant="outline" disabled>Outline</Button>
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
          placeholder="Hey, I'm a full-width input with a hint"
          hint="I'm a hint"
          style="flex: 1" />
      </Swatch>

      <Swatch>
        <Input.Text
          style="flex: 1;"
          disabled
          placeholder="Hey, I'm a disabled input with a placeholder." />
      </Swatch>

      <Swatch>
        <Input.Text
          style="flex: 1;"
          disabled
          value="I'm a disabled input with a value." />
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
          showLeftItem={true}
          validation={{ status: ValidationStatus.Loading }}
          value="user123">
          <div slot="left">
            <Avatar size="small" avatarFallback={avatarFallback1} />
          </div>
        </Input.Text>
      </Swatch>

      <Swatch>
        <Input.Text
          placeholder="Enter user name."
          style="width: 100%"
          showLeftItem={true}
          validation={{ status: ValidationStatus.Error, message: 'Id already taken' }}
          value="myUser">
          <div slot="left">
            <Avatar size="small" avatarFallback={avatarFallback2} />
          </div>
        </Input.Text>
      </Swatch>

      <Swatch>
        <Input.Password
          style="width: 100%;"
          placeholder="Please enter a password" />
      </Swatch>

      <Swatch>
        <Input.Password style="width: 100%;" value="my super long password" />
      </Swatch>

      <Swatch>
        <Input.Password
          style="width: 100%;"
          value="too short"
          validation={{ status: ValidationStatus.Error, message: 'Password too short.' }} />
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
    </Section>

    <h1 style="margin-bottom: 92px">Components</h1>

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
          notification={notification.create('INFO', { message: 'Snackbar' })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('INFO', {
            message: 'Info with icon',
            showIcon: true,
          })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('INFO', {
            message: 'Info without default action',
            actions: [],
          })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('ERROR', {
            message: 'Just plain error',
          })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('ERROR', {
            message: 'Error with icon',
            showIcon: true,
          })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('ERROR', {
            message: 'Error with one action',
            actions: [{ label: 'Action', handler: () => {} }],
          })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('ERROR', {
            message: 'Error with two actions',
            actions: [
              { label: 'Action 1', handler: () => {} },
              { label: 'Action 2', handler: () => {} },
            ],
          })} />
      </Swatch>

      <Swatch>
        <Notification
          notification={notification.create('PRIMARY', {
            message: 'Primary notification',
            actions: [
              { label: 'Action 1', handler: () => {} },
              { label: 'Action 2', handler: () => {} },
            ],
          })} />
      </Swatch>
    </Section>

    <Section title="Tooltips" subTitle="Top, Right, Bottom, Left">
      <Swatch>
        <Tooltip value="Top" position={CSSPosition.Top}>
          <Button variant="outline">Hover me!</Button>
        </Tooltip>
      </Swatch>

      <Swatch>
        <Tooltip value="Right" position={CSSPosition.Right}>
          <Button variant="outline">Hover me!</Button>
        </Tooltip>
      </Swatch>

      <Swatch>
        <Tooltip value="Bottom" position={CSSPosition.Bottom}>
          <Button variant="outline">Hover me!</Button>
        </Tooltip>
      </Swatch>

      <Swatch>
        <Tooltip value="Left" position={CSSPosition.Left}>
          <Button variant="outline">Hover me!</Button>
        </Tooltip>
      </Swatch>
    </Section>

    <Section title="Cards" subTitle="Project, user, etc">
      <Swatch>
        <ProjectCard title="Radicle" />
      </Swatch>

      <Swatch>
        <ProjectCard title="Radicle" description="Best project in the world" />
      </Swatch>
    </Section>

    <Section title="Misc" subTitle="Everything else">
      <Swatch>
        <AdditionalActionsDropdown
          headerTitle="Copy this title"
          menuItems={additionalActionsDropdownItems} />
      </Swatch>

      <Swatch>
        <FollowToggle
          on:follow={() => {
            console.log('follow');
          }}
          on:unfollow={() => {
            console.log('unfollow');
          }} />
      </Swatch>

      <Swatch>
        <FollowToggle
          following
          on:follow={() => {
            console.log('follow');
          }}
          on:unfollow={() => {
            console.log('unfollow');
          }} />
      </Swatch>

      <Swatch>
        <FollowToggle
          disabled
          on:follow={() => {
            console.log('follow');
          }}
          on:unfollow={() => {
            console.log('unfollow');
          }} />
      </Swatch>

      <Swatch>
        <FollowToggle
          disabled
          following
          on:follow={() => {
            console.log('follow');
          }}
          on:unfollow={() => {
            console.log('unfollow');
          }} />
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
        <BranchBox name="branch-name" />
      </Swatch>
      <Swatch>
        <CompareBranches
          baseBranch="base-branch-name"
          compareBranch="compare-branch-name" />
      </Swatch>

      <Swatch>
        <Stats branches={2} commits={12} contributors={4} />
      </Swatch>

      <Swatch>
        <StyledCopyable
          value="hynewpywqj6x4mxgj7sojhue3erucyexiyhobxx4du9w66hxhbfqbw@seedling.radicle.xyz:12345"
          notificationText="The seed was copied to your clipboard" />
      </Swatch>

      <Swatch>
        <StyledCopyable
          value="hwd1yre8ttugonm77udfkti4ou89p4e37gdebmj3o544hzrg3r8dupn8hmr"
          notificationText="The hash was copied to your clipboard"
          truncate
          expandable={false} />
      </Swatch>

      <Swatch>
        <StyledCopyable
          value="hwd1yre8ttugonm77udfkti4ou89p4e37gdebmj3o544hzrg3r8dupn8hmr"
          notificationText="The hash was copied to your clipboard"
          truncate />
      </Swatch>

      <Swatch>
        <RadicleId
          urn="rad:git:hwd1yre8ttugonm77udfkti4ou89p4e37gdebmj3o544hzrg3r8dupn8hmr" />
      </Swatch>

      <Swatch>
        <Spinner />
      </Swatch>

      <Swatch>
        <EmptyState
          illustration={IllustrationVariant.Plant}
          primaryActionText="Take some action!" />
        <EmptyState emoji="👀" secondaryActionText="Take some other action!" />
        <EmptyState
          emoji="🔭"
          primaryActionText="Take the first action!"
          secondaryActionText="Take the secondary action!" />
        <EmptyState text="Hey, I'm a tent." emoji="🎪" />
      </Swatch>
      <Swatch>
        <ConnectionStatusIndicator />
      </Swatch>
    </Section>
  </div>
</Fullscreen>
