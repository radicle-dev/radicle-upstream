open Atom;
open DesignSystem;

module Styles = {
  open Css;

  let personCard = style([display(`flex), alignItems(center)]);

  let image =
    style([height(px(36)), marginRight(px(16)), width(px(36))]);

  let imageContainer = style([marginRight(px(16)), display(inherit_)]);
};

[@react.component]
let make = (~imgUrl=?, ~firstName, ~lastName=?) => {
  let text =
    switch (lastName) {
    | Some(lastName) => firstName ++ " " ++ lastName
    | None => firstName
    };

  let image =
    switch (imgUrl) {
    | Some(imgUrl) => <img className=Styles.image src=imgUrl />
    | None =>
      <Container style=Styles.imageContainer>
        <Icon.PersonAvatarPlaceholder />
      </Container>
    };

  <div className=Styles.personCard>
    image
    <Text> {React.string(text)} </Text>
  </div>;
};
