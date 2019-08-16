open Atom;

[@react.component]
let make = () =>
  <>
    <Title.Big> {React.string("Register a project")} </Title.Big>
    <a onClick={_ => Source.Project.registerProject("foo")}>
      <Button.Secondary> {React.string("Register")} </Button.Secondary>
    </a>
  </>;
