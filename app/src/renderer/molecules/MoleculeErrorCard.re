open Atom;
open DesignSystem;

module Styles = {
  open Css;

  let errorCard =
    style([
      display(`inlineFlex),
      flexDirection(column),
      textAlign(center),
      alignItems(center),
    ]);
};

[@react.component]
let make = (~children, ~style=?) =>
  <El style={Styles.errorCard <<? style}>
    <Icon.SadFace style={margin(24, 0, 24, 0)} />
    <El> children </El>
  </El>;
