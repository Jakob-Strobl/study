import js from "@eslint/js";
import globals from "globals";
import prettier from "eslint-config-prettier";

// It didn't work at first, but it does work on helix!!!
/** @type {import('eslint').Linter.Config[]} */
export default [
  js.configs.recommended,
  {
    files: ["**/*.js", "**/*.jsx"],
    languageOptions: {
      globals: {
        // Union of browser and node globals
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        ecmaFeatures: {
          jsx: true,
        },
      },
    },
  },
  prettier,
];
