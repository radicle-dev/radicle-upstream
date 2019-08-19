open Router;
open Atom;

module Styles = {
  open Css;

  let footer =
    style([
      gridColumnStart(2),
      gridColumnEnd(8),
      gridRowStart(-1),
      height(px(64)),
      display(`flex),
      justifyContent(`center),
      unsafe("place-self", "end stretch"),
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
