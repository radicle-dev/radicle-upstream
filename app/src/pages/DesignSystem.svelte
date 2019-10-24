<script>
  import { Text, Title, Numeric, Caption } from "../DesignSystem";
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

<a href="/" use:link>back</a>

{#each colors as color}
  <div style="background-color: var({color})">{color}</div>
{/each}

<Title.Huge>Open Source Coin</Title.Huge>
<Title.Big>Open Source Coin</Title.Big>
<Title.Regular>Open Source Coin</Title.Regular>

<Text.Regular>Open Source Coin</Text.Regular>
<Text.Small>Open Source Coin</Text.Small>

<Caption>Open Source Coin</Caption>

<Numeric.Big>0123456789</Numeric.Big>
<Numeric.Regular>0123456789</Numeric.Regular>
<Numeric.Small>0123456789</Numeric.Small>
