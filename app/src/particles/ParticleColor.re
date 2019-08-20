open Css;

/** Primaries **/
let purple = (~alpha=1.0, ()) => rgba(120, 52, 232, alpha);
let blue = (~alpha=1.0, ()) => rgba(0, 146, 210, alpha);
let green = (~alpha=1.0, ()) => rgba(0, 217, 110, alpha);
let orange = (~alpha=1.0, ()) => rgba(254, 190, 0, alpha);
let red = (~alpha=1.0, ()) => rgba(250, 64, 72, alpha);
let bordeaux = (~alpha=1.0, ()) => rgba(207, 56, 107, alpha);

/** Secondaries **/
let lightBlue = (~alpha=1.0, ()) => rgba(153, 159, 240, alpha);
let teal = (~alpha=1.0, ()) => rgba(0, 229, 248, alpha);
let lightGreen = (~alpha=1.0, ()) => rgba(0, 249, 126, alpha);
let yellow = (~alpha=1.0, ()) => rgba(223, 239, 0, alpha);
let pink = (~alpha=1.0, ()) => rgba(224, 116, 203, alpha);

/** Grays **/
let black = (~alpha=1.0, ()) => rgba(40, 51, 61, alpha);
let darkGray = (~alpha=1.0, ()) => rgba(84, 100, 116, alpha);
let gray = (~alpha=1.0, ()) => rgba(144, 160, 174, alpha);
let lightGray = (~alpha=1.0, ()) => rgba(206, 216, 225, alpha);
let almostWhite = (~alpha=1.0, ()) => rgba(248, 248, 248, alpha);
let white = (~alpha=1.0, ()) => rgba(255, 255, 255, alpha);
