open Atom;
open DesignSystem;
open Molecule.Modal;
open Particle.Color;

module Styles = {
  open Css;

  let wrapper =
    style([
      width(pct(100.0)),
      height(pct(100.0)),
      backgroundColor(Particle.Color.white()),
      paddingLeft(px(160)),
    ]);

  let content =
    Positioning.gridFullCentered << style([paddingTop(px(145))]);

  let section =
    style([
      marginBottom(px(180)),
      display(`grid),
      gridTemplateColumns([px(342)]),
    ]);

  let firstColumn = style([gridColumnStart(1), marginRight(px(90))]);

  let secondColumn = style([gridColumnStart(2)]);

  let row =
    style([display(`flex), marginBottom(px(34)), alignItems(baseline)]);
  let iconContainer = style([marginRight(px(24))]);

  let label = style([width(px(140))]);
};

module ColorSwatchTemplate = {
  open Css;

  module Styles = {
    let container = style([marginRight(px(24))]);

    let color =
      style([width(px(120)), height(px(120)), marginBottom(px(16))]);
  };

  [@react.component]
  let make = (~color, ~containerStyle=?, ~colorStyle=?) =>
    <El style={Styles.container <<? containerStyle}>
      <div
        className={
          Styles.color
          << style([backgroundColor(rgbaOfColor(color, 1.0))])
          <<? colorStyle
        }
      />
      <Title> {React.string(nameOfColor(color))} </Title>
      <Text.Small style={style([textTransform(`uppercase)])}>
        {React.string(hexOfColor(color))}
      </Text.Small>
    </El>;
};

module ColorSwatch = {
  [@react.component]
  let make = (~color) => <ColorSwatchTemplate color />;

  module HalfSize = {
    open Css;
    [@react.component]
    let make = (~color) =>
      <ColorSwatchTemplate color colorStyle={style([height(px(60))])} />;
  };
};

module Section = {
  open Css;
  open Particle;

  [@react.component]
  let make = (~title, ~subTitle, ~children) =>
    <El style=Styles.section>
      <El style=Styles.firstColumn>
        <Title.Big style={style([marginBottom(px(8))])}> title </Title.Big>
        <Text style={style([color(Color.gray())])}> subTitle </Text>
      </El>
      <El style=Styles.secondColumn> children </El>
    </El>;
};

module Row = {
  [@react.component]
  let make = (~children) => <El style=Styles.row> children </El>;
};

module FontSwatch = {
  open Css;
  open Particle;

  [@react.component]
  let make = (~label, ~children) =>
    <Row>
      <El style=Styles.label>
        <Text.Small style={style([color(Color.gray())])}> label </Text.Small>
      </El>
      <El> children </El>
    </Row>;
};

[@react.component]
let make = () =>
  <Portal>
    <El style=Styles.wrapper>
      <El style=Styles.content>
        <Section
          title={React.string("Oscoin Styleguide")}
          subTitle={React.string("Primary, secondary and grays")}>
          <Row>
            <ColorSwatch color=Purple />
            <ColorSwatch color=Blue />
            <ColorSwatch color=Green />
            <ColorSwatch color=Orange />
            <ColorSwatch color=Red />
            <ColorSwatch color=Bordeaux />
          </Row>
          <Row>
            <ColorSwatch color=LightBlue />
            <ColorSwatch color=Teal />
            <ColorSwatch color=LightGreen />
            <ColorSwatch color=Yellow />
            <ColorSwatch color=Pink />
          </Row>
          <Row>
            <ColorSwatch.HalfSize color=Black />
            <ColorSwatch.HalfSize color=DarkGray />
            <ColorSwatch.HalfSize color=Gray />
            <ColorSwatch.HalfSize color=LightGray />
            <ColorSwatch.HalfSize color=AlmostWhite />
            <ColorSwatch.HalfSize color=White />
          </Row>
        </Section>
        <Section
          title={React.string("Typography")}
          subTitle={
            React.string(
              "Using GT America and GT America Mono from Grill Type",
            )
          }>
          <FontSwatch label={React.string("hugeTitle")}>
            <Title.Huge> {React.string("Open Source Coin")} </Title.Huge>
          </FontSwatch>
          <FontSwatch label={React.string("bigTitle")}>
            <Title.Big> {React.string("Open Source Coin")} </Title.Big>
          </FontSwatch>
          <FontSwatch label={React.string("title")}>
            <Title> {React.string("Open Source Coin")} </Title>
          </FontSwatch>
          <FontSwatch label={React.string("text")}>
            <Text> {React.string("Open Source Coin")} </Text>
          </FontSwatch>
          <FontSwatch label={React.string("smallText")}>
            <Text.Small> {React.string("Open Source Coin")} </Text.Small>
          </FontSwatch>
          <FontSwatch label={React.string("caption")}>
            <Text.Caption> {React.string("Open Source Coin")} </Text.Caption>
          </FontSwatch>
        </Section>
        <Section
          title={React.string("Icons")}
          subTitle={
            React.string(
              "Icons at 16px, 24px, 36px and 64px width and height with 2px stroke weight",
            )
          }>
          <Row>
            <El style=Styles.iconContainer> <Icon.Plus /> </El>
            <El style=Styles.iconContainer> <Icon.Important /> </El>
            <El style=Styles.iconContainer> <Icon.CloseSmall /> </El>
            <El style=Styles.iconContainer> <Icon.Info /> </El>
            <El style=Styles.iconContainer> <Icon.Check /> </El>
            <El style=Styles.iconContainer> <Icon.Search /> </El>
            <El style=Styles.iconContainer> <Icon.Graph /> </El>
            <El style=Styles.iconContainer> <Icon.Inbox /> </El>
            <El style=Styles.iconContainer> <Icon.Wallet /> </El>
            <El style=Styles.iconContainer> <Icon.Close /> </El>
          </Row>
          <Row> <El style=Styles.row> <Icon.Back /> </El> </Row>
          <Row>
            <El style=Styles.row> <Icon.PersonAvatarPlaceholder /> </El>
          </Row>
          <Row>
            <El style=Styles.row> <Icon.ProjectAvatarPlaceholder /> </El>
          </Row>
        </Section>
      </El>
    </El>
  </Portal>;
