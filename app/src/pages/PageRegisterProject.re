open Atom;

[@react.component]
let make = () => {
  let dispatch = Store.useDispatch();

  <>
    <Title.Big> {React.string("Register a project")} </Title.Big>
    <Text> {React.string("Register a project on the network")} </Text>
    <Button> {React.string("Cancel")} </Button>
    <a
      onClick={
        _ => dispatch(StoreMiddleware.Thunk(ThunkProjects.registerProject))
      }>
      <Button.Secondary> {React.string("Register")} </Button.Secondary>
    </a>
  </>;
};
