import eslint from "@eslint/js";
import globals from "globals";

export default [
  eslint.configs.recommended,
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.es2021 },
    },
    ignores: [".nuxt/**", ".output/**", "node_modules/**", "**/*.vue"],
  },
];