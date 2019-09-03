open Atom;
open DesignSystem;

module Styles = {
  open Css;

  let personCard = style([display(`flex), alignItems(center)]);

  let image =
    style([height(px(36)), marginRight(px(16)), width(px(36))]);

  let imageContainer = style([marginRight(px(16)), display(inherit_)]);
};

module Image = {
  [@react.component]
  let make = (~imgUrl=?) => {
    let empty =
      <El style=Styles.imageContainer> <Icon.PersonAvatarPlaceholder /> </El>;

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
let make = (~imgUrl=?, ~firstName, ~lastName=?) => {
  let text =
    switch (lastName) {
    | Some(lastName) => firstName ++ " " ++ lastName
    | None => firstName
    };

  <div className=Styles.personCard>
    <Image ?imgUrl />
    <Text> {React.string(text)} </Text>
  </div>;
};
