import babel from "rollup-plugin-babel";
import externals from "rollup-plugin-node-externals";

export default {
  input: "native/main.js",
  output: {
    file: "native/main.comp.js",
    format: "cjs"
  },
  plugins: [
    babel({
      exclude: "node_modules/**"
    }),
    externals()
  ]
};
