open Router;

module Item = {
  [@react.component]
  let make = (~page: page) => {
    let link = linkOfPage(page);
    let name = nameOfPage(page);

    <li>
      <a onClick={_ => ReasonReactRouter.push(link)}>
        {React.string(name)}
      </a>
    </li>;
  };
};

[@react.component]
let make = () => <ul> <Item page=Root /> <Item page=Projects /> </ul>;
