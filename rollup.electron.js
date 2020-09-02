import babel from "@rollup/plugin-babel";
import externals from "rollup-plugin-node-externals";

export default {
  input: "native/main.js",
  output: {
    file: "native/main.comp.js",
    format: "cjs",
  },
  plugins: [
    babel({
      babelHelpers: 'runtime',
      exclude: "node_modules/**",
    }),

    // This avoids the following warning:
    //
    // (!) Unresolved dependencies
    // https://rollupjs.org/guide/en/#warning-treating-module-as-external-dependency
    externals({ builtins: true, deps: true }),
  ],
};
