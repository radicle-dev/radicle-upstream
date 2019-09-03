open DesignSystem;
open Particle;

module Styles = {
  open Css;

  let number = style([color(Color.black()), display(inline)]);
};

[@react.component]
let make = (~children, ~style=?) =>
  <p className={Styles.number << Css.style(Font.number) <<? style}>
    children
  </p>;

module Small = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <p className={Styles.number << Css.style(Font.smallNumber) <<? style}>
      children
    </p>;
};

module Big = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <p className={Styles.number << Css.style(Font.bigNumber) <<? style}>
      children
    </p>;
};
