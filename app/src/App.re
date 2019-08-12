[@react.component]
let make = () => {
  open Router;

  let url = ReasonReactRouter.useUrl();
  let page =
    switch (pageOfUrl(url)) {
    | Home => <Generic title="Home of Oscoin" />
    | Projects => <Generic title="List of projects" />
    | NotFound(_path) => <Generic title="Not Found" />
    };

  <div className="app"> <Navigation /> page </div>;
};
