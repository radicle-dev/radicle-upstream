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
      hover([backgroundColor(Color.almostWhite())]),
      lastChild([borderBottomWidth(px(0))]),
    ]);

  let link = style([display(block), padding(px(13))]);
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

  <El style=Positioning.gridMediumCentered>
    <div className=Styles.projectHeading>
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
    </div>
    {
      switch (simple) {
      | Error(err) =>
        <div className="error">
          {"ERROR: " ++ err##message |> React.string}
        </div>
      | NoData => React.null
      | Loading => "Loading..." |> React.string
      | Data(response) =>
        <ul>
          {
            response##projects
            |> Array.mapi((index, project) =>
                 <li className=Styles.listItem key={index |> string_of_int}>
                   <Link
                     style=Styles.link
                     page={Router.Project(project##address)}>
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
