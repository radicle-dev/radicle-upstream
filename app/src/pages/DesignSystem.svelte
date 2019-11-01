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
          <Caption>Normal</Caption>
        </td>
        <td>
          <Caption>Disabled</Caption>
        </td>
      </tr>
    </thead>
    <tr>
      <td>
        <Button.Vanilla>Vanilla button</Button.Vanilla>
      </td>
      <td>
        <Button.Vanilla disabled>Vanilla button</Button.Vanilla>
      </td>
    </tr>

    <tr>
      <td>
        <Button.Primary>Primary button</Button.Primary>
      </td>
      <td>
        <Button.Primary disabled>Primary button</Button.Primary>
      </td>
    </tr>

    <tr>
      <td>
        <Button.Secondary>Secondary button</Button.Secondary>
      </td>
      <td>
        <Button.Secondary disabled>Secondary button</Button.Secondary>
      </td>
    </tr>

    <tr>
      <td>
        <Button.Transparent>Transparent button</Button.Transparent>
      </td>
      <td>
        <Button.Transparent disabled>Transparent button</Button.Transparent>
      </td>
    </tr>

    <tr>
      <td>
        <Button.Outline>Outline button</Button.Outline>
      </td>
      <td>
        <Button.Outline disabled>Outline button</Button.Outline>
      </td>
    </tr>
  </table>

  <div>
    <Input placeholder="Hey, I'm an input." />
  </div>
</Layout>
