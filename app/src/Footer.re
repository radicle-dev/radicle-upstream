open Router;
open Atom;

module Styles = {
  open Css;

  let footer =
    style([
      display(`flex),
      justifyContent(`spaceAround),
      alignItems(`center),
      height(px(118)),
    ]);

  let li = style([marginLeft(px(24)), display(inline)]);
};

[@react.component]
let make = () =>
  <footer className=Styles.footer>
    <Link page=Root> <Atom.Icon.Logo /> </Link>
    <ul>
      <li className=Styles.li>
        <a href="/"> {React.string("What is oscoin?")} </a>
      </li>
      <li className=Styles.li>
        <a href="/"> {React.string("Maintainers")} </a>
      </li>
      <li className=Styles.li>
        <a href="/"> {React.string("Contributors")} </a>
      </li>
      <li className=Styles.li>
        <a href="/"> {React.string("Supporters")} </a>
      </li>
      <li className=Styles.li>
        <a href="/"> {React.string("Security")} </a>
      </li>
      <li className=Styles.li>
        <a href="/"> {React.string("Privacy")} </a>
      </li>
    </ul>
  </footer>;
