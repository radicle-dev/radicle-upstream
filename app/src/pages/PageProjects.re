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

module ErrorMessage = {
  module Styles = {
    open Css;

    let errorCard =
      style([
        display(`flex),
        flexDirection(column),
        textAlign(center),
        alignItems(center),
      ]);
  };

  [@react.component]
  let make = (~children) =>
    <El style=Styles.errorCard>
      <Icon.SadFace style={margin(24, 0, 24, 0)} />
      <El> children </El>
    </El>;
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

  let content =
    switch (simple) {
    | Error(_error) =>
      <ErrorMessage>
        <Text> {React.string("Coundn't load projects.")} </Text>
        <br />
        <Text>
          {React.string("Please make sure the proxy is running.")}
        </Text>
      </ErrorMessage>
    | NoData => React.null
    | Loading => React.string("Loading...")
    | Data(response) =>
      <ul> <ProjectList> {response##projects} </ProjectList> </ul>
    };

  <El style=Positioning.gridMediumCentered> <Header /> content </El>;
};
