open Atom;
open DesignSystem;

module Styles = {
  open Css;

  let card = style([display(`flex), flex(`num(1.0)), width(pct(100.0))]);

  let description =
    style([
      display(`flex),
      flexDirection(`column),
      justifyContent(`center),
    ]);

  let image =
    style([height(px(64)), marginRight(px(21)), width(px(64))]);

  let imageContainer = style([marginRight(px(22)), display(inherit_)]);
};

module Template = {
  module Image = {
    [@react.component]
    let make = (~imgUrl=?) => {
      let empty =
        <El style=Styles.imageContainer>
          <Icon.ProjectAvatarPlaceholder />
        </El>;

      switch (imgUrl) {
      | Some(imgUrl) =>
        switch (imgUrl) {
        | "" => empty
        | _ => <img className=Styles.image src=imgUrl />
        }
      | None => empty
      };
    };
  };

  [@react.component]
  let make = (~style=?, ~children, ~imgUrl=?, ~description) =>
    <div className={Styles.card <<? style}>
      <Image ?imgUrl />
      <div className=Styles.description>
        children
        <Text> {React.string(description)} </Text>
      </div>
    </div>;
};

[@react.component]
let make = (~style=?, ~imgUrl=?, ~name, ~description) =>
  <Template ?style ?imgUrl description>
    <Title> {React.string(name)} </Title>
  </Template>;

module Alternate = {
  [@react.component]
  let make = (~style=?, ~imgUrl=?, ~name, ~description) =>
    <Template ?style ?imgUrl description>
      <Title.Big style={margin(0, 0, 6, 0)}>
        {React.string(name)}
      </Title.Big>
    </Template>;
};
