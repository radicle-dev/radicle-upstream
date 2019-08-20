module ProjectCard = {
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
        hover([backgroundColor(Color.almostWhite())]),
      ]);

    let description =
      style([
        display(`flex),
        flexDirection(`column),
        justifyContent(`center),
      ]);

    let image =
      style([marginRight(px(21)), width(px(64)), height(px(64))]);
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
