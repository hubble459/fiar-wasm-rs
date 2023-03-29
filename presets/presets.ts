import Vue from '@vitejs/plugin-vue';
import Pages from 'vite-plugin-pages';
import Layouts from 'vite-plugin-vue-layouts';
import svgLoader from 'vite-svg-loader';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import Icons from 'unplugin-icons/vite';
import IconsResolver from 'unplugin-icons/resolver';
import { ElementPlusResolver, VueUseComponentsResolver } from 'unplugin-vue-components/resolvers';
import WindiCSS from 'vite-plugin-windicss';
import Markdown from 'vite-plugin-vue-markdown'
import Prism from 'markdown-it-prism';
import ViteFonts from 'unplugin-fonts/vite';
import VueI18n from '@intlify/unplugin-vue-i18n/vite';
import LinkAttributes from 'markdown-it-link-attributes';
import { ConfigEnv } from 'vite';
import { resolve } from 'path';
import { VitePWA } from 'vite-plugin-pwa';

const defaultClasses = 'prose prose-sm m-auto text-left';

export default (env: ConfigEnv) => {
    return [
        Vue({
            include: [/\.vue$/, /\.md$/],
        }),
        svgLoader(),
        // https://github.com/hannoeru/vite-plugin-pages
        Pages({
            extensions: ['vue'],
        }),

        // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
        Layouts(),
        AutoImport({
            dts: './src/auto-imports.d.ts',
            imports: ['vue', 'pinia', 'vue-router', 'vue-i18n', '@vueuse/core'],
            // Generate corresponding .eslintrc-auto-import.json file.
            // eslint globals Docs - https://eslint.org/docs/user-guide/configuring/language-options#specifying-globals
            eslintrc: {
                enabled: false, // Default `false`
                filepath: './.eslintrc-auto-import.json', // Default `./.eslintrc-auto-import.json`
                globalsPropValue: true, // Default `true`, (true | false | 'readonly' | 'readable' | 'writable' | 'writeable')
            },
            resolvers: [ElementPlusResolver()],
        }),
        Components({
            dts: './src/components.d.ts',
            extensions: ['vue', 'md'],
            include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
            // imports 指定组件所在位置，默认为 src/components; 有需要也可以加上 view 目录
            dirs: ['src/components/'],
            resolvers: [ElementPlusResolver(), IconsResolver(), VueUseComponentsResolver()],
        }),
        Icons({
            compiler: 'vue3',
            autoInstall: true,
        }),
        ViteFonts({
            google: {
                families: ['Open Sans', 'Montserrat', 'Fira Sans'],
            },
        }),
        VueI18n({
            include: [resolve(__dirname, '../locales/**')],
        }),

        // https://github.com/antfu/vite-plugin-pwa
        VitePWA({
            registerType: 'autoUpdate',
            includeAssets: ['favicon.svg', 'robots.txt', 'safari-pinned-tab.svg'],
            manifest: {
                name: 'Cirqll',
                short_name: 'Cirqll',
                theme_color: '#ffffff',
                icons: [
                    {
                        src: '/pwa-192x192.png',
                        sizes: '192x192',
                        type: 'image/png',
                    },
                    {
                        src: '/pwa-512x512.png',
                        sizes: '512x512',
                        type: 'image/png',
                    },
                    {
                        src: '/pwa-512x512.png',
                        sizes: '512x512',
                        type: 'image/png',
                        purpose: 'any maskable',
                    },
                ],
            },
        }),

        WindiCSS({
            safelist: defaultClasses,
        }),

        Markdown({
            wrapperClasses: defaultClasses,
            headEnabled: false,
            markdownItSetup(md) {
                // https://prismjs.com/
                md.use(Prism);
                // 为 md 中的所有链接设置为 新页面跳转
                md.use(LinkAttributes, {
                    matcher: (link: string) => /^https?:\/\//.test(link),
                    attrs: {
                        target: '_blank',
                        rel: 'noopener',
                    },
                });
            },
        }),
    ];
};
