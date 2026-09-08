import { defineConfig } from "oxfmt";

export default defineConfig({
  ignorePatterns: ["**/node_modules/**", "**/.bun/**", "**/dist/**"],
  trailingComma: "none",
  semi: false,
  singleQuote: true,
  sortImports: {
    newlinesBetween: true,
    // https://oxc.rs/docs/guide/usage/formatter/config-file-reference.html#sortimports-groups
    groups: [
      // External dependencies
      "value-builtin",
      "value-external",
      // Internal dependencies
      "value-internal",
      "value-parent",
      "value-sibling",
      "value-index",
      // Type imports
      "type-import",
    ],
  },
});
