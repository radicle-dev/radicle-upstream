open Molecule;
open Atom;
open DesignSystem;

type projectPage =
  | Overview
  | Code
  | Funds;

module Navigation = {
  open Router;
  open Css;

  module Item = {
    [@react.component]
    let make = (~id: string, ~page: projectPage, ~selected: projectPage) => {
      let (navigate, name) =
        switch (page) {
        | Overview => (navigateToPage(Project(id)), "Overview")
        | Code => (navigateToPage(ProjectCode(id)), "Code")
        | Funds => (navigateToPage(ProjectFunds(id)), "Funds")
        };

      let name = page == selected ? name ++ " <" : name;

      <li> <a onClick=navigate> {React.string(name)} </a> </li>;
    };
  };

  [@react.component]
  let make = (~id: string, ~subPage: projectPage) =>
    <ul className={style([display(none)])}>
      <Item id page=Overview selected=subPage />
      <Item id page=Code selected=subPage />
      <Item id page=Funds selected=subPage />
    </ul>;
};

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

[@react.component]
let make = (~id: string, ~subPage: projectPage) =>
  <El style=Positioning.mediumWidthCentered>
    <El style={margin(0, 0, 50, 0)}>
      <El style={margin(0, 0, 24, 0)}>
        <Breadcrumb page=Router.Projects />
      </El>
      <ProjectCard.Alternate
        name="Monadic"
        description="Open source organization of amazing things"
      />
    </El>
    <El style=Styles.membersHeading>
      <Title> {React.string("Members")} </Title>
    </El>
    <ul className=Styles.list>
      <li>
        <PersonCard
          firstName="Elefterios"
          imgUrl="https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg"
        />
      </li>
      <li> <PersonCard firstName="Willy" lastName="Gomez" /> </li>
    </ul>
    <Navigation id subPage />
  </El>;
