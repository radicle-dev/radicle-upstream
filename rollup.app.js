import commonjs from "@rollup/plugin-commonjs";
import livereload from "rollup-plugin-livereload";
import resolve from "@rollup/plugin-node-resolve";
import inject from "@rollup/plugin-inject";
import json from "@rollup/plugin-json";
import * as browserifyNodeBuiltins from "browserify/lib/builtins";
import svelte from "rollup-plugin-svelte";
import { terser } from "rollup-plugin-terser";
import typescript from "@rollup/plugin-typescript";
import autoPreprocess from "svelte-preprocess";

const production = !process.env.ROLLUP_WATCH;

export default {
  input: "ui/index.ts",
  output: {
    sourcemap: true,
    format: "iife",
    name: "app",
    file: "public/bundle.js",
  },
  plugins: [
    svelte({
      // enable run-time checks when not in production
      dev: !production,
      // we'll extract any component CSS out into
      // a separate file — better for performance
      css: css => {
        css.write("public/bundle.css");
      },
      preprocess: autoPreprocess(),
    }),

    resolve({
      browser: true,
      preferBuiltins: true,
      dedupe: importee =>
        importee === "svelte" || importee.startsWith("svelte/"),
    }),

    commonjs(),

    inject({
      modules: {
        process: "_process",
        Buffer: ["buffer", "Buffer"],
      },
    }),

    {
      name: "node-builtins",
      resolveId(importee) {
        if (importee == "util") {
          return { id: require.resolve("util/util.js") };
        }
        const builtinPath = browserifyNodeBuiltins[importee];
        if (builtinPath) {
          return { id: builtinPath };
        }
      },
    },

    typescript({
      // See https://github.com/rollup/plugins/issues/272
      noEmitOnError: production,
    }),

    json(),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    !production && livereload("public"),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    production && terser(),
  ],
  watch: {
    clearScreen: false,
  },
};
