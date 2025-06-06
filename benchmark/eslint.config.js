export default [
  {
    files: ["**/*.js"],
    languageOptions: {
      parserOptions: {
        ecmaFeatures: { modules: true },
        ecmaVersion: "latest",
      },
    },
    rules: {
      // Biome complexity
      "no-extra-boolean-cast": "error",
      "no-regex-spaces": "error",
      "no-useless-catch": "error",
      "no-useless-constructor": "error",
      "no-extra-label": "error",
      "no-useless-rename": "error",
      "no-void": "error",
      "no-with": "error",
      "prefer-arrow-callback": "error",
      "dot-notation": "error",
      "prefer-regex-literals": "error",

      // Biome correctness
      "no-const-assign": "error",
      "no-constant-condition": "error",
      "no-constructor-return": "error",
      "no-empty-character-class": "error",
      "no-empty-pattern": "error",
      "no-obj-calls": "error",
      "no-inner-declarations": "error",
      "constructor-super": "error",
      "no-new-native-nonconstructor": "error",
      "no-new-symbol": "error",
      "no-use-before-define": "error",
      "no-loss-of-precision": "error",
      "no-self-assign": "error",
      "no-setter-return": "error",
      "no-case-declarations": "error",
      "no-undef": "error",
      "no-unreachable": "error",
      "no-this-before-super": "error",
      "no-unsafe-finally": "error",
      "no-unsafe-optional-chaining": "error",
      "no-unused-labels": "error",
      "no-unused-private-class-members": "error",
      "no-unused-vars": "error",
      "no-array-constructor": "error",
      "use-isnan": "error",
      "for-direction": "error",
      "require-yield": "error",

      // Biome security
      "no-eval": "error",

      // Biome style
      "prefer-rest-params": "error",
      "no-sequences": "error",
      "no-param-reassign": "error",
      "no-var": "error",
      "prefer-const": "error",
      "no-lonely-if": "error",
      "default-param-last": "error",
      "prefer-exponentiation-operator": "error",
      "prefer-numeric-literals": "error",
      "one-var": "error",
      "prefer-template": "error",

      // Biome suspicious
      "no-async-promise-executor": "error",
      "no-cond-assign": "error",
      "no-ex-assign": "error",
      "no-class-assign": "error",
      "no-compare-neg-zero": "error",
      "no-labels": "error",
      "no-control-regex": "error",
      "no-debugger": "error",
      eqeqeq: "error",
      "no-duplicate-case": "error",
      "no-dupe-class-members": "error",
      "no-dupe-keys": "error",
      "no-dupe-args": "error",
      "no-empty": "error",
      "no-empty-static-block": "error",
      "no-empty-function": "error",
      "no-fallthrough": "error",
      "no-func-assign": "error",
      "no-global-assign": "error",
      "no-import-assign": "error",
      "no-label-var": "error",
      "no-misleading-character-class": "error",
      "no-prototype-builtins": "error",
      "no-redeclare": "error",
      "no-self-compare": "error",
      "no-shadow-restricted-names": "error",
      "no-sparse-arrays": "error",
      "no-unsafe-negation": "error",
      "require-await": "error",
      "default-case-last": "error",
      "getter-return": "error",
      "valid-typeof": "error",
    },
  },
];
