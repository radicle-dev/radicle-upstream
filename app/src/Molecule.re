module ProjectCard = {
  open Atom;
  open Particle;

  module Styles = {
    open Css;

    let item =
      style([
        borderBottom(px(1), solid, Color.lightGray()),
        display(`flex),
        flex(`num(1.0)),
        padding(px(13)),
        width(pct(100.0)),
        hover([backgroundColor(Color.almostWhite())]),
        lastChild([borderBottomWidth(px(0))]),
      ]);

    let description =
      style([
        display(`flex),
        flexDirection(`column),
        justifyContent(`center),
      ]);

    let image =
      style([height(px(64)), marginRight(px(21)), width(px(64))]);
  };

  [@react.component]
  let make = (~imgUrl, ~name, ~description) =>
    <div className=Styles.item>
      <img className=Styles.image src=imgUrl />
      <div className=Styles.description>
        <Title> {React.string(name)} </Title>
        <Text> {React.string(description)} </Text>
      </div>
    </div>;
};
