open Atom;
open DesignSystem;
open Molecule;
open Source;

module Styles = {
  open Css;
  open Particle;

  let list = style([children([marginBottom(px(16))])]);

  let membersHeading =
    style([
      marginTop(px(16)),
      marginBottom(px(24)),
      paddingBottom(px(16)),
      borderBottom(px(1), `solid, Color.lightGray()),
    ]);
};

module Members = {
  let renderMember = member =>
    <li key={member.keyName}>
      <PersonCard firstName={member.keyName} imgUrl={member.avatarUrl} />
    </li>;

  [@react.component]
  let make = (~members) =>
    <>
      <El style=Styles.membersHeading>
        <Title> {React.string("Members")} </Title>
      </El>
      <ul className=Styles.list>
        {Array.map(renderMember, members) |> React.array}
      </ul>
    </>;
};

module GetProject = [%graphql
  {|
    query Query($address: String!){
      getProject(address: $address) {
        name
        description
        imgUrl
        members {
          keyName
          avatarUrl
        }
      }
    }
  |}
];

module GetProjectQuery = ReasonApollo.CreateQuery(GetProject);

[@react.component]
let make = (~address: string) => {
  let getProjectQuery = GetProject.make(~address, ());

  <El style=Positioning.gridMediumCentered>
    <El style={margin(0, 0, 24, 0)}> <Breadcrumb page=Router.Projects /> </El>
    <GetProjectQuery variables=getProjectQuery##variables>
      ...{
           ({result}) =>
             switch (result) {
             | Error(e) =>
               <div className="error">
                 {"ERROR: " ++ e##message |> React.string}
               </div>
             | Loading => "Loading..." |> React.string
             | Data(project) =>
               Js.log(project);
               React.null;
             }
         }
    </GetProjectQuery>
  </El>;
};
