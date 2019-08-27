type action =
  | Fetch;

type state =
  | Empty
  | Fetching
  | Present
  | Failed(string);

let initialState = Empty;

let reducer = (_state, action) =>
  switch (action) {
  | Fetch => Fetching
  };
