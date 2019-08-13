open Router;

[@react.component]
let make = (~page: page, ~children=?) => {
  let content =
    switch (children) {
    | Some(child) => child
    | None => React.string(nameOfPage(page))
    };

  <a onClick={navigateOfPage(page)}> content </a>;
};
