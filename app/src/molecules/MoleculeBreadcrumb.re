open Atom;
open Layout;
open Particle;
open Router;

module Styles = {
  open Css;

  let text = style([color(Color.gray())]);

  let container = style([display(`flex), alignItems(center)]);
};

[@react.component]
let make = (~page) =>
  <Container style=Styles.container>
    <Icon.Back />
    <Link page>
      <Text.Small style=Styles.text>
        {React.string("Back to " ++ nameOfPage(page))}
      </Text.Small>
    </Link>
  </Container>;
