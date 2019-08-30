open DesignSystem;
open Molecule.Modal;
open Particle;

module Styles = {
  open Css;

  let content = Positioning.gridFullCentered;

  let wrapper =
    Layout.grid
    << style([
         position(fixed),
         width(pct(100.0)),
         height(pct(100.0)),
         backgroundColor(Color.white()),
         justifyContent(center),
       ]);

  let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);
};

[@react.component]
let make = () =>
  <Portal>
    <El style=Styles.wrapper>
      <El style=Styles.content> <h1> {React.string("Styleguide")} </h1> </El>
    </El>
  </Portal>;
