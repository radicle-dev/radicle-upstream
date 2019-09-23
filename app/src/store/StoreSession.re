open Source;

type action =
  | Fetch
  | Fetched(option(account))
  | FetchFailed(string)
  | Created(account)
  | CreationFailed(string);

type remoteState =
  | Initial
  | Empty
  | Fetching
  | Failed(string);

type state =
  | Present(Source.account)
  | NotPresent(remoteState);

let initialState = NotPresent(Initial);

let reducer = (_state, action) =>
  switch (action) {
  | Fetch => NotPresent(Fetching)
  | Fetched(maybeAccount) =>
    switch (maybeAccount) {
    | Some(account) => Present(account)
    | None => NotPresent(Empty)
    }
  | FetchFailed(reason) => NotPresent(Failed(reason))
  | Created(account) => Present(account)
  | CreationFailed(reason) => NotPresent(Failed(reason))
  };
