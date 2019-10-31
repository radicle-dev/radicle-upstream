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
  div {
    margin-bottom: 32px;
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

  <div>
    <Icon.Search />
    <Icon.Feed />
    <Icon.Projects />
    <Icon.Plus />
    <Icon.Fund />
  </div>

  <div>
    <Button.Vanilla>Vanilla button</Button.Vanilla>
    <Button.Primary>Primary button</Button.Primary>
    <Button.Secondary>Secondary button</Button.Secondary>
    <Button.Transparent>Transparent button</Button.Transparent>
    <Button.Outline>Outline button</Button.Outline>
  </div>

  <div>
    <Button.Vanilla disabled>Vanilla button disabled</Button.Vanilla>
    <Button.Primary disabled>Primary button</Button.Primary>
    <Button.Secondary disabled>Secondary button</Button.Secondary>
    <Button.Transparent disabled>Transparent button</Button.Transparent>
    <Button.Outline disabled>Outline button</Button.Outline>
  </div>

  <div>
    <Input placeholder="Hey, I'm an input." />
  </div>
</Layout>
