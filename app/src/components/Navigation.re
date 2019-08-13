open Router;

module Item = {
  [@react.component]
  let make = (~page: page) => {
    let name = nameOfPage(page);

    <li> <a onClick={navigateOfPage(page)}> {React.string(name)} </a> </li>;
  };
};

[@react.component]
let make = () =>
  <ul>
    <Item page=Root />
    <Item page=Projects />
    <Item page={Project("monokel")} />
  </ul>;
