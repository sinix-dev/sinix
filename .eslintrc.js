module.exports = {
  root: true,
  env: {
    browser: true,
    node: true
  },
  parserOptions: {
    parser: 'babel-eslint'
  },
  extends: [
    '@nuxtjs',
    'plugin:nuxt/recommended'
  ],
  // add your custom rules here
  rules: {
    "nuxt/no-cjs-in-config": "off",
    "semi": ["error", "never"],
    "array-bracket-spacing": [
      "error",
      "never"
    ],
    "no-trailing-spaces": "error",
    "comma-spacing": [
      "error",
      {
        "before": false,
        "after": true
      }
    ],
    "indent": [
      "error",
      2
    ],
    "key-spacing": [
      "error",
      {
        "beforeColon": false,
        "afterColon": true
      }
    ],
    "keyword-spacing": "off",
    "quotes": ["error", "double"],
    "camelcase": "off",
    "no-console": "off",
    "space-before-function-paren": ["error", "never"],
    "space-before-blocks": ["error", "never"],
    "prefer-promise-reject-errors": "off"
  }
}
