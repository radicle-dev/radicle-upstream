open Atom;
open DesignSystem.Layout;
open Page;
open Molecule;

[@react.component]
let make = () => {
  let (isModalVisible, toggleModal) = React.useState(_ => false);

  Router.(
    <header>
      <Container.TwoColumns>
        ...(
             <Link page=Projects> <Atom.Icon.Logo /> </Link>,
             isModalVisible ?
               <Modal closeButtonCallback={_ => toggleModal(_ => false)}>
                 <JoinNetwork
                   cancelButtonCallback={_ => toggleModal(_ => false)}
                 />
               </Modal> :
               <Button.Primary onClick={_ => toggleModal(_ => true)}>
                 {React.string("Join the network")}
               </Button.Primary>,
           )
      </Container.TwoColumns>
    </header>
  );
};
