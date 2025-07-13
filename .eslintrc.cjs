module.exports = {
  rules: {
    'vue/multi-word-component-names': 'off',
    '@typescript-eslint/no-unused-vars': 'error',
    'vue/html-self-closing': [
      'error',
      {
        html: {
          void: 'always',
          normal: 'never',
          component: 'always'
        }
      }
    ],
    'vue/require-prop-types': 'error',
    'vue/prop-name-casing': ['error', 'camelCase'],
    'vue/script-setup-uses-vars': 'error',
    'vue/no-unused-components': 'error',
    'no-unused-vars': ['error', { vars: 'all', args: 'after-used', ignoreRestSiblings: true }],
  }
}