import commonjs from "@rollup/plugin-commonjs";
import livereload from "rollup-plugin-livereload";
import resolve from "@rollup/plugin-node-resolve";
import svelte from "rollup-plugin-svelte";
import { terser } from "rollup-plugin-terser";
import typescript from "@rollup/plugin-typescript";
import autoPreprocess from "svelte-preprocess";
import css from "rollup-plugin-css-only";

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
      compilerOptions: {
        // enable run-time checks when not in production
        dev: !production,
      },
      preprocess: autoPreprocess(),
    }),

    css({ output: "bundle.css" }),

    resolve({
      browser: true,
      dedupe: importee =>
        importee === "svelte" || importee.startsWith("svelte/"),
    }),

    commonjs(),

    typescript({
      // See https://github.com/rollup/plugins/issues/272
      noEmitOnError: production,
    }),

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
