import babel from "rollup-plugin-babel";
import externals from "rollup-plugin-node-externals";

export default {
  input: "native/main.js",
  output: {
    file: "native/main.comp.js",
    format: "cjs",
  },
  plugins: [
    babel({
      exclude: "node_modules/**",
      // We need the runtimeHelpers to avoid
      // "ReferenceError: regeneratorRuntime is not defined"
      // when using ipcMain.handle
      //
      // The following babel plugins (and .babelrc config) are also required
      // for this to work:
      // @babel/plugin-transform-runtime
      // @babel/runtime
      runtimeHelpers: true,
    }),

    // This avoids the following warning:
    //
    // (!) Unresolved dependencies
    // https://rollupjs.org/guide/en/#warning-treating-module-as-external-dependency
    externals({ builtins: true, deps: true }),
  ],
};
