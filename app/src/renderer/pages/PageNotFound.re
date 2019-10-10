open Atom;
open DesignSystem;
open Molecule;

[@react.component]
let make = () =>
  <El style=Positioning.gridMediumCentered>
    <El style=Positioning.flexCentered>
      <ErrorCard>
        <Text> {React.string("This page doesn't exist.")} </Text>
      </ErrorCard>
    </El>
  </El>;
