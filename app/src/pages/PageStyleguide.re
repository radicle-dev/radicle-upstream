open Atom;
open DesignSystem;
open Molecule;
open Molecule.Modal;
open Particle.Color;

module Section = {
  open Css;
  open Particle;

  module Styles = {
    let section =
      style([
        marginBottom(px(180)),
        display(`grid),
        gridTemplateColumns([px(342), auto, px(100)]),
      ]);

    let firstColumn = style([gridColumnStart(1), marginRight(px(90))]);
    let secondColumn = style([gridColumnStart(2)]);
  };

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
  module Styles = {
    open Css;

    let row =
      style([display(`flex), marginBottom(px(34)), alignItems(baseline)]);
  };

  [@react.component]
  let make = (~children) => <El style=Styles.row> children </El>;
};

module ColorSwatch = {
  module Template = {
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

  [@react.component]
  let make = (~color) => <Template color />;

  module HalfSize = {
    open Css;
    [@react.component]
    let make = (~color) =>
      <Template color colorStyle={style([height(px(60))])} />;
  };
};

module FontSwatch = {
  open Css;
  open Particle;

  module Styles = {
    let label = style([width(px(140))]);
  };

  [@react.component]
  let make = (~label, ~children) =>
    <Row>
      <El style=Styles.label>
        <Text.Small style={style([color(Color.gray())])}> label </Text.Small>
      </El>
      <El> children </El>
    </Row>;
};

module IconSwatch = {
  module Styles = {
    open Css;

    let iconContainer = style([marginRight(px(24))]);
  };

  [@react.component]
  let make = (~children) => <El style=Styles.iconContainer> children </El>;
};

module ButtonSwatch = {
  module Styles = {
    open Css;

    let button = style([marginLeft(px(8)), marginRight(px(8))]);
  };

  [@react.component]
  let make = (~children) => <El style=Styles.button> children </El>;
};

module FormElementSwatch = {
  module Styles = {
    open Css;

    let input = style([width(pct(50.0))]);
  };

  [@react.component]
  let make = (~children) => <El style=Styles.input> children </El>;
};

module CardSwatch = {
  module Styles = {
    open Css;

    let card = style([marginTop(px(32)), marginBottom(px(32))]);
  };

  [@react.component]
  let make = (~children) => <El style=Styles.card> children </El>;
};

module AlertSwatch = {
  module Styles = {
    open Css;

    let alert = style([marginTop(px(16)), marginBottom(px(16))]);
  };

  [@react.component]
  let make = (~children) => <El style=Styles.alert> children </El>;
};

module Styles = {
  open Css;

  let wrapper =
    style([
      width(pct(100.0)),
      height(pct(100.0)),
      backgroundColor(Particle.Color.white()),
      paddingLeft(px(160)),
      paddingTop(px(145)),
    ]);

  let content =
    Positioning.gridFullCentered << style([paddingTop(px(145))]);
};

[@react.component]
let make = () =>
  <Portal>
    <El style=Styles.wrapper>
      <Title.Huge> {React.string("Oscoin Styleguide")} </Title.Huge>
      <El style=Styles.content>
        <Section
          title={React.string("Colors")}
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
          <FontSwatch label={React.string("bigNumber")}>
            <Number.Big> {React.string("0123456789")} </Number.Big>
          </FontSwatch>
          <FontSwatch label={React.string("number")}>
            <Number> {React.string("0123456789")} </Number>
          </FontSwatch>
          <FontSwatch label={React.string("smallNumber")}>
            <Number.Small> {React.string("0123456789")} </Number.Small>
          </FontSwatch>
        </Section>
        <Section
          title={React.string("Icons")}
          subTitle={
            React.string(
              "Icons at 16px, 24px, 36px and 64px width and height with 2px stroke weight, multiple color variations",
            )
          }>
          <Row>
            <IconSwatch> <Icon.Plus /> </IconSwatch>
            <IconSwatch> <Icon.Important /> </IconSwatch>
            <IconSwatch> <Icon.Important color=Green /> </IconSwatch>
            <IconSwatch> <Icon.Important color=Bordeaux /> </IconSwatch>
            <IconSwatch> <Icon.CloseSmall /> </IconSwatch>
            <IconSwatch> <Icon.Info /> </IconSwatch>
            <IconSwatch> <Icon.Check /> </IconSwatch>
            <IconSwatch> <Icon.Search /> </IconSwatch>
            <IconSwatch> <Icon.Graph /> </IconSwatch>
            <IconSwatch> <Icon.Inbox /> </IconSwatch>
            <IconSwatch> <Icon.Inbox notificationColor=Purple /> </IconSwatch>
            <IconSwatch> <Icon.Wallet /> </IconSwatch>
            <IconSwatch> <Icon.Close /> </IconSwatch>
          </Row>
          <Row> <IconSwatch> <Icon.Back /> </IconSwatch> </Row>
          <Row>
            <IconSwatch> <Icon.PersonAvatarPlaceholder /> </IconSwatch>
          </Row>
          <Row>
            <IconSwatch> <Icon.ProjectAvatarPlaceholder /> </IconSwatch>
          </Row>
        </Section>
        <Section
          title={React.string("Buttons")}
          subTitle={
            React.string(
              "Vanilla, Primary, Secondary, Cancel, disabled state",
            )
          }>
          <Row>
            <ButtonSwatch>
              <Button> {React.string("Vanilla")} </Button>
            </ButtonSwatch>
            <ButtonSwatch>
              <Button.Primary> {React.string("Primary")} </Button.Primary>
            </ButtonSwatch>
            <ButtonSwatch>
              <Button.Secondary>
                {React.string("Secondary")}
              </Button.Secondary>
            </ButtonSwatch>
            <ButtonSwatch>
              <Button.Cancel> {React.string("Cancel")} </Button.Cancel>
            </ButtonSwatch>
          </Row>
          <Row>
            <ButtonSwatch>
              <Button disabled=true> {React.string("Vanilla")} </Button>
            </ButtonSwatch>
            <ButtonSwatch>
              <Button.Primary disabled=true>
                {React.string("Primary")}
              </Button.Primary>
            </ButtonSwatch>
            <ButtonSwatch>
              <Button.Secondary disabled=true>
                {React.string("Secondary")}
              </Button.Secondary>
            </ButtonSwatch>
            <ButtonSwatch>
              <Button.Cancel disabled=true>
                {React.string("Cancel")}
              </Button.Cancel>
            </ButtonSwatch>
          </Row>
        </Section>
        <Section
          title={React.string("Form elements")}
          subTitle={React.string("Inputs, text areas, dropdowns, etc.")}>
          <FormElementSwatch>
            <Input placeholder="Hey an input, type something here" />
          </FormElementSwatch>
        </Section>
        <Section
          title={React.string("Links")}
          subTitle={React.string("Links to other pages, breadcrumbs, etc.")}>
          <Breadcrumb page=Router.Projects />
          <Link page=Router.RegisterProject>
            {React.string("I'm a link")}
          </Link>
        </Section>
        <Section
          title={React.string("Cards")}
          subTitle={React.string("Projects, persons, etc")}>
          <CardSwatch>
            <PersonCard firstName="Elefterios" lastName="Diakomichalis" />
          </CardSwatch>
          <CardSwatch>
            <ProjectCard
              name="Monadic"
              description="Open source organization of amazing things"
            />
          </CardSwatch>
          <CardSwatch>
            <ProjectCard.Alternate
              name="Monadic"
              description="Open source organization of amazing things"
            />
          </CardSwatch>
        </Section>
        <Section
          title={React.string("Alerts")}
          subTitle={React.string("Info, Success, Error")}>
          <AlertSwatch>
            <Alert onClick={_ => Js.log("Close info alert")}>
              {React.string("Info")}
            </Alert>
          </AlertSwatch>
          <AlertSwatch>
            <Alert.Success onClick={_ => Js.log("Close success alert")}>
              {React.string("Success")}
            </Alert.Success>
          </AlertSwatch>
          <AlertSwatch>
            <Alert.Error onClick={_ => Js.log("Close error alert")}>
              {React.string("Error")}
            </Alert.Error>
          </AlertSwatch>
        </Section>
      </El>
    </El>
  </Portal>;
