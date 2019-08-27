open Atom;
open Molecule;
open Layout;
open DesignSystem;

module Form = {
  module Styles = {
    open Css;

    let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);
  };

  [@react.component]
  let make = (~closeButtonCallback) =>
    <Modal closeButtonCallback>
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
          <Button.Cancel> {React.string("Cancel")} </Button.Cancel>
        </Container>
        <Button.Secondary>
          {React.string("Join the network")}
        </Button.Secondary>
      </Container>
    </Modal>;
};

[@react.component]
let make = () => {
  let (show, toggleShow) = React.useState(() => false);

  <>
    <Button.Primary onClick={_ => toggleShow(_ => true)}>
      {React.string("Join the network")}
    </Button.Primary>
    {
      show ?
        <Form closeButtonCallback={_ => toggleShow(_ => false)} /> : React.null
    }
  </>;
};
