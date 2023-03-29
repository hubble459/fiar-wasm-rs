// i18n
import { createI18n } from 'vue-i18n';
import { createRouter, createWebHistory } from 'vue-router';
import generatedRoutes from 'virtual:generated-pages';
import { setupLayouts } from 'virtual:generated-layouts';
import messages from '@intlify/unplugin-vue-i18n/messages';
// pinia
import axios, { AxiosError } from 'axios';
import store from '@/store';
import App from './App.vue';

import 'virtual:windi.css';
// Devtools: https://windicss.org/integrations/vite.html#design-in-devtools
import 'virtual:windi-devtools';
import '@/assets/styles/index.scss';

const i18n = createI18n({
    locale: 'en',
    messages,
});

const app = createApp(App);

// Setup up pages with layouts
const routes = setupLayouts(generatedRoutes);
const router = createRouter({ history: createWebHistory(), routes });
app
    .use(router)
    .use(store)
    .use(i18n);

axios.interceptors.response.use((a) => a, (err) => {
    if (err instanceof AxiosError) {
        const exceptionCatcher = routes.find((route) => route.meta?.exception === err.status);
        if (exceptionCatcher) {
            router.push(exceptionCatcher.path);
        }
    }
    return err;
});

app.config.errorHandler = (err) => {
    if (import.meta.env.DEV) {
        console.error(err);
    }
    return false;
};

app.mount('#app');
