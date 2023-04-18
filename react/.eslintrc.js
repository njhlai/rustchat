module.exports = {
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
    'plugin:@typescript-eslint/strict',
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    'plugin:@next/next/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    project: true,
    tsconfigRootDir: __dirname,
  },
  plugins: [
    '@typescript-eslint',
    'react',
    'react-hooks',
    '@next/eslint-plugin-next',
  ],
  settings: {
    react: {
      version: "detect"
    }
  },
  ignorePatterns: [
    ".eslintrc.js",
    "next.config.js"
  ],
  root: true,
  rules: {
    semi: "error",
    "react/react-in-jsx-scope": "off",
    "no-unused-vars": "off",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      {
        "argsIgnorePattern": "^_",
        "varsIgnorePattern": "^_",
        "caughtErrorsIgnorePattern": "^_"
      }
    ]
  }
};
