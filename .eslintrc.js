const eslintSveltePreprocess = require("eslint-svelte3-preprocess");
const svelteConfig = require("./svelte.config");

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
  extends: ["eslint:recommended"],
  plugins: ["svelte3", "@typescript-eslint"],
  overrides: [
    {
      files: ["*.svelte"],
      processor: "svelte3/svelte3",
    },
    {
      files: ["*.ts", "*.json"],
      extends: ["plugin:@typescript-eslint/recommended"],
      rules: {
        // TODO(sos): typescript rule changes are ignored unless explicitly associated with the override.
        //
        // we need to turn this one off because svelte-check needs explicit type annotations for boolean
        // props with default values.
        // https://github.com/typescript-eslint/typescript-eslint/blob/master/packages/eslint-plugin/docs/rules/no-inferrable-types.md
        "@typescript-eslint/no-inferrable-types": "off",
      },
    },
  ],
  rules: {
    // Disallow Unused Variables
    // https://eslint.org/docs/rules/no-unused-vars
    "no-unused-vars": ["error", { argsIgnorePattern: "^_" }],
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
  },
  settings: {
    "svelte3/preprocess": eslintSveltePreprocess(svelteConfig.preprocess),
  },
};
