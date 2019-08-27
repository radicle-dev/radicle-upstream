open Atom;
open Layout;
open DesignSystem;

module Styles = {
  open Css;

  let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);
};

[@react.component]
let make = (~cancelButtonCallback) =>
  <>
    <Container style={margin(0, 0, 16, 0)}>
      <Title.Big> {React.string("Join the network")} </Title.Big>
    </Container>
    <Text>
      {React.string("Create an \"account\" to join the network.")}
    </Text>
    <Container style={margin(48, 0, 24, 0)}>
      <Container style={margin(0, 0, 16, 0)}>
        <Input placeholder="Enter your name" />
      </Container>
      <Input placeholder="Enter an avatar URL" />
    </Container>
    <Container style=Styles.buttonContainer>
      <Container>
        <Button.Cancel onClick=cancelButtonCallback>
          {React.string("Cancel")}
        </Button.Cancel>
      </Container>
      <Button.Secondary>
        {React.string("Join the network")}
      </Button.Secondary>
    </Container>
  </>;
