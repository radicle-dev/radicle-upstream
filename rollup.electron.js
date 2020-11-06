import commonjs from "@rollup/plugin-commonjs";
import externals from "rollup-plugin-node-externals";
import typescript from "@rollup/plugin-typescript";

const production = !process.env.ROLLUP_WATCH;

export default {
  input: "native/main.ts",
  output: {
    sourcemap: true,
    file: "native/main.comp.js",
    format: "cjs",
  },
  plugins: [
    commonjs(),

    typescript({
      // See https://github.com/rollup/plugins/issues/272
      noEmitOnError: production,
    }),

    // This avoids the following warning:
    //
    // (!) Unresolved dependencies
    // https://rollupjs.org/guide/en/#warning-treating-module-as-external-dependency
    externals({ builtins: true, deps: true }),
  ],
};
