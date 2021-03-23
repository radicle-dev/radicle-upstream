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
import css from "rollup-plugin-css-only";

const watch = !!process.env.ROLLUP_WATCH;
const release =
  process.env.ROLLUP_RELEASE == "1" || process.env.ROLLUP_RELEASE == "true";

export default {
  input: "ui/index.ts",
  output: {
    sourcemap: true,
    format: "iife",
    name: "app",
    file: "public/bundle.js",
  },
  plugins: [
    json(),
    svelte({
      compilerOptions: {
        // enable run-time checks when not in production
        dev: !release,
      },
      preprocess: autoPreprocess(),
    }),

    css({ output: "bundle.css" }),

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
        if (importee === "util") {
          // We need a more recent version than browserify provides
          return { id: require.resolve("util/util.js") };
        }
        const builtinPath = browserifyNodeBuiltins[importee];
        if (builtinPath) {
          return { id: builtinPath };
        }
      },
    },

    typescript(),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    watch && livereload("public"),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    release && terser(),
  ],
  watch: {
    clearScreen: false,
  },

  // Skip certain warnings originated by third-party libraries
  onwarn: function (warning) {
    if (
      warning.code === "THIS_IS_UNDEFINED" &&
      warning.id.includes("node_modules/@ethersproject/")
    ) {
      return;
    }

    if (
      warning.code === "CIRCULAR_DEPENDENCY" &&
      warning.importer.includes("node_modules")
    ) {
      return;
    }

    // Pass on any other warnings
    console.warn(warning.message);
  },
};
