open Atom;
open Layout;

module Styles = {
  open Css;

  let header =
    style([
      gridColumnStart(2),
      gridColumnEnd(8),
      height(px(64)),
      paddingTop(px(32)),
    ]);
};

module JoinNetwork = {
  [@react.component]
  let make = () =>
    <Button.Primary> {React.string("Join the network")} </Button.Primary>;
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
    <header className=Styles.header>
      <Container.TwoColumns>
        ...(
             <> <Link page=Root> <Atom.Icon.Logo /> </Link> <Navigation /> </>,
             <JoinNetwork />,
           )
      </Container.TwoColumns>
    </header>
  );
