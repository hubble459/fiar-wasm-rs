module.exports = {
    env: {
        browser: true,
        es2021: true,
        node: true,
    },
    globals: {
        defineEmits: true,
        document: true,
        localStorage: true,
        GLOBAL_VAR: true,
        window: true,
        defineProps: true,
        defineExpose: true,
        withDefaults: true,
    },
    extends: [
        './.eslintrc-auto-import.json',
        'airbnb-base',
        'plugin:@typescript-eslint/recommended',
        'plugin:vue/vue3-recommended',
    ],
    parserOptions: {
        ecmaVersion: 'latest',
        parser: '@typescript-eslint/parser',
        sourceType: 'module',
    },
    plugins: ['vue', '@typescript-eslint', 'import'],
    rules: {
        'no-console': 'off',
        'import/no-unresolved': 'off',
        'import/extensions': 'off',
        'import/no-extraneous-dependencies': 'off',
        'vue/multi-word-component-names': 'off',
        indent: ['error', 4],
        '@typescript-eslint/indent': ['error', 4],
        'vue/script-indent': ['error', 4, {
            baseIndent: 1,
        }],
        'vue/html-indent': ['error', 4, {
            attribute: 1,
            closeBracket: 0,
            alignAttributesVertically: true,
            ignores: [],
        }],
        '@typescript-eslint/no-explicit-any': 'off',
    },
    overrides: [
        {
            files: [
                '**/*.vue',
            ],
            rules: {
                '@typescript-eslint/indent': 'off',
                indent: 'off',
            },
        },
    ],
};
