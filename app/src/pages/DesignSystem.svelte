<script>
  import {
    Button,
    Header,
    Text,
    Title,
    Numeric,
    Caption,
    Icon,
    Input
  } from "../DesignSystem";
  import { link } from "svelte-spa-router";
  import Layout from "./Layout.svelte";

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
</style>

<Layout>
  <Header>
    <div slot="left">
      <Title.Big>Design System</Title.Big>
    </div>
  </Header>

  <div>
    {#each colors as color}
      <Text.Regular style="background-color: var({color})">
        {color}
      </Text.Regular>
    {/each}
  </div>

  <div>
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

  <div class="icons">
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
        <Button size="small" icon={Icon.Plus} variant="vanilla">Vanilla</Button>
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
        <Button size="small" variant="transparent" disabled>Transparent</Button>
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

  <div>
    <Input placeholder="Hey, I'm an input." />
  </div>
</Layout>
