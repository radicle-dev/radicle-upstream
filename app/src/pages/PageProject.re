type projectPage =
  | Overview
  | Code
  | Funds;

module Navigation = {
  open Router;

  module Item = {
    [@react.component]
    let make = (~id: string, ~page: projectPage, ~selected: projectPage) => {
      let (navigate, name) =
        switch (page) {
        | Overview => (navigateOfPage(Project(id)), "Overview")
        | Code => (navigateOfPage(ProjectCode(id)), "Code")
        | Funds => (navigateOfPage(ProjectFunds(id)), "Funds")
        };

      let name = page == selected ? name ++ " <" : name;

      <li> <a onClick=navigate> {React.string(name)} </a> </li>;
    };
  };

  [@react.component]
  let make = (~id: string, ~subPage: projectPage) =>
    <ul>
      <Item id page=Overview selected=subPage />
      <Item id page=Code selected=subPage />
      <Item id page=Funds selected=subPage />
    </ul>;
};

[@react.component]
let make = (~id: string, ~subPage: projectPage) =>
  <>
    <h1> {React.string("Project " ++ id)} </h1>
    <Navigation id subPage />
  </>;
