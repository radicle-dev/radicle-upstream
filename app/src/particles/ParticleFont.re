open Css;

let gtBold =
  fontFace(
    ~fontFamily="GT America Bold",
    ~src=[url("/fonts/GTAmericaBold.otf")],
    (),
  );

let gtMedium =
  fontFace(
    ~fontFamily="GT America Medium",
    ~src=[url("/fonts/GTAmericaMedium.otf")],
    (),
  );

let gtRegular =
  fontFace(
    ~fontFamily="GT America Regular",
    ~src=[url("/fonts/GTAmericaRegular.otf")],
    (),
  );

let gtMonoBold =
  fontFace(
    ~fontFamily="GT America Mono Bold",
    ~src=[url("/fonts/GTAmericaMonoBold.otf")],
    (),
  );

let gtMonoMedium =
  fontFace(
    ~fontFamily="GT America Mono Medium",
    ~src=[url("/fonts/GTAmericaMonoMedium.otf")],
    (),
  );

let gtMonoRegular =
  fontFace(
    ~fontFamily="GT America Mono Regular",
    ~src=[url("/fonts/GTAmericaMonoRegular.otf")],
    (),
  );

/** Titles **/

let hugeTitle = [
  fontFamily(gtBold),
  fontSize(px(36)),
  lineHeight(px(43)),
];

let bigTitle = [
  fontFamily(gtBold),
  fontSize(px(24)),
  lineHeight(px(29)),
];

let title = [
  fontFamily(gtMedium),
  fontSize(px(16)),
  lineHeight(`percent(130.0)),
];

/** Texts **/

let text = [fontFamily(gtRegular), fontSize(px(16)), lineHeight(px(24))];

let smallText = [
  fontFamily(gtRegular),
  fontSize(px(14)),
  lineHeight(px(20)),
];

/** Captions **/

let caption = [
  fontFamily(gtMedium),
  fontSize(px(13)),
  lineHeight(px(16)),
  letterSpacing(em(0.1)),
  textTransform(uppercase),
];

/** Numbers **/

let bigNumber = [
  fontFamily(gtMonoBold),
  fontSize(px(22)),
  lineHeight(px(27)),
];

let number = [
  fontFamily(gtMonoMedium),
  fontSize(px(16)),
  lineHeight(px(19)),
];

let smallNumber = [
  fontFamily(gtMonoMedium),
  fontSize(px(12)),
  lineHeight(px(14)),
];
