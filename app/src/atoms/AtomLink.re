open Router;
open DesignSystem;

module Styles = {
  open Css;

  let link = style([userSelect(none)]);
};

[@react.component]
let make = (~page: page, ~style=?, ~children=?) => {
  let content =
    switch (children) {
    | Some(child) => child
    | None => React.string(nameOfPage(page))
    };

  <a onClick={navigateToPage(page)} className={Styles.link <<? style}>
    content
  </a>;
};
