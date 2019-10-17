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

module ProjectItem = {
  [@react.component]
  let make = (~id, ~name, ~description, ~imgUrl) =>
    <li className=Styles.listItem>
      <Link style=Styles.link page={Router.Project(id)}>
        <ProjectCard imgUrl name description />
      </Link>
    </li>;
};

module ProjectList = {
  [@react.component]
  let make = (~children) => {
    let projects =
      Array.map(
        project => {
          let id =
            switch (project##id |> Js.Json.decodeString) {
            | Some(id) => id
            | None => ""
            };

          <ProjectItem
            key=id
            id
            name=project##name
            description=project##description
            imgUrl=project##imgUrl
          />;
        },
        children,
      );

    <ul> {React.array(projects)} </ul>;
  };
};

module ErrorMessage = {
  [@react.component]
  let make = () =>
    <El style=Positioning.flexCentered>
      <ErrorCard>
        <Text> {React.string("Coundn't load projects.")} </Text>
        <br />
        <Text> {React.string("Backend is not reachable.")} </Text>
      </ErrorCard>
    </El>;
};

module GetProjectsConfig = [%graphql
  {|

  query Query{
    projects {
      id
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

  let content =
    switch (simple) {
    | Error(_error) => <ErrorMessage />
    | NoData => React.null
    | Loading => React.string("Loading...")
    | Data(response) =>
      <ul> <ProjectList> {response##projects} </ProjectList> </ul>
    };

  <El style=Positioning.gridMediumCentered> <Header /> content </El>;
};
