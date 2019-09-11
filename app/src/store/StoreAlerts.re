type action =
  | Show(Molecule.Alert.t)
  | Remove;

type state = {latest: option(Molecule.Alert.t)};

let initialState = {latest: None};

let reducer = (_state, action) =>
  switch (action) {
  | Show(alert) => {latest: Some(alert)}
  | Remove => {latest: None}
  };
