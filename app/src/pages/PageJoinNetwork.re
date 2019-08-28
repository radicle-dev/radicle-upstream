open Atom;
open Layout;
open DesignSystem;

module Styles = {
  open Css;

  let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);
};

[@react.component]
let make = (~onComplete) => {
  let dispatch = Store.useDispatch();
  let (name, setName) = React.useState(() => "");
  let (avatarUrl, setAvatarUrl) = React.useState(() => "");

  let onNameChange = ev => {
    /* We need to memoize it as the underlying event is reused for performance
     * reasons. Alternatively we can use the persist mechanism, which might lead
     * to unnecessary garbage.
     */
    let newName = ReactEvent.Form.target(ev)##value;
    setName(_ => newName);
  };
  let onAvatarChange = ev => {
    let newAvatarUrl = ReactEvent.Form.target(ev)##value;
    setAvatarUrl(_ => newAvatarUrl);
  };
  let onSubmit = (name, avatarUrl) => {
    StoreMiddleware.Thunk(ThunkSession.createAccount(name, avatarUrl))
    |> dispatch;
    onComplete();
  };

  <>
    <Container style={margin(0, 0, 16, 0)}>
      <Title.Big> {React.string("Join the network")} </Title.Big>
    </Container>
    <Text>
      {React.string("Create an \"account\" to join the network.")}
    </Text>
    <Container style={margin(48, 0, 24, 0)}>
      <Container style={margin(0, 0, 16, 0)}>
        <Input onChange=onNameChange placeholder="Enter your name" />
      </Container>
      <Input onChange=onAvatarChange placeholder="Enter an avatar URL" />
    </Container>
    <Container style=Styles.buttonContainer>
      <Container>
        <Button.Cancel> {React.string("Cancel")} </Button.Cancel>
      </Container>
      <Button.Secondary onClick={_ => onSubmit(name, avatarUrl)}>
        {React.string("Join the network")}
      </Button.Secondary>
    </Container>
  </>;
};
