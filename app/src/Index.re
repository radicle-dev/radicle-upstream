open Css;

global("body", [color(Particle.Color.black()), ...Particle.Font.text]);

ReactDOMRe.renderToElementWithId(<App />, "app");
