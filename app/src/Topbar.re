open AppStore;
open Atom;
open DesignSystem;
open Page;
open Molecule;
open Source;
open StoreSession;

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

module Account = {
  module Card = {
    [@react.component]
    let make = (~account: Source.account) =>
      <p>
        <strong> {React.string("Account:")} </strong>
        {React.string(account.keyName)}
      </p>;
  };

  module JoinButton = {
    [@react.component]
    let make = (~toggleModal) =>
      <Button.Primary onClick={_ => toggleModal(_ => true)}>
        {React.string("Join the network")}
      </Button.Primary>;
  };

  [@react.component]
  let make = () => {
    let (isModalVisible, toggleModal) = React.useState(_ => false);
    let state = Store.useSelector(state => state.session);
    let dispatch = Store.useDispatch();

    if (state == StoreSession.Initial) {
      dispatch(StoreMiddleware.Thunk(ThunkSession.fetchSession));
    };

    let card =
      switch (state) {
      | Initial
      | Fetching =>
        <Button.Primary disabled=true>
          {React.string("Loading...")}
        </Button.Primary>
      | Empty => <JoinButton toggleModal />
      | Present(account) =>
        <PersonCard firstName={account.keyName} imgUrl={account.avatarUrl} />
      | Failed(reason) =>
        <p>
          <strong> {React.string("Error:")} </strong>
          {React.string(reason)}
        </p>
      };

    isModalVisible ?
      <Modal closeButtonCallback={_ => toggleModal(_ => false)}>
        <JoinNetwork onComplete={_ => toggleModal(_ => false)} />
      </Modal> :
      card;
  };
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
      <El style=Layout.flex>
        <El style=Positioning.flexLeft>
          <Link page=Projects> <Atom.Icon.Logo /> </Link>
        </El>
        <El style=Positioning.flexRight> <Account /> </El>
      </El>
    </header>
  );
