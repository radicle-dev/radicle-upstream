let store = AppStore.createStore();

module Context = {
  module type Config = {
    type context;
    let defaultValue: context;
  };

  module Make = (Config: Config) => {
    let context = React.createContext(Config.defaultValue);

    module Provider = {
      let make = context->React.Context.provider;

      [@bs.obj]
      external makeProps:
        (
          ~value: Config.context,
          ~children: React.element,
          ~key: string=?,
          unit
        ) =>
        {
          .
          "value": Config.context,
          "children": React.element,
        } =
        "";
    };
  };
};

module StoreContext = {
  include Context.Make({
    type context = AppStore.t;
    let defaultValue = store;
  });
};

module Provider = {
  [@react.component]
  let make = (~children) =>
    <StoreContext.Provider value=store> children </StoreContext.Provider>;
};

let useSelector = selector => {
  let storeFromContext = React.useContext(StoreContext.context);
  let (_, forceUpdate) = React.useReducer((s, _) => s + 1, 0);

  let latestSelectedModel =
    React.useRef(selector(Reductive.Store.getState(store)));

  React.useLayoutEffect1(
    () => {
      let checkForUpdate = () => {
        let newSelectedState = selector(Reductive.Store.getState(store));

        let hasStateChanged =
          newSelectedState != React.Ref.current(latestSelectedModel);

        if (hasStateChanged) {
          React.Ref.setCurrent(latestSelectedModel, newSelectedState);
          forceUpdate();
        };
      };
      Some(Reductive.Store.subscribe(storeFromContext, checkForUpdate));
    },
    [|storeFromContext|],
  );

  React.Ref.current(latestSelectedModel);
};

let useDispatch = () => {
  let storeFromContext = React.useContext(StoreContext.context);
  Reductive.Store.dispatch(storeFromContext);
};
