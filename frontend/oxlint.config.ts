import { defineConfig } from 'oxlint'

export default defineConfig({
  plugins: ['import'],
  options: {
    typeAware: true
  },
  rules: {
    'typescript/consistent-type-imports': [
      'error',
      {
        prefer: 'type-imports',
        fixStyle: 'separate-type-imports' // Dictates the fix behavior
      }
    ],
    // Add this rule to explicitly ban inline 'type' syntax
    'import/consistent-type-specifier-style': ['error', 'prefer-top-level']
  }
})
