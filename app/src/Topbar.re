open Atom;

module JoinNetwork = {
  [@react.component]
  let make = () => <Button style=Button.Styles.primary text="Join Network" />;
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
    <header>
      <Link page=Root> <Atom.Icon.Logo /> </Link>
      <Navigation />
      <JoinNetwork />
    </header>
  );
