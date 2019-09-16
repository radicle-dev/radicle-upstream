open Atom;
open DesignSystem;
open Molecule;
open Particle;
open ReasonApolloHooks.Query;

module Styles = {
  open Css;

  let header = style([marginBottom(px(48))]);

  let listItem =
    style([
      borderBottom(px(1), solid, Color.lightGray()),
      hover([backgroundColor(Color.almostWhite())]),
      lastChild([borderBottomWidth(px(0))]),
    ]);

  let link = style([display(block), padding(px(13))]);
};

module Header = {
  [@react.component]
  let make = () =>
    <div className=Styles.header>
      <El style=Layout.flex>
        <El style=Positioning.flexLeft>
          <Title.Huge> {React.string("Explore")} </Title.Huge>
        </El>
        <El style=Positioning.flexRight>
          <Button onClick={Router.navigateToPage(Router.RegisterProject)}>
            {React.string("Register project")}
          </Button>
        </El>
      </El>
    </div>;
};

module ProjectList = {
  [@react.component]
  let make = (~children) => {
    let projects =
      Array.map(
        project =>
          <li className=Styles.listItem key=project##address>
            <Link style=Styles.link page={Router.Project(project##address)}>
              <ProjectCard
                imgUrl=project##imgUrl
                name=project##name
                description=project##description
              />
            </Link>
          </li>,
        children,
      );

    <ul> {React.array(projects)} </ul>;
  };
};

module GetProjectsConfig = [%graphql
  {|
  query Query{
    projects {
      address
      description
      name
      imgUrl
    }
  }
|}
];

module GetProjectsQuery = ReasonApolloHooks.Query.Make(GetProjectsConfig);

[@react.component]
let make = () => {
  let (simple, _full) = GetProjectsQuery.use();
  let dispatch = Store.useDispatch();

  let content =
    switch (simple) {
    | Error(err) =>
      StoreMiddleware.Thunk(
        ThunkAlerts.showAlert(StoreAlerts.Error, err##message),
      )
      |> dispatch;
      React.null;
    | NoData => React.null
    | Loading => React.string("Loading...")
    | Data(response) =>
      <ul> <ProjectList> {response##projects} </ProjectList> </ul>
    };

  <El style=Positioning.gridMediumCentered> <Header /> content </El>;
};
