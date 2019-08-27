[@bs.val] [@bs.return nullable]
external _getElementById: string => option(Dom.element) =
  "document.getElementById";

module Styles = {
  open Css;
  open Particle;

  let modal =
    style([
      position(fixed),
      width(pct(100.0)),
      height(pct(100.0)),
      display(`flex),
      backgroundColor(Color.white()),
      justifyContent(center),
      alignItems(center),
    ]);
  let closeButton =
    style([
      cursor(`pointer),
      position(`absolute),
      top(px(0)),
      right(px(0)),
      padding(px(36)),
    ]);
  let content = style([width(px(390)), textAlign(center)]);
};

module Portal = {
  [@react.component]
  let make = (~children) =>
    switch (_getElementById("portal")) {
    | None => raise(Not_found)
    | Some(modalElement) => ReactDOMRe.createPortal(children, modalElement)
    };
};

[@react.component]
let make = (~children, ~closeButtonCallback) =>
  <Portal>
    <div className=Styles.modal>
      <div className=Styles.closeButton onClick={_ => closeButtonCallback()}>
        <Atom.Icon.Close />
      </div>
      <div className=Styles.content> children </div>
    </div>
  </Portal>;
