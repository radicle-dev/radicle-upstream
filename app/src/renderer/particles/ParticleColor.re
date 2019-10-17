open Css;

type t =
  | Purple
  | Blue
  | Green
  | Orange
  | Red
  | Bordeaux
  | LightBlue
  | Teal
  | LightGreen
  | Yellow
  | Pink
  | Black
  | DarkGray
  | Gray
  | LightGray
  | AlmostWhite
  | White;

let rgbaOfColor = (color, alpha) =>
  switch (color) {
  | Purple => rgba(110, 65, 224, alpha)
  | Blue => rgba(32, 145, 204, alpha)
  | Green => rgba(32, 212, 121, alpha)
  | Orange => rgba(245, 191, 53, alpha)
  | Red => rgba(231, 80, 80, alpha)
  | Bordeaux => rgba(181, 69, 107, alpha)
  | LightBlue => rgba(153, 159, 239, alpha)
  | Teal => rgba(89, 226, 245, alpha)
  | LightGreen => rgba(36, 243, 139, alpha)
  | Yellow => rgba(227, 237, 19, alpha)
  | Pink => rgba(224, 116, 203, alpha)
  | Black => rgba(40, 51, 61, alpha)
  | DarkGray => rgba(85, 100, 116, alpha)
  | Gray => rgba(145, 160, 175, alpha)
  | LightGray => rgba(206, 216, 225, alpha)
  | AlmostWhite => rgba(248, 248, 248, alpha)
  | White => rgba(255, 255, 255, alpha)
  };

let nameOfColor = color =>
  switch (color) {
  | Purple => "purple"
  | Blue => "blue"
  | Green => "green"
  | Orange => "orange"
  | Red => "red"
  | Bordeaux => "bordeaux"
  | LightBlue => "lightBlue"
  | Teal => "teal"
  | LightGreen => "lightGreen"
  | Yellow => "yellow"
  | Pink => "pink"
  | Black => "black"
  | DarkGray => "darkGray"
  | Gray => "gray"
  | LightGray => "lightGray"
  | AlmostWhite => "almostWhite"
  | White => "white"
  };

let hexOfColor = color =>
  switch (color) {
  | Purple => "#6e41e0"
  | Blue => "#1f91cc"
  | Green => "#20d479"
  | Orange => "#f5bf36"
  | Red => "#e75050"
  | Bordeaux => "#bf456b"
  | LightBlue => "#999ff0"
  | Teal => "#51e2f5"
  | LightGreen => "#24f38b"
  | Yellow => "#e3ed11"
  | Pink => "#e074cb"
  | Black => "#28333d"
  | DarkGray => "#546474"
  | Gray => "#90a0af"
  | LightGray => "#ced8e1"
  | AlmostWhite => "#f8f8f8"
  | White => "#ffffff"
  };

/** Primaries **/
let purple = (~alpha=1.0, ()) => rgbaOfColor(Purple, alpha);
let blue = (~alpha=1.0, ()) => rgbaOfColor(Blue, alpha);
let green = (~alpha=1.0, ()) => rgbaOfColor(Green, alpha);
let orange = (~alpha=1.0, ()) => rgbaOfColor(Orange, alpha);
let red = (~alpha=1.0, ()) => rgbaOfColor(Red, alpha);
let bordeaux = (~alpha=1.0, ()) => rgbaOfColor(Bordeaux, alpha);

/** Secondaries **/
let lightBlue = (~alpha=1.0, ()) => rgbaOfColor(LightBlue, alpha);
let teal = (~alpha=1.0, ()) => rgbaOfColor(Teal, alpha);
let lightGreen = (~alpha=1.0, ()) => rgbaOfColor(LightGreen, alpha);
let yellow = (~alpha=1.0, ()) => rgbaOfColor(Yellow, alpha);
let pink = (~alpha=1.0, ()) => rgbaOfColor(Pink, alpha);

/** Grays **/
let black = (~alpha=1.0, ()) => rgbaOfColor(Black, alpha);
let darkGray = (~alpha=1.0, ()) => rgbaOfColor(DarkGray, alpha);
let gray = (~alpha=1.0, ()) => rgbaOfColor(Gray, alpha);
let lightGray = (~alpha=1.0, ()) => rgbaOfColor(LightGray, alpha);
let almostWhite = (~alpha=1.0, ()) => rgbaOfColor(AlmostWhite, alpha);
let white = (~alpha=1.0, ()) => rgbaOfColor(White, alpha);
