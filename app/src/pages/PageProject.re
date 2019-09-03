open AppStore;
open Atom;
open DesignSystem;
open Molecule;
open Source;
open StoreProject;

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

[@react.component]
let make = (~address: string) => {
  let state = Store.useSelector(state => state.projectState);
  let dispatch = Store.useDispatch();

  let content =
    switch (state) {
    | Initial
    | Loading =>
      dispatch(StoreMiddleware.Thunk(ThunkProject.fetchProject(address)));
      <p> {React.string("Loading...")} </p>;
    | Present(project) =>
      if (project.address != address) {
        dispatch(StoreMiddleware.Thunk(ThunkProject.fetchProject(address)));
        <p> {React.string("Loading...")} </p>;
      } else {
        <>
          <El style={margin(0, 0, 50, 0)}>
            <ProjectCard.Alternate
              description={project.description}
              name={project.name}
              imgUrl={project.imgUrl}
            />
          </El>
          <Members members={project.members} />
        </>;
      }
    | Failed(reason) =>
      <p>
        <strong> {React.string("Error:")} </strong>
        {React.string(reason)}
      </p>
    };

  <El style=Positioning.gridMediumCentered>
    <El style={margin(0, 0, 24, 0)}> <Breadcrumb page=Router.Projects /> </El>
    content
  </El>;
};
