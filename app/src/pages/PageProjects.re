let ste = React.string;

open Atom;
open DesignSystem;
open Molecule;
open Particle;

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

module GetAllProjects = [%graphql
  {|
    {
      allProjects {
        address
        description
        name
        imgUrl
      }
    }
|}
];

module GetAllProjectsQuery = ReasonApollo.CreateQuery(GetAllProjects);

[@react.component]
let make = () =>
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
    <GetAllProjectsQuery>
      {
        ({result}) =>
          switch (result) {
          | Error(e) =>
            <div className="error"> {"ERROR: " ++ e##message |> ste} </div>
          | Loading => "Loading..." |> ste
          | Data(response) =>
            <ul>
              {
                Js.log(response);
                response##allProjects
                |> Array.mapi((index, project) =>
                     <li
                       className=Styles.listItem key={index |> string_of_int}>
                       <Link page={Router.Project(project##address)}>
                         <ProjectCard
                           imgUrl=project##imgUrl
                           name=project##name
                           description=project##description
                         />
                       </Link>
                     </li>
                   )
                |> React.array;
              }
            </ul>
          }
      }
    </GetAllProjectsQuery>
  </El>;
