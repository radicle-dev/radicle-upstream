import { get } from "svelte/store";
import * as overlay from "./overlay";

describe("setting the store", () => {
  it("properly stores a div", () => {
    document.body.innerHTML = `<div id="overlay" />`;

    const div = document.getElementById("overlay") as HTMLDivElement;
    overlay.open(div);

    expect(get(overlay.current)).toEqual(div);
  });

  it("replaces old div with new one", () => {
    document.body.innerHTML =
      "<div>" +
      '  <div id="first_overlay" />' +
      '  <div id="second_overlay" />' +
      "</div>";

    const firstDiv = document.getElementById("first_overlay") as HTMLDivElement;
    overlay.open(firstDiv);
    expect(get(overlay.current)).toEqual(firstDiv);

    const otherDiv = document.getElementById(
      "second_overlay"
    ) as HTMLDivElement;
    overlay.open(otherDiv);
    expect(get(overlay.current)).toEqual(otherDiv);
  });

  it("clears store upon close", () => {
    document.body.innerHTML =
      "<div>" +
      '  <div id="first_overlay" />' +
      '  <div id="second_overlay" />' +
      "</div>";

    const div = document.getElementById("first_overlay") as HTMLDivElement;
    overlay.open(div);
    expect(get(overlay.current)).toEqual(div);

    overlay.close();
    expect(get(overlay.current)).toBeFalsy();
  });
});
