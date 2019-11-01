<script>
  import {
    Button,
    ButtonIcon,
    ButtonSmall,
    ButtonIconSmall,
    Header,
    Text,
    Title,
    Numeric,
    Caption,
    Icon,
    Input
  } from "../DesignSystem";
  import { link } from "svelte-spa-router";
  import Layout from "../components/Layout.svelte";

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
    <Icon.Check />
    <Icon.Commit />
    <Icon.Cross />
    <Icon.CrossBig />
    <Icon.Ellipses />
    <Icon.Feed />
    <Icon.File />
    <Icon.Folder />
    <Icon.Fund />
    <Icon.Graph />
    <Icon.Important />
    <Icon.Inbox />
    <Icon.Info />
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
        <ButtonIcon icon={Icon.Plus} variant="vanilla">Vanilla</ButtonIcon>
      </td>
      <td>
        <ButtonIcon icon={Icon.Plus} variant="vanilla" disabled>
          Vanilla
        </ButtonIcon>
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
        <ButtonIcon icon={Icon.Minus} variant="primary">Primary</ButtonIcon>
      </td>
      <td>
        <ButtonIcon icon={Icon.Minus} variant="primary" disabled>
          Primary
        </ButtonIcon>
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
        <ButtonIcon icon={Icon.SearchSmall} variant="secondary">
          Secondary
        </ButtonIcon>
      </td>
      <td>
        <ButtonIcon icon={Icon.SearchSmall} variant="secondary" disabled>
          Secondary
        </ButtonIcon>
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
        <ButtonIcon icon={Icon.Check} variant="transparent">
          Transparent
        </ButtonIcon>
      </td>
      <td>
        <ButtonIcon icon={Icon.Check} variant="transparent" disabled>
          Transparent
        </ButtonIcon>
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
        <ButtonIcon icon={Icon.Graph} variant="outline">Outline</ButtonIcon>
      </td>
      <td>
        <ButtonIcon icon={Icon.Graph} variant="outline" disabled>
          Outline
        </ButtonIcon>
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
        <ButtonSmall variant="vanilla">Vanilla</ButtonSmall>
      </td>
      <td>
        <ButtonSmall variant="vanilla" disabled>Vanilla</ButtonSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Plus} variant="vanilla">
          Vanilla
        </ButtonIconSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Plus} variant="vanilla" disabled>
          Vanilla
        </ButtonIconSmall>
      </td>
    </tr>

    <tr>
      <td>
        <ButtonSmall variant="primary">Primary</ButtonSmall>
      </td>
      <td>
        <ButtonSmall variant="primary" disabled>Primary</ButtonSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Minus} variant="primary">
          Primary
        </ButtonIconSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Minus} variant="primary" disabled>
          Primary
        </ButtonIconSmall>
      </td>
    </tr>

    <tr>
      <td>
        <ButtonSmall variant="secondary">Secondary</ButtonSmall>
      </td>
      <td>
        <ButtonSmall variant="secondary" disabled>Secondary</ButtonSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.SearchSmall} variant="secondary">
          Secondary
        </ButtonIconSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.SearchSmall} variant="secondary" disabled>
          Secondary
        </ButtonIconSmall>
      </td>
    </tr>

    <tr>
      <td>
        <ButtonSmall variant="transparent">Transparent</ButtonSmall>
      </td>
      <td>
        <ButtonSmall variant="transparent" disabled>Transparent</ButtonSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Check} variant="transparent">
          Transparent
        </ButtonIconSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Check} variant="transparent" disabled>
          Transparent
        </ButtonIconSmall>
      </td>
    </tr>

    <tr>
      <td>
        <ButtonSmall variant="outline">Outline</ButtonSmall>
      </td>
      <td>
        <ButtonSmall variant="outline" disabled>Outline</ButtonSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Graph} variant="outline">
          Outline
        </ButtonIconSmall>
      </td>
      <td>
        <ButtonIconSmall icon={Icon.Graph} variant="outline" disabled>
          Outline
        </ButtonIconSmall>
      </td>
    </tr>
  </table>

  <div>
    <Input placeholder="Hey, I'm an input." />
  </div>
</Layout>
