open Atom;
open Layout;
open Page;
open Molecule;

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

[@react.component]
let make = () => {
  let (isModalVisible, toggleModal) = React.useState(_ => false);

  Router.(
    <header className=Styles.header>
      <Container.TwoColumns>
        ...(
             <Link page=Projects> <Atom.Icon.Logo /> </Link>,
             isModalVisible ?
               <Modal closeButtonCallback={_ => toggleModal(_ => false)}>
                 <JoinNetwork />
               </Modal> :
               <Button.Primary onClick={_ => toggleModal(_ => true)}>
                 {React.string("Join the network")}
               </Button.Primary>,
           )
      </Container.TwoColumns>
    </header>
  );
};
