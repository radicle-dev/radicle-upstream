<script>
  import { Button, Text, Icon, Input } from "../DesignSystem/Primitives";
  import {
    Avatar,
    Notification,
    GuideSection,
    Swatch
  } from "../DesignSystem/Components";

  import { link } from "svelte-spa-router";

  let colors = Array.from(document.styleSheets)
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
</script>

<style>
  div,
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
  <Text variant="hugeTitle" style="margin-bottom: 92px">Primitives</Text>

  <GuideSection title="Colors" subTitle="Primary, secondary and grays">

    {#each colors as color}
      <Text style="background-color: var({color})">{color}</Text>
    {/each}
  </GuideSection>

  <GuideSection
    title="Typography"
    subTitle="Using GT America and GT America Mono from Grill Type">

    <Text variant="hugeTitle">Open Source Coin</Text>
    <Text variant="bigTitle">Open Source Coin</Text>
    <Text variant="title">Open Source Coin</Text>

    <Text>Open Source Coin</Text>
    <Text variant="smallText">Open Source Coin</Text>
    <Text variant="smallTextBold">Open Source Coin</Text>

    <Text variant="code">Open Source Coin</Text>

    <Text variant="caption">Open Source Coin</Text>

    <Text variant="bigNumber">0123456789</Text>
    <Text variant="number">0123456789</Text>
    <Text variant="smallNumber">0123456789</Text>
  </GuideSection>

  <GuideSection
    title="Icons"
    subTitle="Icons at 16px, 24px, 36px and 64px width and height with 2px
    stroke weight, multiple color variations">

    <Icon.Branch />
    <Icon.Carret />
    <Icon.Check />
    <Icon.Commit />
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
    <Icon.Member />
    <Icon.Minus />
    <Icon.Plus />
    <Icon.Projects />
    <Icon.Search />
    <Icon.SearchSmall />
    <Icon.Source />
  </GuideSection>

  <GuideSection
    title="Buttons"
    subTitle="Vanilla, Primary, Secondary, Cancel, disabled state">

    <table>
      <thead>
        <tr>
          <td>
            <Text variant="caption">Variant</Text>
          </td>
          <td>
            <Text variant="caption">Disabled</Text>
          </td>
          <td>
            <Text variant="caption">Variant</Text>
          </td>
          <td>
            <Text variant="caption">Disabled</Text>
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
            <Text variant="caption">Variant</Text>
          </td>
          <td>
            <Text variant="caption">Disabled</Text>
          </td>
          <td>
            <Text variant="caption">Variant</Text>
          </td>
          <td>
            <Text variant="caption">Disabled</Text>
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
  </GuideSection>

  <GuideSection
    title="Form elements"
    subTitle="Inputs, text areas, dropdowns, etc.">

    <Swatch>
      <Input.Text placeholder="Hey, I'm an input." />
    </Swatch>

    <Swatch>
      <Input.Dropdown items={['master', 'dev']} value={'dev'} />
      <Input.Dropdown disabled items={['master', 'dev']} value={'dev'} />
    </Swatch>

    <Swatch>
      <Input.Checkbox>How about a checkbox?</Input.Checkbox>
    </Swatch>
  </GuideSection>

  <Text variant="hugeTitle" style="margin-bottom: 92px">Components</Text>

  <GuideSection title="Notifications" subTitle="Info, Warnings and Errors">

    <Swatch>
      <Notification>This is harmless, but you should know anyway.</Notification>
    </Swatch>

    <Swatch>
      <Notification level="warning">This is slightly concerning.</Notification>
    </Swatch>

    <Swatch>
      <Notification level="error">Something bad happened, halp!</Notification>
    </Swatch>
  </GuideSection>

  <GuideSection
    title="Avatars"
    subTitle="User, project, etc avatars in various sizes and shapes.">

    <Swatch>
      <Avatar />
      <Avatar size="big" />
      <Avatar
        imageUrl="https://avatars.dicebear.com/v2/avataaars/gWIIBkeCYYZFCPhBp3Cl.svg" />
      <Avatar
        imageUrl="https://avatars.dicebear.com/v2/avataaars/gWIIBkeCYYZFCPhBp3Cl.svg"
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
      <Avatar title="My name" />
      <Avatar size="big" title="My name" />
    </Swatch>
  </GuideSection>
</div>
