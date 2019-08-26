open Atom;

[@react.component]
let make = () => {
  let dispatch = Store.useDispatch();
  let (name, _setName) = React.useState(() => "mvp");
  let (description, _setDescription) =
    React.useState(() => "minimal viable product");
  let (imgUrl, _setImgUrl) = React.useState(() => "");

  <>
    <Title.Big> {React.string("Register a project")} </Title.Big>
    <Text> {React.string("Register a project on the network")} </Text>
    <Button> {React.string("Cancel")} </Button>
    <a
      onClick={
        _ =>
          dispatch(
            StoreMiddleware.Thunk(
              ThunkProjects.registerProject(name, description, imgUrl),
            ),
          )
      }>
      <Button.Secondary> {React.string("Register")} </Button.Secondary>
    </a>
  </>;
};
