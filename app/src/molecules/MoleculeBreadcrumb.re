open Atom;
open DesignSystem;
open Particle;
open Router;

module Styles = {
  open Css;

  let text = style([color(Color.gray())]);

  let container = style([display(`flex), alignItems(center)]);
};

[@react.component]
let make = (~page) =>
  <El style=Styles.container>
    <Icon.Back />
    <Link page>
      <Text.Small style=Styles.text>
        {React.string("Back to " ++ nameOfPage(page))}
      </Text.Small>
    </Link>
  </El>;
