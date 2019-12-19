<script>
  import {
    Button,
    Text,
    Title,
    Numeric,
    Caption,
    Icon,
    Input,
    Select,
    CheckboxInput
  } from "../DesignSystem";

  import Notification from "../components/Notification.svelte";

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

  .icons {
    display: flex;
    align-items: center;
  }

  .layout {
    margin: 160px;
  }

  .section {
    margin-bottom: 180px;
    display: grid;
    grid-template-columns: 342px auto 100px;
  }

  .first-column {
    grid-column-start: 1;
    margin-right: 90px;
  }

  .row {
    display: flex;
    margin-bottom: 34px;
    align-items: baseline;
  }
</style>

<div class="layout">
  <div class="section">
    <div class="first-column">
      <Title.Big>Colors</Title.Big>
      <Text.Regular style="color: var(--color-gray)">
        Primary, secondary and grays
      </Text.Regular>
    </div>
    <div class="second-column">
      {#each colors as color}
        <Text.Regular style="background-color: var({color})">
          {color}
        </Text.Regular>
      {/each}
    </div>
  </div>

  <div class="section">
    <div class="first-column">
      <Title.Big>Typography</Title.Big>
      <Text.Regular style="color: var(--color-gray)">
        Using GT America and GT America Mono from Grill Type
      </Text.Regular>
    </div>
    <div class="second-column">
      <Title.Huge>Open Source Coin</Title.Huge>
      <Title.Big>Open Source Coin</Title.Big>
      <Title.Regular>Open Source Coin</Title.Regular>

      <Text.Regular>Open Source Coin</Text.Regular>
      <Text.Small>Open Source Coin</Text.Small>

      <Caption>Open Source Coin</Caption>

      <Numeric.Big>0123456789</Numeric.Big>
      <Numeric.Regular>0123456789</Numeric.Regular>
      <Numeric.Small>0123456789</Numeric.Small>

      <Caption>Open Source Coin</Caption>
    </div>
  </div>

  <div class="section">
    <div class="first-column">
      <Title.Big>Icons</Title.Big>
      <Text.Regular style="color: var(--color-gray)">
        Icons at 16px, 24px, 36px and 64px width and height with 2px stroke
        weight, multiple color variations
      </Text.Regular>
    </div>
    <div class="second-column">
      <Icon.Branch />
      <Icon.Carret />
      <Icon.Check />
      <Icon.Commit />
      <Icon.Cross />
      <Icon.CrossBig />
      <Icon.Ellipse />
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
    </div>
  </div>

  <div class="section">
    <div class="first-column">
      <Title.Big>Buttons</Title.Big>
      <Text.Regular style="color: var(--color-gray)">
        Vanilla, Primary, Secondary, Cancel, disabled state
      </Text.Regular>
    </div>
    <div class="second-column">
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
            <Button icon={Icon.Graph} variant="outline" disabled>
              Outline
            </Button>
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
            <Button
              size="small"
              icon={Icon.Check}
              variant="transparent"
              disabled>
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
    </div>
  </div>

  <div class="section">
    <div class="first-column">
      <Title.Big>Form elements</Title.Big>
      <Text.Regular style="color: var(--color-gray)">
        Inputs, text areas, dropdowns, etc.
      </Text.Regular>
    </div>
    <div class="second-column">
      <div class="row">
        <Input placeholder="Hey, I'm an input." />
      </div>

      <div class="row">
        <Select items={['master', 'dev']} value={'dev'} />
        <Select disabled items={['master', 'dev']} value={'dev'} />
      </div>

      <div class="row">
        <CheckboxInput>How about a checkbox?</CheckboxInput>
      </div>
    </div>
  </div>

  <div class="section">
    <div class="first-column">
      <Title.Big>Components</Title.Big>
      <Text.Regular style="color: var(--color-gray)">
        More complex UI elements
      </Text.Regular>
    </div>
    <div class="second-column">
      <div class="row">
        <Notification>
          This is harmless, but you should know anyway.
        </Notification>
      </div>
    </div>
  </div>
</div>
