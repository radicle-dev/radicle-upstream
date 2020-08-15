const eslintSveltePreprocess = require('eslint-svelte3-preprocess');
const svelteConfig = require('./svelte.config')

module.exports = {
  env: {
    browser: true,
    es6: true,
    node: true
  },
  plugins: ["no-only-tests", "svelte3"],
  extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: 2017,
    sourceType: "module",
    ecmaFeatures: {
      modules: true,
      experimentalObjectRestSpread: true
    }
  },
  overrides: [
    {
      files: ["**/*.svelte"],
      processor: "svelte3/svelte3",
    },
    {
      files: ["**/*.ts"],
      extends: [
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:@typescript-eslint/recommended-requiring-type-checking"
      ],
      plugins: ["@typescript-eslint"],
      parser: "@typescript-eslint/parser",
      parserOptions: {
        project: "./tsconfig.json"
      }
    }
  ],
  rules: {
    "no-only-tests/no-only-tests": "error",
    // Disallow Unused Variables
    // https://eslint.org/docs/rules/no-unused-vars
    "no-unused-vars": ["error", { "argsIgnorePattern": "^_" }],
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
    "spaced-comment": ["warn", "always"]
  },
  settings: {
    "svelte3/preprocess": eslintSveltePreprocess(svelteConfig.preprocess)
  }
}
