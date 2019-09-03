open Atom;
open DesignSystem;
open Router;

module Styles = {
  open Css;

  let content = style([textAlign(center)]) << Positioning.gridNarrowCentered;

  let buttonContainer = style([display(`flex), justifyContent(flexEnd)]);
};

[@react.component]
let make = () => {
  let dispatch = Store.useDispatch();
  let (name, setName) = React.useState(() => "mvp");
  let (description, setDescription) =
    React.useState(() => "minimal viable product");
  let (imgUrl, setImgUrl) = React.useState(() => "");

  let onNameChange = ev => {
    let newName = ev->ReactEvent.Form.target##value;
    setName(_ => newName);
  };
  let onDescriptionChange = ev => {
    let newDescription = ev->ReactEvent.Form.target##value;
    setDescription(_ => newDescription);
  };
  let onImgChange = ev => {
    let newImgUrl = ev->ReactEvent.Form.target##value;
    setImgUrl(_ => newImgUrl);
  };
  let registerCallback = _ =>
    dispatch(
      StoreMiddleware.Thunk(
        ThunkProjects.registerProject(name, description, imgUrl),
      ),
    );

  <El style=Styles.content>
    <Title.Big style={margin(0, 0, 16, 0)}>
      {React.string("Register a project")}
    </Title.Big>
    <Text> {React.string("Register a project on the network")} </Text>
    <El style={margin(48, 0, 24, 0)}>
      <Input
        onChange=onNameChange
        placeholder="Enter the project name"
        style={margin(0, 0, 16, 0)}
      />
      <Input
        onChange=onDescriptionChange
        placeholder="Enter your project description"
        style={margin(0, 0, 16, 0)}
      />
      <Input onChange=onImgChange placeholder="Add a project picture" />
    </El>
    <El style=Styles.buttonContainer>
      <Button.Cancel onClick={navigateToPage(Projects)}>
        {React.string("Cancel")}
      </Button.Cancel>
      <Button.Secondary onClick=registerCallback>
        {React.string("Register")}
      </Button.Secondary>
    </El>
  </El>;
};
