open Router;

type action =
  | Show((page, page))
  | Hide;

type state = option((page, page));

let initialState = None;

let reducer = (_state, action): state =>
  switch (action) {
  | Show((overlay, next)) => Some((overlay, next))
  | Hide => None
  };
