open Atom;
open Molecule;

module Form = {
  module Styles = {
    open Css;

    let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);

    let inputContainer = style([marginTop(px(48)), marginBottom(px(24))]);

    let nameInput = style([marginBottom(px(16))]);
  };

  [@react.component]
  let make = (~closeButtonCallback) =>
    <Modal closeButtonCallback>
      <Title.Big> {React.string("Join the network")} </Title.Big>
      <Text>
        {React.string("Create an \"account\" to join the network.")}
      </Text>
      <div className=Styles.inputContainer>
        <Input placeholder="Enter your name" />
        <Input placeholder="Enter an avatar URL" />
      </div>
      <div className=Styles.buttonContainer>
        <Button> {React.string("Cancel")} </Button>
        <Button.Secondary>
          {React.string("Join the network")}
        </Button.Secondary>
      </div>
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
