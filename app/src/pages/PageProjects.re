open AppStore;
open Atom;
open DesignSystem;
open Molecule;
open Source;
open Particle;
open ReasonApolloHooks.Query;

module Styles = {
  open Css;

  let projectHeading = style([marginBottom(px(48))]);

  let listItem =
    style([
      borderBottom(px(1), solid, Color.lightGray()),
      padding(px(13)),
      hover([backgroundColor(Color.almostWhite())]),
      lastChild([borderBottomWidth(px(0))]),
    ]);
};

module List = {
  [@react.component]
  let make = (~projects: array(project)) => {
    let ps =
      Array.map(
        project =>
          <li className=Styles.listItem key={project.address}>
            <Link page={Router.Project(project.address)}>
              <ProjectCard
                imgUrl={project.imgUrl}
                name={project.name}
                description={project.description}
              />
            </Link>
          </li>,
        projects,
      );

    <ul> {React.array(ps)} </ul>;
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

  <El style=Positioning.gridMediumCentered>
    <div className=Styles.projectHeading>
      <El style=Layout.flex>
        <El style=Positioning.flexLeft>
          <Title.Huge> {React.string("Explore")} </Title.Huge>
        </El>
        <El style=Positioning.flexRight>
          <Link page=Router.RegisterProject>
            <Button> {React.string("Register project")} </Button>
          </Link>
        </El>
      </El>
    </div>
    {
      switch (simple) {
      | Error(err) =>
        dispatch(
          AlertsAction(
            StoreAlerts.Show(
              StoreAlerts.{severity: Alert.Error, message: err##message},
            ),
          ),
        );
        React.null;
      | NoData => React.null
      | Loading => "Loading..." |> React.string
      | Data(response) =>
        <ul>
          {
            response##projects
            |> Array.mapi((index, project) =>
                 <li className=Styles.listItem key={index |> string_of_int}>
                   <Link page={Router.Project(project##address)}>
                     <ProjectCard
                       imgUrl=project##imgUrl
                       name=project##name
                       description=project##description
                     />
                   </Link>
                 </li>
               )
            |> React.array
          }
        </ul>
      }
    }
  </El>;
};
