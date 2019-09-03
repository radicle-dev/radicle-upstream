open DesignSystem;
open Particle;

module Styles = {
  open Css;

  let text = style([color(Color.black()), display(inline)]);
};

[@react.component]
let make = (~children, ~style=?) =>
  <p className={Styles.text << Css.style(Particle.Font.text) <<? style}>
    children
  </p>;

module Small = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <p
      className={Styles.text << Css.style(Particle.Font.smallText) <<? style}>
      children
    </p>;
};

module Caption = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <p className={Styles.text << Css.style(Particle.Font.caption) <<? style}>
      children
    </p>;
};
