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

  <El style=Styles.content>
    <El style={margin(0, 0, 16, 0)}>
      <Title.Big> {React.string("Join the network")} </Title.Big>
    </El>
    <Text>
      {React.string("Create an \"account\" to join the network.")}
    </Text>
    <El style={margin(48, 0, 24, 0)}>
      <Input
        onChange=onNameChange
        placeholder="Enter your name"
        style={margin(0, 0, 16, 0)}
      />
      <Input onChange=onAvatarChange placeholder="Enter an avatar URL" />
    </El>
    <El style=Styles.buttonContainer>
      <Button.Cancel> {React.string("Cancel")} </Button.Cancel>
      <Button.Secondary onClick={_ => onSubmit(name, avatarUrl)}>
        {React.string("Join the network")}
      </Button.Secondary>
    </El>
  </El>;
};
