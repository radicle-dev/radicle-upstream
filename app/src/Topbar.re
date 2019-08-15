open Atom;

module Styles = {
  open Css;

  let topBar =
    style([
      display(`flex),
      justifyContent(`spaceAround),
      alignItems(`center),
      height(px(118)),
    ]);
};

module JoinNetwork = {
  [@react.component]
  let make = () =>
    <Button style=Button.Styles.primary>
      {React.string("Join the network")}
    </Button>;
};

module Navigation = {
  open Router;

  [@react.component]
  let make = () =>
    <ul>
      <li> <Link page=Projects> {React.string("Explore")} </Link> </li>
      <li> <Link page={Project("monokel")} /> </li>
    </ul>;
};

[@react.component]
let make = () =>
  Router.(
    <header className=Styles.topBar>
      <Link page=Root> <Atom.Icon.Logo /> </Link>
      <Navigation />
      <JoinNetwork />
    </header>
  );
