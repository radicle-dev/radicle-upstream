type action =
  | Fetch
  | Fetched(Source.account);

type state =
  | Initial
  | Empty
  | Fetching
  | Present(Source.account)
  | Failed(string);

let initialState = Initial;

let reducer = (_state, action) =>
  switch (action) {
  | Fetch => Fetching
  | Fetched(account) => Present(account)
  };
