open Router;
open DesignSystem;

[@react.component]
let make = (~page: page, ~style=?, ~children=?) => {
  let content =
    switch (children) {
    | Some(child) => child
    | None => React.string(nameOfPage(page))
    };

  <a onClick={navigateToPage(page)} className={Css.style([]) <<? style}>
    content
  </a>;
};
