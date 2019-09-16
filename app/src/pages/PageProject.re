open Atom;
open DesignSystem;
open Molecule;
open ReasonApolloHooks.Query;

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
    <li key=member##keyName>
      <PersonCard firstName=member##keyName imgUrl=member##avatarUrl />
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

module GetProjectConfig = [%graphql
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

module GetProjectQuery = ReasonApolloHooks.Query.Make(GetProjectConfig);

[@react.component]
let make = (~address: string) => {
  let getProject = GetProjectConfig.make(~address, ());
  let (state, _full) =
    GetProjectQuery.use(~variables=getProject##variables, ());

  let content =
    switch (state) {
    | Error(err) =>
      <div className="error">
        {"Error: " ++ err##message |> React.string}
      </div>
    | NoData
    | Loading => <p> {React.string("Loading...")} </p>
    | Data(response) =>
      switch (response##getProject) {
      | None => "Not Found" |> React.string
      | Some(project) =>
        <>
          <ProjectCard.Alternate
            style={margin(0, 0, 50, 0)}
            description=project##description
            name=project##name
            imgUrl=project##imgUrl
          />
          <Members members=project##members />
        </>
      }
    };

  <El style=Positioning.gridMediumCentered>
    <El style={margin(0, 0, 24, 0)}> <Breadcrumb page=Router.Projects /> </El>
    content
  </El>;
};
