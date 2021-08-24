// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

module.exports = {
  env: {
    node: true,
    browser: true,
    es6: true,
  },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    createDefaultProgram: true,
    ecmaVersion: 2019,
    sourceType: "module",
  },
  ignorePatterns: ["!.license-compliancerc.js"],
  extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
  plugins: ["svelte3", "@typescript-eslint", "no-only-tests"],
  overrides: [
    {
      files: ["*.svelte"],
      processor: "svelte3/svelte3",
    },
    {
      files: ["scripts/*.ts"],
      rules: {
        // Script files are not bundled so we can’t use module imports.
        "@typescript-eslint/no-var-requires": "off",
      },
    },
  ],
  rules: {
    // Disallow Unused Variables
    // https://eslint.org/docs/rules/no-unused-vars
    "@typescript-eslint/no-unused-vars": ["error", { argsIgnorePattern: "^_" }],
    // require using arrow functions as callbacks
    // https://eslint.org/docs/rules/prefer-arrow-callback
    "prefer-arrow-callback": "error",
    // require using template literals instead of string concatenation
    // http://eslint.org/docs/rules/prefer-template
    "prefer-template": "error",
    // require using of const declaration for variables that are never modified after declared
    // https://eslint.org/docs/rules/prefer-const
    "prefer-const": "error",
    // disallow modifying variables that are declared using const
    // https://eslint.org/docs/rules/no-const-assign
    "no-const-assign": "error",
    // require let or const instead of var
    // https://eslint.org/docs/rules/no-var
    "no-var": "error",
    // require at least one whitespace after comments( // and /*)
    // https://eslint.org/docs/rules/spaced-comment
    "spaced-comment": ["warn", "always"],
    // Disallow focused tests
    "no-only-tests/no-only-tests": "error",
    // Require `===` and `!==` comparisons
    eqeqeq: "error",
    // Enforce curly braces for if/else statements for better clarity.
    curly: "error",

    // We are ok with providing explict type annotations for additional
    // clarity.
    "@typescript-eslint/no-inferrable-types": "off",
    // We are ok with empty functions. Often we need a no-op function
    // as an argument.
    "@typescript-eslint/no-empty-function": "off",
    "@typescript-eslint/no-implicit-any-catch": "error",
  },
  settings: {
    "svelte3/typescript": true,
  },
};
