type thunk('state) = ..;

type thunkFunc('state) = (thunk('state) => unit, Source.source) => unit;

type thunk('state) +=
  | Thunk(thunkFunc('state));

let middleware = (source: Source.source, store, next, action) =>
  switch (action) {
  | Thunk(func) => func(Reductive.Store.dispatch(store), source)
  | _ => next(action)
  };
