open Router;

[@react.component]
let make = () =>
  <footer>
    <Link page=Root> <Icon.Logo /> </Link>
    <ul>
      <li> <a href="/"> {React.string("What is oscoin?")} </a> </li>
      <li> <a href="/"> {React.string("Maintainers")} </a> </li>
      <li> <a href="/"> {React.string("Contributors")} </a> </li>
      <li> <a href="/"> {React.string("Supporters")} </a> </li>
      <li> <a href="/"> {React.string("Security")} </a> </li>
      <li> <a href="/"> {React.string("Privacy")} </a> </li>
    </ul>
  </footer>;
