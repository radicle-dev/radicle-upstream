type thunk('state) = ..;

type thunkFunc('state) =
  (thunk('state) => unit, 'state, Source.source) => unit;

type thunk('state) +=
  | Thunk(thunkFunc('state));

let middleware = (source: Source.source, store, next, action) =>
  switch (action) {
  | Thunk(func) =>
    func(
      Reductive.Store.dispatch(store),
      Reductive.Store.getState(store),
      source,
    )
  | _ => next(action)
  };
