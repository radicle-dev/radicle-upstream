open DesignSystem.Operators;

module Styles = {
  open Css;
  open Particle;

  let input =
    style([
      border(px(1), solid, Color.lightGray()),
      display(block),
      padding(px(8)),
      borderRadius(px(4)),
      width(pct(100.0)),
      height(px(48)),
    ]);
};

[@react.component]
let make = (~placeholder="", ~onChange=_ => (), ~style=?) =>
  <input className={Styles.input <<? style} onChange placeholder />;
