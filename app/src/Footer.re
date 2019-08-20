module Styles = {
  open Css;

  let footer =
    style([
      gridColumnStart(2),
      gridColumnEnd(8),
      gridRowStart(-1),
      height(px(64)),
      justifySelf(center),
      display(grid),
    ]);

  let ul =
    style([
      children([marginRight(px(24)), display(inline), alignSelf(center)]),
      alignSelf(center),
    ]);
};

[@react.component]
let make = () =>
  <footer className=Styles.footer>
    <ul className=Styles.ul>
      <li> <a href="/"> {React.string("What is oscoin?")} </a> </li>
      <li> <a href="/"> {React.string("Maintainers")} </a> </li>
      <li> <a href="/"> {React.string("Contributors")} </a> </li>
      <li> <a href="/"> {React.string("Supporters")} </a> </li>
      <li> <a href="/"> {React.string("Security")} </a> </li>
      <li> <a href="/"> {React.string("Privacy")} </a> </li>
    </ul>
  </footer>;
