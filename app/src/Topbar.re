open Atom;
open DesignSystem;
open Page;
open Molecule;

[@react.component]
let make = () => {
  let (isModalVisible, toggleModal) = React.useState(_ => false);

  Router.(
    <header>
      <El style=Layout.flex>
        <El style=Positioning.flexLeft>
          <Link page=Projects> <Atom.Icon.Logo /> </Link>
        </El>
        <El style=Positioning.flexRight>
          {
            isModalVisible ?
              <Modal closeButtonCallback={_ => toggleModal(_ => false)}>
                <JoinNetwork
                  cancelButtonCallback={_ => toggleModal(_ => false)}
                />
              </Modal> :
              <Button.Primary onClick={_ => toggleModal(_ => true)}>
                {React.string("Join the network")}
              </Button.Primary>
          }
        </El>
      </El>
    </header>
  );
};
