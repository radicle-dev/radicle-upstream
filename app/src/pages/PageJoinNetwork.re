open Atom;
open DesignSystem;

module Styles = {
  open Css;

  let content =
    Positioning.gridNarrowCentered
    << style([textAlign(center), gridRowStart(2)]);

  let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);
};

[@react.component]
let make = (~cancelButtonCallback) =>
  <El style=Styles.content>
    <El style={margin(0, 0, 16, 0)}>
      <Title.Big> {React.string("Join the network")} </Title.Big>
    </El>
    <Text>
      {React.string("Create an \"account\" to join the network.")}
    </Text>
    <El style={margin(48, 0, 24, 0)}>
      <Input style={margin(0, 0, 16, 0)} placeholder="Enter your name" />
      <Input placeholder="Enter an avatar URL" />
    </El>
    <El style=Styles.buttonContainer>
      <Button.Cancel onClick=cancelButtonCallback>
        {React.string("Cancel")}
      </Button.Cancel>
      <Button.Secondary>
        {React.string("Join the network")}
      </Button.Secondary>
    </El>
  </El>;
