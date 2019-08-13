open Router;

[@react.component]
let make = () =>
  <>
    <h1> {React.string("List of projects")} </h1>
    <ul> <li> <Link page={Project("monokel")} /> </li> </ul>
  </>;
