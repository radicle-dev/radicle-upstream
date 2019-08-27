open Atom;
open Layout;
open Page;

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
let make = () => {
  let (isModalVisible, toggleModal) = React.useState(_ => false);

  Router.(
    <header className=Styles.header>
      <Container.TwoColumns>
        ...(
             <> <Link page=Root> <Atom.Icon.Logo /> </Link> <Navigation /> </>,
             isModalVisible ?
               <JoinNetwork
                 closeButtonCallback={_ => toggleModal(_ => false)}
               /> :
               <Button.Primary onClick={_ => toggleModal(_ => true)}>
                 {React.string("Join the network")}
               </Button.Primary>,
           )
      </Container.TwoColumns>
    </header>
  );
};
