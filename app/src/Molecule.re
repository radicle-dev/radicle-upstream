open Source.Project;
open Atom;
open Particle;

module Styles = {
  open Css;

  let item =
    style([
      width(pct(100.0)),
      display(`flex),
      flex(`num(1.0)),
      padding(px(13)),
      borderBottom(px(1), solid, Color.lightGray()),
      lastChild([borderBottomWidth(px(0))]),
    ]);

  let description =
    style([
      display(`flex),
      flexDirection(`column),
      justifyContent(`center),
    ]);

  let link = style([display(`flex)]);

  let image =
    style([marginRight(px(21)), width(px(64)), height(px(64))]);
};

module ProjectCard = {
  [@react.component]
  let make = (~project) =>
    <li className=Styles.item key={project.address}>
      <Link style=Styles.link page={Router.Project(project.address)}>
        <img className=Styles.image src={project.imgUrl} />
        <div className=Styles.description>
          <Title> {React.string(project.name)} </Title>
          <p> {React.string(project.description)} </p>
        </div>
      </Link>
    </li>;
};
